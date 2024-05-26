use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;
use bevy_xpbd_2d::plugins::collision::Collider;

use crate::mobs::spawn_mobs::Mob;
use crate::setup::PlayerStats;
#[derive(Event)]
pub struct PlayerMoveEvent(pub Vec2);

#[derive(Event)]
pub struct PlayerAttackEvent;

#[derive(Component)]
pub struct Targeted;

pub struct InputHandlingPlugin;
impl Plugin for InputHandlingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (target_mob, release_targeted_mob, handle_input))
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerAttackEvent>();
    }
}
fn handle_input(
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut player_attack_event_writer: EventWriter<PlayerAttackEvent>,
    button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mob_query: Query<(&GlobalTransform, &Transform, &Collider), With<Targeted>>,
    player_query: Query<(&PlayerStats, &GlobalTransform)>,
) {
    let (camera, camera_transform) = camera_query.single();
    if button_input.pressed(MouseButton::Left) {
        if let Some(position) = window_query
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            let (player_stats, player_global_transform) = player_query.single();
            if let Ok((mob_global_transform, mob_transform, mob_collider)) = mob_query.get_single()
            {
                // if mob is in attack range, attack it
                // otherwise move towards it
                if mob_collider.distance_to_point(
                    mob_transform.translation.truncate(),
                    mob_transform.rotation,
                    player_global_transform.translation().truncate(),
                    true,
                ) <= player_stats.attack_range {
                    player_attack_event_writer.send(PlayerAttackEvent);
                } else {
                    player_move_event_writer.send(PlayerMoveEvent(
                        mob_global_transform.translation().truncate(),
                    ));
                }
            } else {
                // no mob targeted, move to cursor
                player_move_event_writer.send(PlayerMoveEvent(position));
            }
        }
    }
}

fn target_mob(
    window_query: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<Camera>>,
    mob_query: Query<(&Collider, &Transform, Entity), With<Mob>>,
    mut commands: Commands,
    mut mouse_button_event_reader: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for event in mouse_button_event_reader.read() {
        let (camera, camera_transform) = camera_query.single();
        match event.state {
            ButtonState::Pressed => {
                if let Some(position) = window_query
                    .single()
                    .cursor_position()
                    .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
                {
                    for (collider, mob_transform, entity) in mob_query.iter() {
                        if collider.contains_point(
                            mob_transform.translation.truncate(),
                            mob_transform.rotation,
                            position,
                        ) {
                            commands.entity(entity).insert(Targeted);
                        }
                    }
                }
            }
            ButtonState::Released => (),
        }
    }
}

fn release_targeted_mob(
    mut commands: Commands,
    targeted_mob_query: Query<Entity, (With<Mob>, With<Targeted>)>,
    mut mouse_button_event_reader: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for event in mouse_button_event_reader.read() {
        match event.state {
            ButtonState::Pressed => (),
            ButtonState::Released => {
                if let Ok(targeted_mob) = targeted_mob_query.get_single() {
                    commands.entity(targeted_mob).remove::<Targeted>();
                }
            }
        }
    }
}
