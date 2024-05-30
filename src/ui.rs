use crate::states::GameState;
use crate::ui::inventory::{disable_inventory, enable_inventory, show_inventory, InventoryState};
use bevy::prelude::*;
pub mod inventory;

pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Inventory), enable_inventory)
            .add_systems(OnExit(GameState::Inventory), disable_inventory)
            .add_systems(Update, show_inventory)
            .insert_resource(InventoryState::default());
    }
}
