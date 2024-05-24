use bevy_xpbd_2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct SetupPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let radius = 10.0;
    let ball = Mesh2dHandle(meshes.add(Circle { radius }));
    commands.spawn((Camera2dBundle::default(), Camera));
    commands.spawn((
        RigidBody::Dynamic,
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
    let rectangle = Mesh2dHandle(meshes.add(Rectangle {
        half_size: Vec2::new(100.0, 100.0),
    }));
    commands.spawn((
        RigidBody::Static,
        Friction::new(0.0),
        Restitution::new(1.0),
        GravityScale(0.0),
        Rotation::from_degrees(0.0),
        Collider::rectangle(200.0, 200.0),
        LinearVelocity(Vec2::new(0.0, 0.0)),
        MaterialMesh2dBundle {
            mesh: rectangle,
            material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
            transform: Transform {
                translation: Vec3 {
                    x: 0.0,
                    // add offset to prevent ball from getting stuck
                    y: 300.0,
                    z: 0.0,
                },
                ..Transform::default()
            },
            ..default()
        },
    ));
}
