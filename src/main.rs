#![feature(async_closure)]
#![feature(trivial_bounds)]

use azalea::prelude::*;
use azalea::swarm::prelude::*;

use crate::bot::{handle, swarm_handle};

mod action;
mod bot;
mod model;
mod web;

pub const ATTACK_DELAY: u16 = 1000;

#[tokio::main]
async fn main() {
    let farmer = Account::microsoft("begrapa@outlook.com").await.unwrap();
    let afk = Account::offline("BeGrapa2");

    SwarmBuilder::new()
        .set_handler(handle)
        .set_swarm_handler(swarm_handle)
        .add_account(farmer)
        .add_account(afk)
        .start("localhost:25566")
        .await
        .unwrap();
}
