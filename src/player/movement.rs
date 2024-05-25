use bevy::prelude::*;
use bevy_xpbd_2d::prelude::LinearVelocity;

use crate::player::input_handling::PlayerMoveEvent;
use crate::setup::{Camera, Player};

#[derive(Event)]
pub struct CameraMoveEvent(pub Vec3);


pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (run, apply_movement_damping, center_camera))
            .add_event::<PlayerMoveEvent>()
            .add_event::<CameraMoveEvent>();
    }
}

// todo: use time delta for movement
fn run(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut player_linear_velocity_query: Query<&mut LinearVelocity, With<Player>>,
    player_query: Query<(&GlobalTransform, &Transform), With<Player>>,
    time: Res<Time>,
    mut camera_move_event_writer: EventWriter<CameraMoveEvent>,
) {
    for event in player_move_event_reader.read() {
        let mut linear_velocity = player_linear_velocity_query.single_mut();
        let (global_transform, transform) = player_query.single();
        // todo: convert cursor to world coordinates: https://bevy-cheatbook.github.io/cookbook/cursor2world.html
        let direction = Vec2::new(
            event.0.x - global_transform.translation().x,
            event.0.y - global_transform.translation().y,
        );
        let normalized_direction = direction.normalize();
        linear_velocity.x = normalized_direction.x * 10000.0 * time.delta_seconds();
        linear_velocity.y = normalized_direction.y * 10000.0 * time.delta_seconds();
        camera_move_event_writer.send(CameraMoveEvent(transform.translation));
    }
}

fn apply_movement_damping(mut linear_velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    let mut linear_velocity = linear_velocity_query.single_mut();
    linear_velocity.x *= 0.5;
    linear_velocity.y *= 0.5;
}

fn center_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut camera_move_event_reader: EventReader<CameraMoveEvent>,
) {
    for event in camera_move_event_reader.read() {
        let mut camera_transform = camera_transform_query.single_mut();
        camera_transform.translation = event.0;
    }
}
