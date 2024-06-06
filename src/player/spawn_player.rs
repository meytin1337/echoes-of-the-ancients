use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Camera;

#[derive(Component)]
pub struct AnimationIndices {
    pub run_up_first: usize,
    pub run_up_last: usize,
    pub run_down_first: usize,
    pub run_down_last: usize,
    pub run_left_first: usize,
    pub run_left_last: usize,
    pub run_right_first: usize,
    pub run_right_last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("player/naked.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(64.0, 64.0), 13, 46, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let animation_config_1 = AnimationConfig::new(1, 6, 10);
    let animation_indices = AnimationIndices {
        run_up_first: 105,
        run_up_last: 112,
        run_left_first: 117,
        run_left_last: 125,
        run_down_first: 131,
        run_down_last: 138,
        run_right_first: 143,
        run_right_last: 151,
    };
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
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 130,
            },
            ..default()
        },
        Player,
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}
