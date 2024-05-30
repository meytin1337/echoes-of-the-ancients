use crate::player::{
    attack::{attack, killed_mob_cleanup, ItemDropEvent, MobKillEvent},
    movement::{apply_movement_damping, center_camera, run, CameraMoveEvent},
    spawn_player::spawn_player,
};
use bevy::prelude::*;
pub mod attack;
pub mod movement;
pub mod spawn_player;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, (attack, killed_mob_cleanup))
            .add_event::<MobKillEvent>()
            .add_event::<ItemDropEvent>();
        app.add_systems(Update, (run, apply_movement_damping, center_camera))
            .add_event::<CameraMoveEvent>();
    }
}
