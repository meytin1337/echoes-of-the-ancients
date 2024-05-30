use crate::items::drop::Item;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::items::pick_up::InventoryItem;

#[derive(Resource)]
pub struct InventoryState {
    pub is_window_open: bool,
}

#[derive(Component)]
pub struct EquippedItem;

impl Default for InventoryState {
    fn default() -> Self {
        Self {
            is_window_open: false,
        }
    }
}

pub fn show_inventory(
    mut inventory_state: ResMut<InventoryState>,
    mut egui_contexts: EguiContexts,
    item_query: Query<(Entity, &Item), With<InventoryItem>>,
    mut commands: Commands,
) {
    if inventory_state.is_window_open {
        egui::Window::new("Inventory")
            .open(&mut inventory_state.is_window_open)
            .show(egui_contexts.ctx_mut(), |ui| {
                for (entity, item) in item_query.iter() {
                    ui.label(item.name.clone());
                    if ui.button("Drop").clicked() {
                        commands.entity(entity).remove::<InventoryItem>();
                    }
                    if ui.button("Equip").clicked() {
                        commands.entity(entity).insert(EquippedItem);
                    }
                }
                ui.label("This is the inventory window");
            });
    }
}

pub fn enable_inventory(mut inventory_state: ResMut<InventoryState>) {
    inventory_state.is_window_open = true;
}

pub fn disable_inventory(mut inventory_state: ResMut<InventoryState>) {
    inventory_state.is_window_open = false;
}
