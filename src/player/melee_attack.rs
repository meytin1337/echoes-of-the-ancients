use bevy::prelude::*;

use crate::mobs::spawn_mobs::Mob;
use crate::player::input_handling::PlayerAttackEvent;
use crate::setup::PlayerStats;

pub struct MeleeAttackPlugin;
impl Plugin for MeleeAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack);
    }
}

fn attack(
    mut player_attack_event_reader: EventReader<PlayerAttackEvent>,
    mut mob_query: Query<&mut Mob>,
    mut command: Commands,
    mut player_query: Query<&mut PlayerStats>,
    time: Res<Time>,
) {
    for event in player_attack_event_reader.read() {
        let mut player_stats = player_query.single_mut();
        player_stats.attack_timer.tick(time.delta());
        if let Ok(mut mob_state) = mob_query.get_mut(event.target) {
            if player_stats.attack_timer.elapsed_secs() > player_stats.attack_speed {
                println!("Attacking mob");
                player_stats.attack_timer.reset();
                mob_state.health -= player_stats.attack_damage - mob_state.armor;
                if mob_state.health <= 0.0 {
                    command.entity(event.target).despawn();
                }
            }
        }
    }
}
