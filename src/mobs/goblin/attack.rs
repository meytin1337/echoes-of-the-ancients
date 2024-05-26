use bevy::prelude::*;

#[derive(Event)]
pub struct GoblinAttackEvent {
    pub goblin_entity: Entity,
}

pub struct GoblinAttackPlugin;

impl Plugin for GoblinAttackPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_player)
            .add_event::<GoblinAttackEvent>();
    }
}

fn attack_player() {

}
