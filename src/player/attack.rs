use bevy::prelude::*;

use crate::mobs::spawn_mobs::{Mob, DeadMob};
use crate::player::input_handling::{PlayerAttackEvent, Targeted};
use crate::setup::PlayerStats;

#[derive(Event)]
pub struct MobKillEvent(pub Entity);

#[derive(Event)]
pub struct ItemDropEvent(pub Entity);

pub struct PlayerAttackPlugin;
impl Plugin for PlayerAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack, killed_mob_cleanup))
            .add_event::<MobKillEvent>()
            .add_event::<ItemDropEvent>();
    }
}

fn attack(
    mut player_attack_event_reader: EventReader<PlayerAttackEvent>,
    mut mob_kill_event_writer: EventWriter<MobKillEvent>,
    mut item_drop_event_writer: EventWriter<ItemDropEvent>, 
    mut mob_query: Query<&mut Mob>,
    mut commands: Commands,
    mut player_query: Query<&mut PlayerStats>,
    time: Res<Time>,
) {
    let mut player_stats = player_query.single_mut();
    player_stats.attack_timer.tick(time.delta());
    for event in player_attack_event_reader.read() {
        if let Ok(mut mob_stats) = mob_query.get_mut(event.target) {
            if player_stats.attack_timer.elapsed_secs() > player_stats.attack_speed {
                mob_stats.health -= player_stats.attack_damage - mob_stats.armor;
                println!("Mob health: {}", mob_stats.health);
                if mob_stats.health <= 0.0 {
                    // command.entity(event.target).despawn();
                    item_drop_event_writer.send(ItemDropEvent(event.target));
                    mob_kill_event_writer.send(MobKillEvent(event.target));
                    commands.entity(event.target).remove::<Targeted>();
                    commands.entity(event.target).insert(DeadMob);
                }
                player_stats.attack_timer.reset();
            }
        }
    }
}

fn killed_mob_cleanup(
    mut commands: Commands,
    mut mob_query: Query<(Entity, &mut Mob), With<DeadMob>>,
    time: Res<Time>,
) {
    for (entity, mut mob_stats) in mob_query.iter_mut() {
        mob_stats.death_timer.tick(time.delta());
        if mob_stats.death_timer.elapsed_secs() > 5.0 {
            commands.entity(entity).despawn();
        }
    }
}
