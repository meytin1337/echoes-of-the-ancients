use crate::states::{GameState, Location};
use crate::ui::inventory::InventoryPlugin;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_xpbd_2d::prelude::*;

mod input_handling;
mod items;
mod mobs;
mod player;
mod states;
mod ui;

fn main() {
    App::new()
        .insert_state(Location::Floor1)
        .insert_state(GameState::Playing)
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            PhysicsPlugins::default(),
            input_handling::InputHandlingPlugin,
            EguiPlugin,
            InventoryPlugin,
            player::PlayerPlugin,
            items::ItemsPlugin,
            mobs::MobsPlugin,
        ))
        .run();
}
