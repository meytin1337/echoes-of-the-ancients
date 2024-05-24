use bevy::prelude::*;
use bevy_xpbd_2d::prelude::LinearVelocity;

use crate::character::input_handling::MouseRightClickEvent;
use crate::setup::{Camera, Player};

#[derive(Event)]
pub struct PlayerMoveEvent(pub Vec3);


pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (run, apply_movement_damping, center_camera))
            .add_event::<PlayerMoveEvent>();
    }
}

// todo: use time delta for movement
fn run(
    mut mouse_right_click_event_reader: EventReader<MouseRightClickEvent>,
    mut player_linear_velocity_query: Query<&mut LinearVelocity, With<Player>>,
    player_transform_query: Query<&Transform, With<Player>>,
    window_query: Query<&Window>,
    time: Res<Time>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
) {
    for event in mouse_right_click_event_reader.read() {
        let mut linear_velocity = player_linear_velocity_query.single_mut();
        println!("event: {:?}", event.0);
        let window = window_query.single();
        // center is at window width / 2, window height / 2
        // but the y axis is flipped
        let direction = Vec2::new(
            event.0.x - window.width() / 2.0,
            window.height() / 2.0 - event.0.y,
        );
        let normalized_direction = direction.normalize();
        linear_velocity.x = normalized_direction.x * 10000.0 * time.delta_seconds();
        linear_velocity.y = normalized_direction.y * 10000.0 * time.delta_seconds();
        let transform = player_transform_query.single();
        println!("transform: {:?}", transform);
        player_move_event_writer.send(PlayerMoveEvent(transform.translation));
    }
}

fn apply_movement_damping(mut linear_velocity_query: Query<&mut LinearVelocity, With<Player>>) {
    let mut linear_velocity = linear_velocity_query.single_mut();
    linear_velocity.x *= 0.5;
    linear_velocity.y *= 0.5;
}

fn center_camera(
    mut camera_transform_query: Query<&mut Transform, With<Camera>>,
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
) {
    for event in player_move_event_reader.read() {
        let mut camera_transform = camera_transform_query.single_mut();
        camera_transform.translation = event.0;
    }
}
