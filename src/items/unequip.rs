use crate::items::drop::{Item, WeaponType, ItemType, ArmorType};
use crate::player::spawn_player::Player;
use crate::player::spawn_player::PlayerAssets;
use crate::ui::inventory::EquippedItem;
use ::bevy::prelude::*;

#[derive(Event)]
pub struct UnequipItemEvent;

pub fn unequip_item(
    mut player_query: Query<&mut Handle<Image>, With<Player>>,
    player_assets: ResMut<PlayerAssets>,
    equipped_items_query: Query<&Item, With<EquippedItem>>,
    mut unequip_item_event: EventReader<UnequipItemEvent>,
) {
    for _event in unequip_item_event.read() {
        let mut player_image = player_query.single_mut();
        let item_types: Vec<ItemType> = equipped_items_query.into_iter().map(|item| item.item_type.clone()).collect();
        if item_types.contains(&&ItemType::Weapon(WeaponType::Sword)) { 
            *player_image = player_assets.naked_with_sword.clone();
        } else {
            *player_image = player_assets.naked.clone();
        }
    }
}
