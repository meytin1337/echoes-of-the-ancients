use crate::items::{
    drop::drop_item,
    equip::{equip_item, EquipItemEvent},
    unequip::{unequip_item, UnequipItemEvent},
    pick_up::pick_up_item,
};
use bevy::prelude::*;
pub mod drop;
pub mod equip;
pub mod pick_up;
pub mod unequip;

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (drop_item, pick_up_item).in_set(crate::sets::PlayingSet),
        )
        .add_systems(Update, (equip_item, unequip_item))
        .add_event::<EquipItemEvent>()
        .add_event::<UnequipItemEvent>();
    }
}
