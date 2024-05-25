use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use crate::mobs::goblin::spawn_goblin::SpawnGoblinPlugin;
use crate::game_state::Location;
use crate::mobs::spawn_mobs::SpawnMobsPlugin;

mod setup;
mod character;
mod mobs;
mod game_state;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), PhysicsPlugins::default(), character::input_handling::InputHandlingPlugin, character::movement::MovementPlugin, setup::SetupPlugin, SpawnGoblinPlugin, SpawnMobsPlugin))
        .insert_state(Location::Floor1)
        .run();
}
