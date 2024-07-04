use crate::player::spawn_player::PlayerAssets;
use crate::player::PlayerStats;
use bevy::prelude::*;
use bevy_xpbd_2d::prelude::LinearVelocity;

use crate::input_handling::PlayerMoveEvent;
use crate::player::spawn_player::{Camera, Player};

#[derive(Event)]
pub struct CameraMoveEvent(pub Vec3);

pub fn run(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut player_query: Query<
        (
            &GlobalTransform,
            &mut Transform,
            &mut LinearVelocity,
            &mut TextureAtlas,
        ),
        With<Player>,
    >,
    player_stats_res: Res<PlayerStats>,
    time: Res<Time>,
    mut camera_move_event_writer: EventWriter<CameraMoveEvent>,
    mut player_assets: ResMut<PlayerAssets>,
) {
    for event in player_move_event_reader.read() {
        let (global_transform, transform, mut linear_velocity, mut atlas) =
            player_query.single_mut();
        let direction = Vec2::new(
            event.0.x - global_transform.translation().x,
            event.0.y - global_transform.translation().y,
        );
        let normalized_direction = direction.normalize();
        linear_velocity.x =
            normalized_direction.x * player_stats_res.move_speed * 100.0 * time.delta_seconds();
        linear_velocity.y =
            normalized_direction.y * player_stats_res.move_speed * 100.0 * time.delta_seconds();
        player_assets.walk_animation_timer.tick(time.delta());
        if player_assets.walk_animation_timer.just_finished() {
            if normalized_direction.x > 0.0
                && normalized_direction.x.abs() > normalized_direction.y.abs()
            {
                atlas.index =
                    if atlas.index == player_assets.general_animation_indices.run_right_last {
                        player_assets.general_animation_indices.run_right_first
                    } else if atlas.index > player_assets.general_animation_indices.run_right_last
                        || atlas.index < player_assets.general_animation_indices.run_right_first
                    {
                        player_assets.general_animation_indices.run_right_first
                    } else {
                        atlas.index + 1
                    };
            } else if normalized_direction.x < 0.0
                && normalized_direction.x.abs() > normalized_direction.y.abs()
            {
                atlas.index =
                    if atlas.index == player_assets.general_animation_indices.run_left_last {
                        player_assets.general_animation_indices.run_left_first
                    } else if atlas.index > player_assets.general_animation_indices.run_left_last
                        || atlas.index < player_assets.general_animation_indices.run_left_first
                    {
                        player_assets.general_animation_indices.run_left_first
                    } else {
                        atlas.index + 1
                    };
            } else if normalized_direction.y > 0.0
                && normalized_direction.y.abs() > normalized_direction.x.abs()
            {
                atlas.index = if atlas.index == player_assets.general_animation_indices.run_up_last
                {
                    player_assets.general_animation_indices.run_up_first
                } else if atlas.index > player_assets.general_animation_indices.run_up_last
                    || atlas.index < player_assets.general_animation_indices.run_up_first
                {
                    player_assets.general_animation_indices.run_up_first
                } else {
                    atlas.index + 1
                };
            } else if normalized_direction.y < 0.0
                && normalized_direction.y.abs() > normalized_direction.x.abs()
            {
                atlas.index =
                    if atlas.index == player_assets.general_animation_indices.run_down_last {
                        player_assets.general_animation_indices.run_down_first
                    } else if atlas.index > player_assets.general_animation_indices.run_down_last
                        || atlas.index < player_assets.general_animation_indices.run_down_first
                    {
                        player_assets.general_animation_indices.run_down_first
                    } else {
                        atlas.index + 1
                    }
            }
        }

        camera_move_event_writer.send(CameraMoveEvent(transform.translation));
    }
}

pub fn apply_movement_damping(mut linear_velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    let mut linear_velocity = linear_velocity_query.single_mut();
    linear_velocity.x *= 0.5;
    linear_velocity.y *= 0.5;
}

pub fn center_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut camera_move_event_reader: EventReader<CameraMoveEvent>,
) {
    for event in camera_move_event_reader.read() {
        let mut camera_transform = camera_transform_query.single_mut();
        camera_transform.translation = event.0;
    }
}
