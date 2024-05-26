use crate::game_state::Location;
use crate::mobs::goblin::{attack::GoblinAttackPlugin, spawn_goblin::SpawnGoblinPlugin};
use crate::mobs::{general_mob_behaviour::GeneralMobBehaviourPlugin, spawn_mobs::SpawnMobsPlugin};
use crate::player::{input_handling::InputHandlingPlugin, movement::MovementPlugin, melee_attack::MeleeAttackPlugin};
use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_xpbd_2d::prelude::*;

mod game_state;
mod mobs;
mod player;
mod setup;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            PhysicsPlugins::default(),
            InputHandlingPlugin,
            MovementPlugin,
            setup::SetupPlugin,
            SpawnGoblinPlugin,
            SpawnMobsPlugin,
            GoblinAttackPlugin,
            GeneralMobBehaviourPlugin,
            MeleeAttackPlugin,
        ))
        .insert_state(Location::Floor1)
        .run();
}
