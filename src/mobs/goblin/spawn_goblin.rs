use bevy_xpbd_2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::mobs::spawn_mobs::{Mob, SpawnGoblinEvent};

pub struct SpawnGoblinPlugin;

#[derive(Component)]
pub struct GoblinState {
    pub health: f32,
    pub attack: f32,
    pub defense: f32,
}

#[derive(Bundle)]
pub struct GoblinBundle {
    pub goblin_state: GoblinState,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub linear_velocity: LinearVelocity,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
    pub mob: Mob,
}

impl Plugin for SpawnGoblinPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn);
    }
}


fn spawn(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut spawn_goblin_event_reader: EventReader<SpawnGoblinEvent>,
) {
    for event in spawn_goblin_event_reader.read() {
        let rectangle = Mesh2dHandle(meshes.add(Rectangle {
            half_size: Vec2::new(10.0, 10.0),
        }));
        commands.spawn(GoblinBundle {
            rigid_body: RigidBody::Static,
            collider: Collider::rectangle(20.0, 20.0),
            linear_velocity: LinearVelocity(Vec2::new(0.0, 0.0)),
            material_mesh_2d_bundle: MaterialMesh2dBundle {
                mesh: rectangle,
                material: materials.add(Color::rgb(1.0, 0.0, 0.0)),
                transform: Transform {
                    translation: event.0,
                    ..Transform::default()
                },
                ..default()
            },
            goblin_state: GoblinState {
                health: 100.0,
                attack: 10.0,
                defense: 5.0,
            },
            mob: Mob,
        });
    }
}
