use bevy_xpbd_2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    time::Stopwatch,
};
use crate::mobs::spawn_mobs::{Mob, SpawnGoblinEvent, MobType};

pub struct SpawnGoblinPlugin;

#[derive(Component)]
pub struct Goblin;

#[derive(Bundle)]
pub struct GoblinBundle {
    pub goblin: Goblin,
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
            rigid_body: RigidBody::Dynamic,
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
            goblin: Goblin,
            mob: Mob {
                item_drop_chance: 1.0,
                death_timer: Stopwatch::new(),
                attack_timer: Stopwatch::new(),
                mob_type: MobType::Goblin,
                health: 30.0,
                movement_speed: 1.0,
                view_range: 300.0,
                attack_range: 20.0,
                attack_damage: 10.0,
                attack_speed: 1.0,
                armor: 0.0,
                move_speed: 5000.0,
            },
        });
    }
}
