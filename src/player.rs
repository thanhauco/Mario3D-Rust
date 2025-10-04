use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (
                player_movement,
                player_jump,
                player_animation,
            ));
    }
}

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub is_grounded: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 8.0,
            jump_force: 12.0,
            is_grounded: false,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Mario character (red capsule for now, can be replaced with 3D model)
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Capsule3d::new(0.4, 1.2)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.9, 0.1, 0.1), // Mario red
                metallic: 0.1,
                perceptual_roughness: 0.8,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Player::default(),
        RigidBody::Dynamic,
        Collider::capsule_y(0.6, 0.4),
        Velocity::default(),
        GravityScale(2.0),
        LockedAxes::ROTATION_LOCKED,
        Friction {
            coefficient: 0.7,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Name::new("Player"),
    ))
    .with_children(|parent| {
        // Mario's cap (blue sphere on top)
        parent.spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.35)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.1, 0.1, 0.9), // Blue cap
                metallic: 0.1,
                perceptual_roughness: 0.7,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.8, 0.0),
            ..default()
        });
    });
}

fn player_movement(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &Player, &Transform)>,
) {
    for (mut velocity, player, transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            direction.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            direction.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            velocity.linvel.x = direction.x * player.speed;
            velocity.linvel.z = direction.z * player.speed;
        } else {
            // Apply friction when not moving
            velocity.linvel.x *= 0.8;
            velocity.linvel.z *= 0.8;
        }
    }
}

fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &Player)>,
    rapier_context: Res<RapierContext>,
    player_query: Query<Entity, With<Player>>,
) {
    for (mut velocity, player) in query.iter_mut() {
        // Check if grounded using raycast
        if let Ok(player_entity) = player_query.get_single() {
            let ray_pos = Vec3::new(0.0, -0.7, 0.0);
            let ray_dir = Vec3::new(0.0, -1.0, 0.0);
            let max_toi = 0.2;
            let solid = true;
            let filter = QueryFilter::default().exclude_rigid_body(player_entity);

            if rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter).is_some() {
                // Player is grounded
                if keyboard.just_pressed(KeyCode::Space) {
                    velocity.linvel.y = player.jump_force;
                }
            }
        }
    }
}

fn player_animation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        // Simple bobbing animation when moving
        let bob = (time.elapsed_seconds() * 8.0).sin() * 0.05;
        // This would be enhanced with actual animations
    }
}
