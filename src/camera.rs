use bevy::prelude::*;
use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}

#[derive(Component)]
pub struct MainCamera {
    pub offset: Vec3,
    pub smoothness: f32,
}

impl Default for MainCamera {
    fn default() -> Self {
        Self {
            offset: Vec3::new(0.0, 8.0, 12.0),
            smoothness: 5.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    // Main camera with modern settings
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 8.0, 12.0)
                .looking_at(Vec3::new(0.0, 2.0, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true, // HDR rendering for modern graphics
                ..default()
            },
            ..default()
        },
        MainCamera::default(),
    ));

    // Directional light (sun)
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(
            EulerRot::XYZ,
            -std::f32::consts::FRAC_PI_4,
            std::f32::consts::FRAC_PI_4,
            0.0,
        )),
        ..default()
    });

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 300.0,
    });
}

fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (mut camera_transform, camera) in camera_query.iter_mut() {
            let target_position = player_transform.translation + camera.offset;
            
            // Smooth camera follow
            camera_transform.translation = camera_transform.translation.lerp(
                target_position,
                time.delta_seconds() * camera.smoothness,
            );

            // Look at player with slight offset
            let look_target = player_transform.translation + Vec3::new(0.0, 2.0, 0.0);
            let direction = (look_target - camera_transform.translation).normalize();
            camera_transform.look_to(direction, Vec3::Y);
        }
    }
}
