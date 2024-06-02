use crate::mobs::spawn_mobs::Mob;
use crate::player::attack::ItemDropEvent;
use bevy::{
    prelude::*,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_xpbd_2d::plugins::collision::Collider;
use rand::distributions::{Bernoulli, Distribution, WeightedIndex};
use rand::prelude::*;
use std::fmt;

#[derive(Bundle)]
pub struct ItemBundle {
    pub collider: Collider,
    pub item: Item,
    pub material_mesh_2d_bundle: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component)]
pub struct Item {
    pub name: String,
    pub item_type: ItemType,
    pub item_stats: Option<Vec<Stat>>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum ItemType {
    Armor(ArmorType),
    Weapon(WeaponType),
    Gold(u32),
    HealthPotion,
    ManaPotion,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Gloves,
    Amulet,
    Ring,
    Boots,
}

impl fmt::Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum WeaponType {
    Sword,
    Bow,
    Staff,
    Axe,
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Attributes {
    Health,
    Armor,
    Mana,
    MovementSpeed,
    FireDamage,
    ColdDamage,
    PoisonDamage,
    AttackSpeed,
    PhysicalDamage,
}

impl fmt::Display for Attributes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Attributes::MovementSpeed => write!(f, "Movement Speed"),
            Attributes::FireDamage => write!(f, "Fire Damage"),
            Attributes::ColdDamage => write!(f, "Cold Damage"),
            Attributes::PoisonDamage => write!(f, "Poison Damage"),
            Attributes::AttackSpeed => write!(f, "Attack Speed"),
            Attributes::PhysicalDamage => write!(f, "Physical Damage"),
            _ => write!(f, "{:?}", self),
        }
    }
}

pub struct Stat {
    pub attribute_type: Attributes,
    pub value: u32,
}

pub struct AttributeRange {
    attribute_type: Attributes,
    min: u32,
    max: u32,
}

const ARMOR_ATTRIBUTES: [AttributeRange; 4] = [
    AttributeRange {
        attribute_type: Attributes::Mana,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::Health,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::Armor,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::MovementSpeed,
        min: 1,
        max: 10,
    },
];
const WEAPON_ATTRIBUTES: [AttributeRange; 5] = [
    AttributeRange {
        attribute_type: Attributes::FireDamage,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::ColdDamage,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::PoisonDamage,
        min: 10,
        max: 100,
    },
    AttributeRange {
        attribute_type: Attributes::AttackSpeed,
        min: 1,
        max: 10,
    },
    AttributeRange {
        attribute_type: Attributes::PhysicalDamage,
        min: 10,
        max: 100,
    },
];

const ITEM_PREFIXES: [&str; 26] = [
    "Ancient",
    "Battleworn",
    "Royal",
    "Forged",
    "Noble",
    "Minstrel's",
    "Rustic",
    "Sturdy",
    "Polished",
    "Ornate",
    "Handcrafted",
    "Durable",
    "Refined",
    "Elegant",
    "Prized",
    "Treasured",
    "Heirloom",
    "Vintage",
    "Classic",
    "Timeless",
    "Exquisite",
    "Masterwork",
    "Rugged",
    "Robust",
    "Weathered",
    "Cherished",
];

const ITEM_SUFFIXES: [&str; 30] =
[
    "Valor",
    "Honor",
    "Protection",
    "Fortitude",
    "Might",
    "Triumph",
    "Courage",
    "Glory",
    "Dominance",
    "Conquest",
    "the Empire",
    "the Kingdom",
    "the Forest",
    "the Mountains",
    "the Plains",
    "the Seas",
    "the Ancestors",
    "the Monarch",
    "the Fallen",
    "the Brave",
    "the Guardian",
    "the Defender",
    "the Martyr",
    "the Elder",
    "the Conqueror",
    "the Hero",
    "the Paladin",
    "the Warrior",
    "the Knight",
    "the Bard"
];

pub fn roll_item_type() -> ItemType {
    let mut rng = rand::thread_rng();
    // * monster multiplier
    let item_types_with_weights = [
        (ItemType::Gold(rng.gen_range(1..500)), 0.3),
        (ItemType::Armor(ArmorType::Ring), 0.005),
        (ItemType::Armor(ArmorType::Amulet), 0.005),
        (ItemType::Armor(ArmorType::Chestplate), 0.038),
        (ItemType::Armor(ArmorType::Boots), 0.038),
        (ItemType::Armor(ArmorType::Gloves), 0.038),
        (ItemType::Armor(ArmorType::Helmet), 0.038),
        (ItemType::Armor(ArmorType::Gloves), 0.038),
        (ItemType::Weapon(WeaponType::Bow), 0.05),
        (ItemType::Weapon(WeaponType::Staff), 0.05),
        (ItemType::Weapon(WeaponType::Sword), 0.05),
        (ItemType::Weapon(WeaponType::Axe), 0.05),
        (ItemType::HealthPotion, 0.15),
        (ItemType::ManaPotion, 0.15),
    ];
    let item_type_distribution =
        WeightedIndex::new(item_types_with_weights.iter().map(|x| x.1)).unwrap();
    item_types_with_weights[item_type_distribution.sample(&mut rng)].0
}

