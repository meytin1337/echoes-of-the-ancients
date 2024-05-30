use bevy::prelude::*;
use crate::player::input_handling::ItemPickUpEvent;
use crate::items::drop::Item;

pub struct PickUpPlugin;


impl Plugin for PickUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, pick_up_item);
    }
}

fn pick_up_item (
    pick_up_event_reader: EventReader<ItemPickUpEvent>,
    item_query: Query<(Entity, &Transform), With<Item>>,
    mut commands: Commands,
) {}
