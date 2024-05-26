use crate::mobs::goblin::attack::GoblinAttackEvent;
use crate::mobs::spawn_mobs::Mob;
use crate::mobs::spawn_mobs::MobType;
use crate::setup::Player;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Component)]
pub struct Attacking;
#[derive(Event)]
pub struct MobMoveEvent {
    pub mob_entity: Entity,
    pub target_position: Vec2,
}

pub struct GeneralMobBehaviourPlugin;

impl Plugin for GeneralMobBehaviourPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (attack_player, mob_move))
            .add_event::<MobMoveEvent>();
    }
}

fn attack_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mob_query: Query<(Entity, &Transform, &Collider, &Mob)>,
    mut goblin_attack_event_writer: EventWriter<GoblinAttackEvent>,
    mut mob_move_event_writer: EventWriter<MobMoveEvent>,
) {
    for (mob_entity, mob_transform, mob_collider, mob_stats) in &mob_query {
        let player_global_transform = player_query.single();
        if mob_collider.distance_to_point(
            mob_transform.translation.truncate(),
            mob_transform.rotation,
            player_global_transform.translation().truncate(),
            true,
        ) <= mob_stats.attack_range
        {
            println!("Mob attacking");
            match mob_stats.mob_type {
                MobType::Goblin => {
                    goblin_attack_event_writer.send(GoblinAttackEvent {
                        goblin_entity: mob_entity,
                    });
                }
            }
        } else {
            mob_move_event_writer.send(MobMoveEvent {
                mob_entity,
                target_position: player_global_transform.translation().truncate(),
            });
        }
    }
}

fn mob_move(
    mut mob_move_event_reader: EventReader<MobMoveEvent>,
    mut mob_query: Query<(&GlobalTransform, &mut LinearVelocity, &Mob)>,
    time: Res<Time>,
) {
    for event in mob_move_event_reader.read() {
        if let Ok((global_transform, mut linear_velocity, mob_state)) =
            mob_query.get_mut(event.mob_entity)
        {
            println!("Mob moving");
            let direction = Vec2::new(
                event.target_position.x - global_transform.translation().x,
                event.target_position.y - global_transform.translation().y,
            );
            let normalized_direction = direction.normalize();
            linear_velocity.x =
                normalized_direction.x * mob_state.move_speed * time.delta_seconds();
            linear_velocity.y =
                normalized_direction.y * mob_state.move_speed * time.delta_seconds();
        }
    }
}

fn apply_movement_damping(mut linear_velocity_query: Query<&mut LinearVelocity, With<Mob>>) {
    let mut linear_velocity = linear_velocity_query.single_mut();
    linear_velocity.x *= 0.5;
    linear_velocity.y *= 0.5;
}
