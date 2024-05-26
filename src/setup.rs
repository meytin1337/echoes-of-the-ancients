use bevy_xpbd_2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct SetupPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerStats {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
    pub attack_range: f32,
}

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
        PlayerStats {
            health: 100.0,
            attack: 10.0,
            defense: 5.0,
            attack_range: 20.0,
        },
    ));
}
