use crate::mobs::combat::mob_attack::MobAttackEvent;
use crate::mobs::goblin::goblin_attack::GoblinAttackEvent;
use crate::mobs::spawn_mobs::{DeadMob, Mob, MobType};
use crate::player::spawn_player::Player;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Event)]
pub struct MobMoveEvent {
    pub mob_entity: Entity,
    pub target_position: Vec2,
}

#[derive(Component)]
pub struct MobWalkAnimationIndices {
    pub run_up_first: usize,
    pub run_up_last: usize,
    pub run_down_first: usize,
    pub run_down_last: usize,
    pub run_left_first: usize,
    pub run_left_last: usize,
    pub run_right_first: usize,
    pub run_right_last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct MobWalkAnimationTimer(pub Timer);

pub fn target_player(
    player_query: Query<&GlobalTransform, With<Player>>,
    mob_query: Query<(Entity, &Transform, &Collider, &Mob), Without<DeadMob>>,
    mut goblin_attack_event_writer: EventWriter<GoblinAttackEvent>,
    mut mob_move_event_writer: EventWriter<MobMoveEvent>,
    mut mob_attack_event_writer: EventWriter<MobAttackEvent>,
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
            mob_attack_event_writer.send(MobAttackEvent {
                mob_entity,
                attack_vector: Vec2::new(
                    player_global_transform.translation().x - mob_transform.translation.x,
                    player_global_transform.translation().y - mob_transform.translation.y,
                ),
            });
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

pub fn run_to_player(
    mut mob_move_event_reader: EventReader<MobMoveEvent>,
    mut mob_query: Query<
        (
            Entity,
            &GlobalTransform,
            &mut LinearVelocity,
            &Mob,
            &Collider,
        ),
        Without<DeadMob>,
    >,
    spatial_query: SpatialQuery,
    time: Res<Time>,
    player_query: Query<Entity, With<Player>>,
) {
    for event in mob_move_event_reader.read() {
        if let Ok((entity, global_transform, mut linear_velocity, mob_stats, collider)) =
            mob_query.get_mut(event.mob_entity)
        {
            if mob_stats.attack_animation_playing {
                return;
            }
            let direction_vec = Vec2::new(
                event.target_position.x - global_transform.translation().x,
                event.target_position.y - global_transform.translation().y,
            );
            let normalized_direction_vec = direction_vec.normalize();
            let player_entity = player_query.single();
            if let Ok(direction) = Direction2d::new(normalized_direction_vec) {
                if let Some(_) = spatial_query.cast_shape(
                    collider,
                    global_transform.translation().truncate(),
                    0.0,
                    direction,
                    30.0,
                    true,
                    SpatialQueryFilter::from_excluded_entities([entity, player_entity]),
                ) {
                    linear_velocity.x = normalized_direction_vec.y
                        * mob_stats.move_speed
                        * 100.0
                        * time.delta_seconds();
                    linear_velocity.y = -normalized_direction_vec.x
                        * mob_stats.move_speed
                        * 100.0
                        * time.delta_seconds();
                } else {
                    linear_velocity.x = normalized_direction_vec.x
                        * mob_stats.move_speed
                        * 100.0
                        * time.delta_seconds();
                    linear_velocity.y = normalized_direction_vec.y
                        * mob_stats.move_speed
                        * 100.0
                        * time.delta_seconds();
                }
            }
        }
        
    }
}

pub fn mob_walk_animation(
    mut mob_query: Query<(
        &LinearVelocity,
        &Mob,
        &MobWalkAnimationIndices,
        &mut MobWalkAnimationTimer,
        &mut TextureAtlas,
    )>,
    time: Res<Time>,
) {
    for (linear_velocity, mob, indices, mut timer, mut atlas) in &mut mob_query {
        if !mob.attack_animation_playing {
            timer.tick(time.delta());
            if timer.just_finished() {
                if linear_velocity.x > 0.0 && linear_velocity.x.abs() > linear_velocity.y.abs() {
                    atlas.index = if atlas.index == indices.run_right_last {
                        indices.run_right_first
                    } else if atlas.index > indices.run_right_last
                        || atlas.index < indices.run_right_first
                    {
                        indices.run_right_first
                    } else {
                        atlas.index + 1
                    };
                } else if linear_velocity.x < 0.0
                    && linear_velocity.x.abs() > linear_velocity.y.abs()
                {
                    atlas.index = if atlas.index == indices.run_left_last {
                        indices.run_left_first
                    } else if atlas.index > indices.run_left_last
                        || atlas.index < indices.run_left_first
                    {
                        indices.run_left_first
                    } else {
                        atlas.index + 1
                    };
                } else if linear_velocity.y > 0.0
                    && linear_velocity.y.abs() > linear_velocity.x.abs()
                {
                    atlas.index = if atlas.index == indices.run_up_last {
                        indices.run_up_first
                    } else if atlas.index > indices.run_up_last
                        || atlas.index < indices.run_up_first
                    {
                        indices.run_up_first
                    } else {
                        atlas.index + 1
                    };
                } else if linear_velocity.y < 0.0
                    && linear_velocity.y.abs() > linear_velocity.x.abs()
                {
                    atlas.index = if atlas.index == indices.run_down_last {
                        indices.run_down_first
                    } else if atlas.index > indices.run_down_last
                        || atlas.index < indices.run_down_first
                    {
                        indices.run_down_first
                    } else {
                        atlas.index + 1
                    }
                }
            }
        }
    }
}

pub fn apply_mob_movement_damping(
    mut linear_velocity_query: Query<&mut LinearVelocity, With<Mob>>,
) {
    for mut linear_velocity in &mut linear_velocity_query {
        linear_velocity.x *= 0.5;
        linear_velocity.y *= 0.5;
    }
}
