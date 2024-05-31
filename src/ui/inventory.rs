use crate::items::drop::Item;
use crate::items::pick_up::InventoryItem;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

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
    item_query: Query<(Entity, &Item), (With<InventoryItem>, Without<EquippedItem>)>,
    equipped_item_query: Query<(Entity, &Item), With<EquippedItem>>,
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
                        for (equipped_entity, equipped_item) in equipped_item_query.iter() {
                            // if another item of the same type is equipped, remove it
                            println!("equipped item: {}", item.name);
                            if entity != equipped_entity && item.item_type == equipped_item.item_type {
                                println!("unequipped item: {}", equipped_item.name);
                                commands.entity(equipped_entity).remove::<EquippedItem>();
                            }
                        }
                    }
                }
                for (entity, item) in equipped_item_query.iter() {
                    ui.label(String::from("equipped item:") + &item.name);
                    if ui.button("Unequip").clicked() {
                        commands.entity(entity).remove::<EquippedItem>();
                    }
                }
            });
    }
}

pub fn enable_inventory(mut inventory_state: ResMut<InventoryState>) {
    inventory_state.is_window_open = true;
}

pub fn disable_inventory(mut inventory_state: ResMut<InventoryState>) {
    inventory_state.is_window_open = false;
}
