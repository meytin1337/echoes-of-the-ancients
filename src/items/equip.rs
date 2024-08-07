use crate::items::drop::{Item, WeaponType, ItemType, ArmorType};
use crate::player::spawn_player::Player;
use crate::player::spawn_player::PlayerAssets;
use crate::ui::inventory::EquippedItem;
use ::bevy::prelude::*;

#[derive(Event)]
pub struct EquipItemEvent {
    pub item_type: ItemType
}

pub fn equip_item(
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    player_assets: ResMut<PlayerAssets>,
    equipped_items_query: Query<&Item, With<EquippedItem>>,
    mut equip_item_event: EventReader<EquipItemEvent>,
) {
    for event in equip_item_event.read() {
        let mut player_image = player_query.single_mut();
        // handle other cases where player has other items equipped
        //let item_types: Vec<ItemType> = equipped_items_query.iter().map(|item| item.item_type.clone()).collect();
        // why is this sometimes empty??
        //if item_types.contains(&&ItemType::Weapon(WeaponType::Sword)) { 
        //    *player_image = player_assets.naked_with_sword.clone();
        //}
        match event.item_type {
            ItemType::Weapon(WeaponType::Sword) =>  *player_image = player_assets.naked_with_sword.clone(),
            _ => ()
        }
    }
}
