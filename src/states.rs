use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

pub struct StatesPlugin;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Location {
    Floor1,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    Playing,
    Paused,
    Inventory,
}

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(Location::Floor1)
            .insert_state(GameState::Playing)
            .add_systems(OnEnter(GameState::Inventory), pause)
            .add_systems(OnExit(GameState::Inventory), unpause);
    }
}

fn pause(mut time: ResMut<Time<Physics>>) {
    time.pause();
}

fn unpause(mut time: ResMut<Time<Physics>>) {
    time.unpause();
}
