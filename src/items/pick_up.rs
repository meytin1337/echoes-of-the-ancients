use bevy::prelude::*;
use crate::ui::inventory::Inventory;
use crate::items::drop::ItemType;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_xpbd_2d::prelude::*;
use crate::items::drop::Item;
use crate::input_handling::ItemPickUpEvent;

#[derive(Component)]
pub struct InventoryItem;

pub fn pick_up_item (
    mut pick_up_event_reader: EventReader<ItemPickUpEvent>,
    mut item_query: Query<(Entity, &Item)>,
    mut commands: Commands,
    mut inventory_res: ResMut<Inventory>,

) {
    for event in pick_up_event_reader.read() {
        for (entity, item) in item_query.iter_mut() {
            if entity == event.item {
                match item.item_type {
                    ItemType::Gold(gold) => {
                        inventory_res.gold += gold;
                        return commands.entity(entity).despawn();
                    }
                    _ => {}
                }
                commands.entity(entity).remove::<Collider>();
                commands.entity(entity).remove::<MaterialMesh2dBundle<ColorMaterial>>();
                commands.entity(entity).insert(InventoryItem);
            }
        }
    }
}
