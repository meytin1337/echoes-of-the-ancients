use bevy::prelude::*;
use crate::input_handling::ItemPickUpEvent;
use crate::items::drop::Item;

pub fn pick_up_item (
    pick_up_event_reader: EventReader<ItemPickUpEvent>,
    item_query: Query<(Entity, &Transform), With<Item>>,
    mut commands: Commands,
) {}
