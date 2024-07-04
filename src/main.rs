use crate::states::{GameState, Location};
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
mod sets;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            EmbeddedAssetPlugin::default(),
            PhysicsPlugins::default(),
            bevy_framepace::FramepacePlugin,
            input_handling::InputHandlingPlugin,
            EguiPlugin,
            ui::UiPlugin,
            player::PlayerPlugin,
            items::ItemsPlugin,
            mobs::MobsPlugin,
            sets::SetsPlugin,
            states::StatesPlugin,
        ))
        .run();
}
