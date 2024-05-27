use bevy::prelude::*;

use crate::mobs::spawn_mobs::Mob;
use crate::player::input_handling::PlayerAttackEvent;
use crate::setup::PlayerStats;

pub struct PlayerAttackPlugin;
impl Plugin for PlayerAttackPlugin {
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
    let mut player_stats = player_query.single_mut();
    player_stats.attack_timer.tick(time.delta());
    for event in player_attack_event_reader.read() {
        if let Ok(mut mob_stats) = mob_query.get_mut(event.target) {
            println!("Player attacked mob {:?}", player_stats.attack_timer.elapsed_secs());
            if player_stats.attack_timer.elapsed_secs() > player_stats.attack_speed {
                mob_stats.health -= player_stats.attack_damage - mob_stats.armor;
                println!("Mob health: {}", mob_stats.health);
                if mob_stats.health <= 0.0 {
                    command.entity(event.target).despawn();
                }
                player_stats.attack_timer.reset();
            }
        }
    }
}
