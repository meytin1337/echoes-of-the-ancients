use crate::mobs::{
    general_mob_behaviour::{
        mob_walk_animation, apply_mob_movement_damping, target_player, run_to_player, MobMoveEvent,
    },
    goblin::{
        goblin_attack::{goblin_attack, GoblinAttackEvent},
        spawn_goblin::spawn_goblin,
    },
    spawn_mobs::{floor_1_spawner, SpawnGoblinEvent},
    combat::mob_attack::{attack_player, tick_mob_attack_timer, play_attack_animation, MobAttackEvent},
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
        .add_systems(
            Update,
            (target_player, run_to_player, mob_walk_animation, apply_mob_movement_damping, attack_player, play_attack_animation.after(attack_player), tick_mob_attack_timer, goblin_attack, spawn_goblin).in_set(crate::sets::PlayingSet))
            .add_event::<SpawnGoblinEvent>()
            .add_event::<MobAttackEvent>()
            .add_event::<SpawnGoblinEvent>()
            .add_event::<GoblinAttackEvent>()
            .add_event::<MobMoveEvent>();
    }
}
