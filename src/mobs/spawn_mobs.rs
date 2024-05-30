use bevy::prelude::*;

use bevy::time::Stopwatch;

#[derive(Component)]
pub struct Mob {
    pub health: f32,
    pub movement_speed: f32,
    pub view_range: f32,
    pub attack_range: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub attack_timer: Stopwatch,
    pub armor: f32,
    pub mob_type: MobType,
    pub move_speed: f32,
    pub death_timer: Stopwatch,
    pub item_drop_chance: f32,
}

#[derive(Component)]
pub struct DeadMob;

#[derive(Clone)]
pub enum MobType {
    Goblin,
}

#[derive(Event)]
pub struct SpawnGoblinEvent(pub Vec3);


pub fn floor_1_spawner(mut spawn_goblin_event_writer: EventWriter<SpawnGoblinEvent>) {
    // todo only spawn goblins if there are no goblins in the room 
    // safe amount of spawned mobs in some struct
    // goblin positions should be random
    // eventually the mobs spawned should be randomly selected between different types of mobs 
    for i in -3..3 {
        spawn_goblin_event_writer.send(SpawnGoblinEvent(Vec3::new(i as f32 * 100.0, i as f32 * 100.0, 0.0)));
    }
}
