use crate::items::drop::{Item, ItemType};
use crate::items::equip::EquipItemEvent;
use crate::items::pick_up::InventoryItem;
use crate::items::unequip::UnequipItemEvent;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct Inventory {
    pub is_window_open: bool,
    pub gold: u32,
}

#[derive(Component)]
pub struct EquippedItem;

impl Default for Inventory {
    fn default() -> Self {
        Self {
            is_window_open: false,
            gold: 0,
        }
    }
}

pub fn show_inventory(
    inventory_state: ResMut<Inventory>,
    mut egui_contexts: EguiContexts,
    item_query: Query<(Entity, &Item), (With<InventoryItem>, Without<EquippedItem>)>,
    equipped_item_query: Query<(Entity, &Item), With<EquippedItem>>,
    mut equip_item_event: EventWriter<EquipItemEvent>,
    mut unequip_item_event: EventWriter<UnequipItemEvent>,
    mut commands: Commands,
) {
    if inventory_state.is_window_open {
        let mut health_potion_counter = 0;
        let mut mana_potion_counter = 0;
        egui::SidePanel::left("Inventory").show(egui_contexts.ctx_mut(), |ui| {
            ui.label("Equipped Items:");
            for (entity, item) in equipped_item_query.iter() {
                ui.horizontal(|ui| {
                    ui.label(&item.name).on_hover_ui(|ui| {
                        if let Some(item_stats) = &item.item_stats {
                            for stat in item_stats {
                                ui.label(format!(
                                    "{}: {}",
                                    stat.attribute_type.to_string(),
                                    stat.value,
                                ));
                            }
                        }
                    });
                    if ui.button("Unequip").clicked() {
                        commands.entity(entity).remove::<EquippedItem>();
                        unequip_item_event.send(UnequipItemEvent {
                            item_type: item.item_type
                        });
                    }
                });
            }
            ui.label("Inventory:");
            for (entity, item) in item_query.iter() {
                match item.item_type {
                    ItemType::HealthPotion => {
                        health_potion_counter += 1;
                    }
                    ItemType::ManaPotion => {
                        mana_potion_counter += 1;
                    }
                    ItemType::Gold(_) => (),
                    _ => {
                        ui.horizontal(|ui| {
                            ui.label(&item.name).on_hover_ui(|ui| {
                                if let Some(item_stats) = &item.item_stats {
                                    for stat in item_stats {
                                        ui.label(format!(
                                            "{}: {}",
                                            stat.attribute_type.to_string(),
                                            stat.value
                                        ));
                                    }
                                }
                            });
                            if ui.button("Drop").clicked() {
                                commands.entity(entity).remove::<InventoryItem>();
                            }
                            if ui.button("Equip").clicked() {
                                commands.entity(entity).insert(EquippedItem);
                                equip_item_event.send(EquipItemEvent {
                                    item_type: item.item_type
                                });
                                for (equipped_entity, equipped_item) in equipped_item_query.iter() {
                                    // if another item of the same type is equipped, remove it
                                    if entity != equipped_entity
                                        && item.item_type == equipped_item.item_type
                                    {
                                        commands.entity(equipped_entity).remove::<EquippedItem>();
                                    }
                                }
                            }
                        });
                    }
                }
            }
            ui.label(format!("Health Potions: {}", health_potion_counter));
            ui.label(format!("Mana Potions: {}", mana_potion_counter));
            ui.label(format!("Gold: {}", inventory_state.gold));
        });
    }
}

pub fn enable_inventory(mut inventory_state: ResMut<Inventory>) {
    inventory_state.is_window_open = true;
}

pub fn disable_inventory(mut inventory_state: ResMut<Inventory>) {
    inventory_state.is_window_open = false;
}
