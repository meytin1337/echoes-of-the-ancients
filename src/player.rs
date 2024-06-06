use crate::player::{
    attack::{attack, killed_mob_cleanup, ItemDropEvent, MobKillEvent},
    movement::{apply_movement_damping, center_camera, run, CameraMoveEvent},
    spawn_player::spawn_player,
};
use bevy::{prelude::*, time::Stopwatch};
pub mod attack;
pub mod movement;
pub mod spawn_player;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerSet;

#[derive(Resource)]
pub struct PlayerStats {
    pub radius: f32,
    pub health: u32,
    pub mana: u32,
    pub attack_damage: u32,
    pub armor: u32,
    pub attack_range: f32,
    pub attack_speed: f32,
    pub attack_timer: Stopwatch,
    pub move_speed: f32,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    attack,
                    killed_mob_cleanup,
                    run,
                    apply_movement_damping,
                    center_camera,
                )
                    .in_set(crate::sets::PlayingSet),
            )
            .insert_resource(PlayerStats {
                mana: 100,
                health: 100000,
                attack_damage: 30,
                armor: 0,
                attack_range: 50.0,
                attack_speed: 1.0,
                attack_timer: Stopwatch::new(),
                radius: 10.0,
                move_speed: 200.0,
            })
            .add_event::<CameraMoveEvent>()
            .add_event::<MobKillEvent>()
            .add_event::<ItemDropEvent>();
    }
}
