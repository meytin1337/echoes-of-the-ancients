use crate::items::{drop::drop_item, pick_up::pick_up_item};
use bevy::prelude::*;
pub mod drop;
pub mod pick_up;

pub struct ItemsPlugin;
impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drop_item);
        app.add_systems(Update, pick_up_item);
    }
}
