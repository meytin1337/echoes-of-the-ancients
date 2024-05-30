use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayingSet;

pub struct SetsPlugin;
impl Plugin for SetsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, PlayingSet.run_if(in_state(crate::states::GameState::Playing)));
    }
}
