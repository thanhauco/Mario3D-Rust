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
    pub sprint_speed: f32,
    pub jump_force: f32,
    pub wall_jump_force: f32,
    pub is_grounded: bool,
    pub is_sprinting: bool,
    pub has_double_jump: bool,
    pub wall_normal: Option<Vec3>,
    pub wall_jump_cooldown: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 8.0,
            sprint_speed: 14.0,
            jump_force: 12.0,
            wall_jump_force: 10.0,
            is_grounded: false,
            is_sprinting: false,
            has_double_jump: true,
            wall_normal: None,
            wall_jump_cooldown: 0.0,
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
    mut query: Query<(&mut Velocity, &mut Player, &Transform)>,
) {
    for (mut velocity, mut player, transform) in query.iter_mut() {
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

        // Check for sprint (Shift key)
        player.is_sprinting = keyboard.pressed(KeyCode::ShiftLeft) || keyboard.pressed(KeyCode::ShiftRight);
        let current_speed = if player.is_sprinting {
            player.sprint_speed
        } else {
            player.speed
        };

        if direction.length() > 0.0 {
            direction = direction.normalize();
            velocity.linvel.x = direction.x * current_speed;
            velocity.linvel.z = direction.z * current_speed;
        } else {
            // Apply friction when not moving
            velocity.linvel.x *= 0.8;
            velocity.linvel.z *= 0.8;
        }
    }
}
}

fn player_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Velocity, &mut Player, &Transform)>,
    rapier_context: Res<RapierContext>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    for (player_entity, mut velocity, mut player, transform) in query.iter_mut() {
        let mut is_grounded = false;
        let mut wall_normal = None;
        
        // Update wall jump cooldown
        if player.wall_jump_cooldown > 0.0 {
            player.wall_jump_cooldown -= time.delta_seconds();
        }

        // Check if player is on the ground or against a wall
        let ray_origin = transform.translation;
        
        // Ground check (down)
        if let Some((_entity, toi)) = rapier_context.cast_ray(
            ray_origin,
            Vec3::NEG_Y,
            1.1,
            true,
            QueryFilter::exclude_rigid_body(player_entity)
        ) {
            if toi < 1.1 {
                is_grounded = true;
                player.has_double_jump = true; // Reset double jump when grounded
            }
        }

        // Wall check (in movement direction)
        if !is_grounded && player.wall_jump_cooldown <= 0.0 {
            let move_dir = Vec3::new(
                velocity.linvel.x.signum(),
                0.0,
                velocity.linvel.z.signum()
            ).normalize_or_zero();
            
            if move_dir != Vec3::ZERO {
                if let Some((_entity, hit)) = rapier_context.cast_ray(
                    ray_origin,
                    move_dir,
                    1.1,
                    true,
                    QueryFilter::exclude_rigid_body(player_entity)
                ) {
                    wall_normal = Some(hit.normal);
                }
            }
        }

        player.wall_normal = wall_normal;
        let was_grounded = player.is_grounded;
        player.is_grounded = is_grounded;

        // Handle jumping
        if keyboard.just_pressed(KeyCode::Space) {
            if is_grounded {
                // Regular jump
                velocity.linvel.y = player.jump_force;
                spawn_jump_effect(&mut commands, &mut meshes, &mut materials, transform.translation);
            } else if player.has_double_jump {
                // Double jump
                velocity.linvel.y = player.jump_force * 0.9; // Slightly weaker than first jump
                player.has_double_jump = false;
                spawn_double_jump_effect(&mut commands, &mut meshes, &mut materials, transform.translation);
            } else if let Some(normal) = player.wall_normal.filter(|_| player.wall_jump_cooldown <= 0.0) {
                // Wall jump
                let wall_jump_dir = (Vec3::Y + normal * 1.5).normalize();
                velocity.linvel = wall_jump_dir * player.wall_jump_force;
                player.wall_jump_cooldown = 0.3; // Small cooldown to prevent wall jump spam
                spawn_wall_jump_effect(&mut commands, &mut meshes, &mut materials, transform.translation, normal);
            }
        }
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in query.iter_mut() {
        // Simple bobbing animation when moving
        let bob = (time.elapsed_seconds() * 8.0).sin() * 0.05;
        // This would be enhanced with actual animations
    }
}
