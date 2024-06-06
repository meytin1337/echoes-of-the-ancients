use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::mobs::spawn_mobs::Mob;
use crate::player::{spawn_player::Player, PlayerStats};

#[derive(Event)]
pub struct MobAttackEvent {
    pub mob_entity: Entity,
    pub attack_vector: Vec2,
}

#[derive(Component)]
pub struct MobAttackAnimationIndices {
    pub attack_up_first: usize,
    pub attack_up_last: usize,
    pub attack_down_first: usize,
    pub attack_down_last: usize,
    pub attack_left_first: usize,
    pub attack_left_last: usize,
    pub attack_right_first: usize,
    pub attack_right_last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct MobAttackAnimationTimer(pub Timer);

pub fn attack_player(
    mut mob_melee_attack_event_reader: EventReader<MobAttackEvent>,
    mut mob_query: Query<(
        &mut Mob,
        &MobAttackAnimationIndices,
        &mut TextureAtlas,
        &MobAttackAnimationTimer,
    )>,
) {
    // always tick time so that first attack on mobs will be instant
    for event in mob_melee_attack_event_reader.read() {
        if let Ok((mut mob_stats, indices, mut atlas, timer)) = mob_query.get_mut(event.mob_entity)
        {
            mob_stats.attack_animation_playing = true;
            // start animation if index is out of animation atlas
            if event.attack_vector.x > 0.0
                && event.attack_vector.x.abs() > event.attack_vector.y.abs()
            {
                if atlas.index > indices.attack_right_last
                    || atlas.index < indices.attack_right_first
                {
                    atlas.index = indices.attack_right_first
                }
            } else if event.attack_vector.x < 0.0
                && event.attack_vector.x.abs() > event.attack_vector.y.abs()
            {
                if atlas.index > indices.attack_left_last || atlas.index < indices.attack_left_first
                {
                    atlas.index = indices.attack_left_first
                }
            } else if event.attack_vector.y > 0.0
                && event.attack_vector.y.abs() > event.attack_vector.x.abs()
            {
                if atlas.index > indices.attack_up_last || atlas.index < indices.attack_up_first {
                    atlas.index = indices.attack_up_first
                }
            } else if event.attack_vector.y < 0.0
                && event.attack_vector.y.abs() > event.attack_vector.x.abs()
            {
                if atlas.index > indices.attack_down_last || atlas.index < indices.attack_down_first
                {
                    atlas.index = indices.attack_down_first
                }
            }
            // reset animation if its finished
            if atlas.index == indices.attack_down_last {
                if timer.finished() {
                    atlas.index = indices.attack_down_first;
                }
            } else if atlas.index == indices.attack_up_last {
                if timer.finished() {
                    atlas.index = indices.attack_up_first;
                }
            } else if atlas.index == indices.attack_left_last {
                if timer.finished() {
                    atlas.index = indices.attack_left_first;
                }
            } else if atlas.index == indices.attack_right_last {
                if timer.finished() {
                    atlas.index = indices.attack_right_first;
                }
            }
        }
    }
}


pub fn play_attack_animation(
    mut mob_query: Query<(
        &MobAttackAnimationIndices,
        &mut TextureAtlas,
        &mut MobAttackAnimationTimer,
        &mut Mob,
        &Transform,
        &Collider,
    )>,
    mut player_stats: ResMut<PlayerStats>,
    player_global_transform: Query<&GlobalTransform, With<Player>>,
    time: Res<Time>,
) {
    for (indices, mut atlas, mut timer, mut mob, mob_transform, mob_collider) in
        mob_query.iter_mut()
    {
        if atlas.index < indices.attack_down_last && atlas.index >= indices.attack_down_first
            || atlas.index < indices.attack_up_last && atlas.index >= indices.attack_up_first
            || atlas.index < indices.attack_left_last && atlas.index >= indices.attack_left_first
            || atlas.index < indices.attack_right_last && atlas.index >= indices.attack_right_first
        {
            timer.tick(time.delta());
            if timer.just_finished() {
                // fix: on last animation the atlas gets immediately reset to first
                atlas.index += 1;
                let player_global_transform = player_global_transform.single();
                if (atlas.index == indices.attack_down_last
                    || atlas.index == indices.attack_up_last
                    || atlas.index == indices.attack_left_last
                    || atlas.index == indices.attack_right_last)
                    && mob_collider.distance_to_point(
                        mob_transform.translation.truncate(),
                        mob_transform.rotation,
                        player_global_transform.translation().truncate(),
                        true,
                    ) <= mob.attack_range
                {
                    player_stats.health -= mob.attack_damage - player_stats.armor;
                }
            }
        } else {
            mob.attack_animation_playing = false;
            mob.attack_timer.reset();
        }
    }
}

pub fn tick_mob_attack_timer(mut mob_query: Query<&mut Mob>, time: Res<Time>) {
    for mut mob in mob_query.iter_mut() {
        mob.attack_timer.tick(time.delta());
    }
}
