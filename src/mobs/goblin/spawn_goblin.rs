use bevy_xpbd_2d::prelude::*;
use bevy::{
    prelude::*,
    time::Stopwatch,
};
use crate::mobs::general_mob_behaviour::{MobWalkAnimationIndices, MobWalkAnimationTimer};
use crate::mobs::combat::mob_attack::{MobAttackAnimationIndices, MobAttackAnimationTimer};
use crate::mobs::spawn_mobs::{Mob, SpawnGoblinEvent, MobType};

#[derive(Component)]
pub struct Goblin;

#[derive(Bundle)]
pub struct GoblinBundle {
    pub goblin: Goblin,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub linear_velocity: LinearVelocity,
    pub sprite_sheet_bundle: SpriteSheetBundle,
    pub mob: Mob,
    pub walk_animation_indices: MobWalkAnimationIndices,
    pub walk_animation_timer: MobWalkAnimationTimer,
    pub attack_animation_indices: MobAttackAnimationIndices,
    pub attack_animation_timer: MobAttackAnimationTimer,
    pub locked_axes: LockedAxes,
}


pub fn spawn_goblin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_goblin_event_reader: EventReader<SpawnGoblinEvent>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("goblin/goblin.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(128.0, 128.0), 8, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    for event in spawn_goblin_event_reader.read() {
        commands.spawn(GoblinBundle {
            locked_axes: LockedAxes::ROTATION_LOCKED,
            rigid_body: RigidBody::Dynamic,
            // todo: fix collider size
            // also check out logical and physical size
            collider: Collider::rectangle(32.0, 52.0),
            linear_velocity: LinearVelocity(Vec2::new(0.0, 0.0)),
            sprite_sheet_bundle: SpriteSheetBundle {
                texture: texture.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout.clone(),
                    index: 16,
                },
                transform: Transform {
                    translation: event.0,
                    scale: Vec3::splat(0.8),
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
                health: 30,
                movement_speed: 1,
                view_range: 300.0,
                attack_range: 30.0,
                attack_damage: 10,
                attack_speed: 1.0,
                armor: 0,
                move_speed: 50.0,
                attack_animation_playing: false,
                run_animation_playing: false,
            },
            walk_animation_indices: MobWalkAnimationIndices {
                run_up_first: 0,
                run_up_last: 7,
                run_left_first: 8,
                run_left_last: 15,
                run_down_first: 16,
                run_down_last: 23,
                run_right_first: 24,
                run_right_last: 31,
            },
            walk_animation_timer: MobWalkAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
            attack_animation_indices: MobAttackAnimationIndices {
                attack_up_first: 32,
                attack_up_last: 37,
                attack_left_first: 40,
                attack_left_last: 45,
                attack_down_first: 48,
                attack_down_last: 53,
                attack_right_first: 56,
                attack_right_last: 61,
            },
            attack_animation_timer: MobAttackAnimationTimer(Timer::from_seconds(0.2, TimerMode::Repeating)),
        });
    }
}
