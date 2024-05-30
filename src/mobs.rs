use crate::mobs::{
    general_mob_behaviour::{
        apply_mob_movement_damping, target_player, run_to_player, MobMoveEvent,
    },
    goblin::{
        goblin_attack::{goblin_attack, GoblinAttackEvent},
        spawn_goblin::spawn_goblin,
    },
    spawn_mobs::{floor_1_spawner, SpawnGoblinEvent},
    combat::mob_attack::{attack_player, tick_mob_attack_timer, MobAttackEvent},
};
use bevy::prelude::*;
pub mod combat;
pub mod general_mob_behaviour;
pub mod goblin;
pub mod spawn_mobs;

use crate::states::Location;

pub struct MobsPlugin;
impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Location::Floor1), floor_1_spawner)
            .add_event::<SpawnGoblinEvent>();
        app.add_systems(
            Update,
            (target_player, run_to_player, apply_mob_movement_damping),
        )
        .add_event::<MobMoveEvent>();
        app.add_systems(Update, (attack_player, tick_mob_attack_timer))
            .add_event::<MobAttackEvent>();
        app.add_systems(Update, spawn_goblin)
            .add_event::<SpawnGoblinEvent>();
        app.add_systems(Update, goblin_attack)
            .add_event::<GoblinAttackEvent>();
    }
}
