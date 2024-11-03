use anyhow::{anyhow, Result};
use azalea::ecs::prelude::{With, Without};
use azalea::entity::metadata::Allay;
use azalea::entity::{Dead, LocalEntity, Position};
use azalea::prelude::*;
use azalea::world::{InstanceName, MinecraftEntityId};

use crate::bot::{State, Task};

pub async fn attack_nearest_armor_stand(bot: &mut Client, state: &State) -> Result<()> {
    if !can_attack(bot, state).await {
        return Ok(());
    }

    {
        let mut action = state.task.lock().await;
        *action = Task::Attack;
    }

    let armor_stand = find_nearest_armor_stand(bot)
        .await?
        .ok_or_else(|| anyhow!("No armor stand found"))?;

    bot.attack(armor_stand);

    {
        let mut action = state.task.lock().await;
        *action = Task::None;
    }

    Ok(())
}

async fn can_attack(bot: &Client, state: &State) -> bool {
    let action = state.task.lock().await;

    *action == Task::None && !bot.has_attack_cooldown()
}

async fn find_nearest_armor_stand(bot: &Client) -> Result<Option<MinecraftEntityId>> {
    let mut nearest_entity = None;
    let bot_pos = bot.eye_position();
    let bot_instance_name = bot.component::<InstanceName>();

    {
        let mut ecs = bot.ecs.lock();
        let mut query = ecs
            .query_filtered::<(&MinecraftEntityId, &Position, &InstanceName), (
                With<Allay>,
                Without<LocalEntity>,
                Without<Dead>,
            )>();

        for (&id, pos, instance_name) in query.iter(&ecs) {
            if instance_name != &bot_instance_name {
                continue;
            }

            let distance = bot_pos.distance_to(pos);
            if distance > 4. || distance > f64::INFINITY {
                continue;
            }

            nearest_entity = Some(id);
        }
    }

    Ok(nearest_entity)
}
