use crate::states::GameState;
use crate::ui::{character_menu::{show_character_menu, CharacterMenu, enable_character_menu, disable_character_menu}, inventory::{disable_inventory, enable_inventory, show_inventory, Inventory}};
use bevy::prelude::*;
pub mod inventory;
pub mod character_menu;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Inventory), enable_inventory)
            .add_systems(OnExit(GameState::Inventory), disable_inventory)
            .add_systems(OnEnter(GameState::CharacterMenu), enable_character_menu)
            .add_systems(OnExit(GameState::CharacterMenu), disable_character_menu)
            .add_systems(Update, (show_inventory, show_character_menu))
            .insert_resource(Inventory::default())
            .insert_resource(CharacterMenu{is_window_open: false});
    }
}
