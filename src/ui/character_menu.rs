use crate::player::PlayerStats;
use crate::items::drop::Attributes;
use crate::ui::inventory::EquippedItem;
use crate::items::drop::Item;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

#[derive(Resource)]
pub struct CharacterMenu {
    pub is_window_open: bool,
}

pub fn show_character_menu(
    mut egui_contexts: EguiContexts,
    equipped_item_query: Query<&Item, With<EquippedItem>>,
    player_stats_res: Res<PlayerStats>,
    character_menu_state: Res<CharacterMenu>,
) {
    if character_menu_state.is_window_open {
        let mut total_damage = player_stats_res.attack_damage;
        let mut total_armor = player_stats_res.armor;
        let mut total_health = player_stats_res.health;
        let mut total_mana = player_stats_res.mana;
        egui::SidePanel::left("CharacterMenu").show(egui_contexts.ctx_mut(), |ui| {
            ui.label("Stats:");
            for item in equipped_item_query.iter() {
                match &item.item_stats {
                    Some(item_stats) => {
                        for stat in item_stats {
                            match stat.attribute_type {
                                Attributes::PhysicalDamage => {
                                    total_damage += stat.value;
                                }
                                Attributes::Armor => {
                                    total_armor += stat.value;
                                }
                                Attributes::Health => {
                                    total_health += stat.value;
                                }
                                Attributes::Mana => {
                                    total_mana += stat.value;
                                }
                                Attributes::FireDamage => {
                                    total_damage += stat.value;
                                }
                                Attributes::ColdDamage => {
                                    total_damage += stat.value;
                                }
                                Attributes::PoisonDamage => {
                                    total_damage += stat.value;
                                }
                                _ => {}
                            }
                        }
                    }
                    None => {}
                }
            }
            ui.label(format!("Damage: {}", total_damage));
            ui.label(format!("Armor: {}", total_armor));
            ui.label(format!("Health: {}", total_health));
            ui.label(format!("Mana: {}", total_mana));
        });
    }
}

pub fn enable_character_menu(mut character_menu_state: ResMut<CharacterMenu>) {
    character_menu_state.is_window_open = true;
}

pub fn disable_character_menu(mut character_menu_state: ResMut<CharacterMenu>) {
    character_menu_state.is_window_open = false;
}
