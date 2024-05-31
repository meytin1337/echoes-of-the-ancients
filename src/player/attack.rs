use bevy::prelude::*;
use bevy_xpbd_2d::plugins::collision::Collider;
use bevy_xpbd_2d::components::RigidBody;

use crate::mobs::spawn_mobs::{Mob, DeadMob};
use crate::input_handling::{PlayerAttackEvent, Targeted};

use crate::player::PlayerStats;

#[derive(Event)]
pub struct MobKillEvent(pub Entity);

#[derive(Event)]
pub struct ItemDropEvent(pub Entity);

pub fn attack(
    mut player_attack_event_reader: EventReader<PlayerAttackEvent>,
    mut mob_kill_event_writer: EventWriter<MobKillEvent>,
    mut item_drop_event_writer: EventWriter<ItemDropEvent>, 
    mut mob_query: Query<&mut Mob>,
    mut commands: Commands,
    mut player_stats: ResMut<PlayerStats>,
    time: Res<Time>,
) {
    player_stats.attack_timer.tick(time.delta());
    for event in player_attack_event_reader.read() {
        if let Ok(mut mob_stats) = mob_query.get_mut(event.target) {
            if player_stats.attack_timer.elapsed_secs() > player_stats.attack_speed {
                // add items to calculation
                mob_stats.health -= player_stats.attack_damage - mob_stats.armor;
                if mob_stats.health <= 0.0 {
                    item_drop_event_writer.send(ItemDropEvent(event.target));
                    mob_kill_event_writer.send(MobKillEvent(event.target));
                    commands.entity(event.target).remove::<Targeted>();
                    commands.entity(event.target).remove::<RigidBody>();
                    commands.entity(event.target).remove::<Collider>();
                    commands.entity(event.target).insert(DeadMob);
                }
                player_stats.attack_timer.reset();
            }
        }
    }
}

pub fn killed_mob_cleanup(
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
