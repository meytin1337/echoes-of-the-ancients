use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EmbeddedAssetPlugin::default(), PhysicsPlugins::default()))
        .run();
}
