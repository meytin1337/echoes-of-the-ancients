use bevy::prelude::*;

use crate::mobs::spawn_mobs::Mob;
use crate::player::spawn_player::PlayerStats;

#[derive(Event)]
pub struct MobAttackEvent {
    pub mob_entity: Entity,
}

pub fn attack_player(
    mut mob_melee_attack_event_reader: EventReader<MobAttackEvent>,
    mut player_query: Query<&mut PlayerStats>,
    mut mob_query: Query<&mut Mob>,
) {
    // always tick time so that first attack on mobs will be instant
    let mut player_stats = player_query.single_mut();
    for event in mob_melee_attack_event_reader.read() {
        if let Ok(mut mob_stats) = mob_query.get_mut(event.mob_entity) {
            if mob_stats.attack_timer.elapsed_secs() > mob_stats.attack_speed {
                player_stats.health -= mob_stats.attack_damage - player_stats.armor;
                mob_stats.attack_timer.reset();
            }
        }
    }
}

pub fn tick_mob_attack_timer(mut mob_query: Query<&mut Mob>, time: Res<Time>) {
    for mut mob in mob_query.iter_mut() {
        mob.attack_timer.tick(time.delta());
    }
}
