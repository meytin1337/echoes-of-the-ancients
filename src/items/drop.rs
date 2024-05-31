use crate::mobs::spawn_mobs::Mob;
use crate::player::attack::ItemDropEvent;
use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::plugins::collision::Collider;
use rand::distributions::{Bernoulli, Distribution};
use rand::prelude::*;

#[derive(Bundle)]
pub struct ItemBundle {
    pub collider: Collider,
    pub item: Item,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
pub struct Item {
    pub item_stats: ItemStats,
    pub name: String,
    pub item_type: ItemType,
}

#[derive(PartialEq)]
pub enum ItemType {
    Armor,
    Weapon,
}

pub struct ItemStats {
    pub health: Option<f32>,
    pub armor: Option<f32>,
    pub movement_speed: Option<f32>,
    pub fire_damage: Option<f32>,
    pub ice_damage: Option<f32>,
    pub poison_damage: Option<f32>,
    pub attack_speed: Option<f32>,
}

pub fn drop_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mob_query: Query<(&Mob, &Transform)>,
    mut item_drop_event_reader: EventReader<ItemDropEvent>,
) {
    let mut rng = rand::thread_rng();
    let item_random_number: f32 = rng.gen();
    for event in item_drop_event_reader.read() {
        if let Ok((mob_stats, mob_transform)) = mob_query.get(event.0) {
            // fix this, we want
            let drop_distribution = Bernoulli::new(mob_stats.item_drop_chance as f64).unwrap();
            if drop_distribution.sample(&mut rng) {
                // todo: randomize item stats
                if item_random_number > 0.5 {
                    commands.spawn(ItemBundle {
                        collider: Collider::rectangle(20.0, 20.0),
                        item: Item {
                            item_type: ItemType::Armor,
                            item_stats: ItemStats {
                                health: Some(10.0),
                                armor: Some(10.0),
                                movement_speed: Some(1.0),
                                fire_damage: None,
                                ice_damage: None,
                                poison_damage: None,
                                attack_speed: None,
                            },
                            name: String::from("test"),

                        },
                        material_mesh_2d_bundle: MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Rectangle {
                                half_size: Vec2::new(10.0, 10.0),
                            })),
                            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
                            transform: Transform {
                                translation: mob_transform.translation,
                                ..Transform::default()
                            },
                            ..default()
                        },
                    });
                } else {
                    commands.spawn(ItemBundle {
                        collider: Collider::rectangle(20.0, 20.0),
                        item: Item {
                            item_stats: ItemStats {
                                health: None,
                                armor: None,
                                movement_speed: None,
                                fire_damage: Some(10.0),
                                ice_damage: Some(10.0),
                                poison_damage: Some(10.0),
                                attack_speed: Some(1.0),
                            },
                            item_type: ItemType::Weapon,
                            name: String::from("test_weapon"),
                        },
                        material_mesh_2d_bundle: MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Rectangle {
                                half_size: Vec2::new(10.0, 10.0),
                            })),
                            material: materials.add(Color::rgb(0.0, 1.0, 0.0)),
                            transform: Transform {
                                translation: mob_transform.translation,
                                ..Transform::default()
                            },
                            ..default()
                        },
                    });
                }
            }
        }
    }
}
