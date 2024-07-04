use bevy::{render::texture::Image, prelude::*};
use bevy_xpbd_2d::prelude::*;


#[derive(Resource)]
pub struct PlayerAssets {
    pub naked: Handle<Image>,
    pub naked_with_sword: Handle<Image>,
    pub general_layout: Handle<TextureAtlasLayout>,
    pub general_animation_indices: GeneralAnimationIndices,
    pub attack_layout: Handle<TextureAtlasLayout>,
    pub attack_animation_indices: AttackAnimationIndices,
    pub walk_animation_timer: Timer,
    pub attack_animation_timer: Timer,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct GeneralAnimationIndices {
    pub run_up_first: usize,
    pub run_up_last: usize,
    pub run_down_first: usize,
    pub run_down_last: usize,
    pub run_left_first: usize,
    pub run_left_last: usize,
    pub run_right_first: usize,
    pub run_right_last: usize,
}

pub struct AttackAnimationIndices {
    pub attack_up_first: usize,
    pub attack_up_last: usize,
    pub attack_down_first: usize,
    pub attack_down_last: usize,
    pub attack_left_first: usize,
    pub attack_left_last: usize,
    pub attack_right_first: usize,
    pub attack_right_last: usize,
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let naked_texture = asset_server.load("player/naked.png");
    let naked_with_sword_texture = asset_server.load("player/naked_with_sword.png"); 
    let general_layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 13, 46, None, None);
    let attack_layout = TextureAtlasLayout::from_grid(Vec2::new(192.0, 192.0), 8, 12, None, None);
    let attack_layout = texture_atlas_layouts.add(attack_layout);
    let general_layout = texture_atlas_layouts.add(general_layout);
    let attack_animation_indices = AttackAnimationIndices {
        attack_up_first: 0,
        attack_up_last: 7,
        attack_left_first: 8,
        attack_left_last: 15,
        attack_down_first: 16,
        attack_down_last: 23,
        attack_right_first: 24,
        attack_right_last: 31,
    };
    commands.insert_resource(PlayerAssets {
        naked: naked_texture.clone(),
        naked_with_sword: naked_with_sword_texture,
        general_layout: general_layout.clone(),
        general_animation_indices: GeneralAnimationIndices {
            run_up_first: 105,
            run_up_last: 112,
            run_left_first: 117,
            run_left_last: 125,
            run_down_first: 131,
            run_down_last: 138,
            run_right_first: 143,
            run_right_last: 151,
        },
        attack_layout: attack_layout.clone(),
        attack_animation_indices,
        walk_animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
        attack_animation_timer: Timer::from_seconds(0.1, TimerMode::Repeating),
    });
    commands.spawn((Camera2dBundle::default(), Camera));
    commands.spawn((
        RigidBody::Kinematic,
        Friction::new(0.0),
        Restitution::new(1.0),
        GravityScale(0.0),
        Rotation::from_degrees(0.0),
        Collider::rectangle(32.0, 52.0),
        LinearVelocity(Vec2::new(0.0, 0.0)),
        SpriteSheetBundle {
            texture: naked_texture,
            atlas: TextureAtlas {
                layout: general_layout,
                index: 130,
            },
            ..default()
        },
        Player,
    ));
}
