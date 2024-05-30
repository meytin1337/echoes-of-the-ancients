use crate::mobs::spawn_mobs::Mob;
use crate::player::attack::ItemDropEvent;
use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::plugins::collision::Collider;
use rand::prelude::*;
use rand::distributions::{Bernoulli, Distribution};

#[derive(Bundle)]
pub struct ItemBundle {
    pub collider: Collider,
    pub item: Item,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
pub enum Item {
    Armor(Armor),
    Weapon(Weapon),
}

pub struct Armor {
    first_armor_stat: Option<ArmorStats>,
    second_armor_stat: Option<ArmorStats>,
    third_armor_stat: Option<ArmorStats>,
}

pub struct Weapon {
    first_weapon_stat: Option<WeaponStats>,
    second_weapon_stat: Option<WeaponStats>,
    third_weapon_stat: Option<WeaponStats>,
}

pub enum ArmorStats {
    Health(f32),
    Mana(f32),
    Armor(f32),
}

pub enum WeaponStats {
    AttackDamage(f32),
    AttackSpeed(f32),
    FireDamage(f32),
}

pub struct DropPlugin;


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
                        item: Item::Armor(Armor {
                            first_armor_stat: Some(ArmorStats::Health(10.0)),
                            second_armor_stat: Some(ArmorStats::Armor(5.0)),
                            third_armor_stat: None,
                        }),
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
                        item: Item::Weapon(Weapon {
                            first_weapon_stat: Some(WeaponStats::AttackDamage(10.0)),
                            second_weapon_stat: Some(WeaponStats::AttackSpeed(1.0)),
                            third_weapon_stat: None,
                        }),
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
