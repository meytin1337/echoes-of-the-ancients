use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::prelude::*;

pub struct SetupPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    let ball = Mesh2dHandle(meshes.add(Circle { radius }));
    commands.spawn((Camera2dBundle::default(), Camera));
    commands.spawn((
        RigidBody::Kinematic,
        Friction::new(0.0),
        Restitution::new(1.0),
        GravityScale(0.0),
        Rotation::from_degrees(0.0),
        Collider::circle(radius),
        LinearVelocity(Vec2::new(0.0, 0.0)),
        MaterialMesh2dBundle {
            mesh: ball,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            ..default()
        },
        Player,
    ));
}
