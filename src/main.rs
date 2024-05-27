use crate::game_state::Location;
use crate::items::drop::DropPlugin;
use crate::mobs::goblin::{attack::GoblinAttackPlugin, spawn_goblin::SpawnGoblinPlugin};
use crate::mobs::{general_mob_behaviour::GeneralMobBehaviourPlugin, spawn_mobs::SpawnMobsPlugin};
use crate::player::{
    attack::PlayerAttackPlugin, input_handling::InputHandlingPlugin, movement::PlayerMovementPlugin,
};
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_xpbd_2d::prelude::*;

mod game_state;
mod mobs;
mod player;
mod setup;
mod items;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            PhysicsPlugins::default(),
            InputHandlingPlugin,
            PlayerMovementPlugin,
            setup::SetupPlugin,
            SpawnGoblinPlugin,
            SpawnMobsPlugin,
            GoblinAttackPlugin,
            GeneralMobBehaviourPlugin,
            PlayerAttackPlugin,
            mobs::combat::attack::MobAttackPlugin,
            DropPlugin,
        ))
        .insert_state(Location::Floor1)
        .run();
}