pub fn roll_item_stats(_monster_level: u32, item_type: ItemType) -> Option<Vec<Stat>> {
    // todo: include monster level in the calculation
    let mut rng = rand::thread_rng();
    let mut item_stats = Vec::new();
    let mut item_stat_counter = 0;
    let distribution = Bernoulli::new(0.5).unwrap();
    match item_type {
        ItemType::Armor(_) => {
            while item_stat_counter < 3 {
                let armor_attribute_index = rng.gen_range(0..ARMOR_ATTRIBUTES.len());
                let armor_attribute = &ARMOR_ATTRIBUTES[armor_attribute_index];
                if item_stats.len() > 0
                    && item_stats
                        .iter()
                        .map(|stat: &Stat| stat.attribute_type)
                        .collect::<Vec<Attributes>>()
                        .contains(&armor_attribute.attribute_type)
                {
                    continue;
                }
                if distribution.sample(&mut rng) {
                    item_stats.push(Stat {
                        attribute_type: armor_attribute.attribute_type,
                        value: rng.gen_range(armor_attribute.min..armor_attribute.max),
                    });
                }
                item_stat_counter += 1;
            }
        }
        ItemType::Weapon(_) => {
            while item_stat_counter < 3 {
                let weapon_attribute_index = rng.gen_range(0..WEAPON_ATTRIBUTES.len());
                let weapon_attribute = &WEAPON_ATTRIBUTES[weapon_attribute_index];
                if item_stats.len() > 0
                    && item_stats
                        .iter()
                        .map(|stat: &Stat| stat.attribute_type)
                        .collect::<Vec<Attributes>>()
                        .contains(&weapon_attribute.attribute_type)
                {
                    continue;
                }
                if distribution.sample(&mut rng) {
                    // no need option
                    item_stats.push(Stat {
                        attribute_type: weapon_attribute.attribute_type,
                        value: rng.gen_range(weapon_attribute.min..weapon_attribute.max),
                    });
                }
                item_stat_counter += 1;
            }
        }
        _ => return None,
    }
    Some(item_stats)
}

fn roll_item_name(item_type: ItemType) -> String {
    match item_type {
        ItemType::Gold(_) => String::from("Gold"),
        ItemType::HealthPotion => String::from("Health Potion"),
        ItemType::ManaPotion => String::from("Mana Potion"),
        ItemType::Armor(armor_type) => {
            let prefix = ITEM_PREFIXES[rand::thread_rng().gen_range(0..ITEM_PREFIXES.len())];
            let suffix = ITEM_SUFFIXES[rand::thread_rng().gen_range(0..ITEM_SUFFIXES.len())];
            format!("{} {} of {}", prefix, armor_type, suffix)
        }
        ItemType::Weapon(weapon_type) => {
            let prefix = ITEM_PREFIXES[rand::thread_rng().gen_range(0..ITEM_PREFIXES.len())];
            let suffix = ITEM_SUFFIXES[rand::thread_rng().gen_range(0..ITEM_SUFFIXES.len())];
            format!("{} {} of {}", prefix, weapon_type, suffix)
        }
    }
}

pub fn drop_item(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mob_query: Query<(&Mob, &Transform)>,
    mut item_drop_event_reader: EventReader<ItemDropEvent>,
) {
    let mut rng = rand::thread_rng();
    for event in item_drop_event_reader.read() {
        if let Ok((mob_attributes, mob_transform)) = mob_query.get(event.0) {
            // fix this, we want
            let drop_distribution = Bernoulli::new(mob_attributes.item_drop_chance as f64).unwrap();
            if drop_distribution.sample(&mut rng) {
                let item_type = roll_item_type();
                commands.spawn(ItemBundle {
                    collider: Collider::rectangle(20.0, 20.0),
                    item: Item {
                        item_type,
                        item_stats: roll_item_stats(1, item_type),
                        name: roll_item_name(item_type),
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
