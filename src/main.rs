use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod player;
mod camera;
mod level;
mod collectibles;
mod enemies;
mod powerups;
mod ui;

use player::PlayerPlugin;
use camera::CameraPlugin;
use level::LevelPlugin;
use collectibles::CollectiblesPlugin;
use enemies::EnemiesPlugin;
use powerups::PowerUpsPlugin;
use ui::UIPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mario 3D - Rust Edition".to_string(),
                resolution: (1920., 1080.).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LevelPlugin)
        .add_plugins(CollectiblesPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(PowerUpsPlugin)
        .add_plugins(UIPlugin)
        .insert_resource(ClearColor(Color::srgb(0.53, 0.81, 0.92))) // Sky blue
        .insert_resource(GameState::default())
        .run();
}

#[derive(Resource)]
pub struct GameState {
    pub score: u32,
    pub coins: u32,
    pub lives: u32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            coins: 0,
            lives: 3,
        }
    }
}
