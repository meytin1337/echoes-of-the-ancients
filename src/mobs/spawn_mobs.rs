use bevy::prelude::*;

use crate::game_state::Location;

pub struct SpawnMobsPlugin;

#[derive(Component)]
pub struct Mob;

#[derive(Event)]
pub struct SpawnGoblinEvent(pub Vec3);

impl Plugin for SpawnMobsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(Location::Floor1), floor_1_spawner) .add_event::<SpawnGoblinEvent>();
    }
}

fn floor_1_spawner(mut spawn_goblin_event_writer: EventWriter<SpawnGoblinEvent>) {
    // todo only spawn goblins if there are no goblins in the room 
    // safe amount of spawned mobs in some struct
    // goblin positions should be random
    // eventually the mobs spawned should be randomly selected between different types of mobs 
    println!("Spawning goblins");
    for i in -3..3 {
        spawn_goblin_event_writer.send(SpawnGoblinEvent(Vec3::new(i as f32 * 100.0, i as f32 * 100.0, 0.0)));
    }
}
