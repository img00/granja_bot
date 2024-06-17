use std::sync::Arc;
use std::time::Duration;

use azalea::prelude::*;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::swarm::prelude::*;
use tokio::spawn;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::action::attack_nearest_armor_stand;
use crate::model::Experience;
use crate::web;
use crate::web::create_web_server;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Task {
    None,
    Attack,
}

#[derive(Clone, Component)]
pub struct State {
    pub connected: Arc<Mutex<bool>>,
    pub task: Arc<Mutex<Task>>,
    pub experience: Arc<Mutex<Experience>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            connected: Arc::new(Mutex::new(false)),
            task: Arc::new(Mutex::new(Task::None)),
            experience: Arc::new(Mutex::new(Experience::default())),
        }
    }
}

pub async fn handle(mut bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    if bot.profile.name.eq("BeGrapa2") {
        return Ok(());
    }

    match event {
        Event::Disconnect(_) => {
            let mut connected = state.connected.lock().await;
            *connected = false;
        }
        Event::Login => {
            create_web_server(state).await;
        }
        Event::Init => {
            {
                let mut connected = state.connected.lock().await;
                *connected = true;
            }

            web::update_state(state.clone()).await;

            spawn(async move {
                loop {
                    sleep(Duration::from_millis(500)).await;

                    {
                        let connected = state.connected.lock().await;
                        if !*connected {
                            return;
                        }
                    }

                    attack_nearest_armor_stand(&mut bot, &state)
                        .await
                        .unwrap_or_else(|err| {
                            eprintln!("Problem attacking armor stand: {err}");
                            bot.disconnect();
                        });
                }
            });
        }
        Event::Packet(packet) => match packet.as_ref() {
            ClientboundGamePacket::SetExperience(packet) => {
                let mut experience = state.experience.lock().await;
                *experience = Experience {
                    level: packet.experience_level,
                    progress: packet.experience_progress,
                    total: packet.total_experience,
                };
            }
            _ => {}
        },
        _ => {}
    }

    Ok(())
}

#[derive(Default, Clone, Resource)]
pub struct SwarmState {}

pub async fn swarm_handle(
    mut swarm: Swarm,
    event: SwarmEvent,
    _state: SwarmState,
) -> anyhow::Result<()> {
    match event {
        SwarmEvent::Disconnect(account, join_opts) => {
            sleep(Duration::from_secs(5)).await;

            swarm
                .add_and_retry_forever_with_opts(&account, State::default(), &join_opts)
                .await;
        }
        _ => {}
    }
    Ok(())
}
