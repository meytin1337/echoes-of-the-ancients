use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

mod setup;
mod character;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), PhysicsPlugins::default(), character::input_handling::InputHandlingPlugin, character::movement::MovementPlugin, setup::SetupPlugin))
        .run();
}
