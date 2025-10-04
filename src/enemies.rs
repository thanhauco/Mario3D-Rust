use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::Player;
use crate::GameState;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                enemy_collision_with_player,
                enemy_patrol,
            ));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub patrol_direction: Vec3,
    pub damage: u32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 2.0,
            patrol_direction: Vec3::new(1.0, 0.0, 0.0),
            damage: 1,
        }
    }
}

#[derive(Component)]
struct PatrolPoint {
    start: Vec3,
    end: Vec3,
}

fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let enemy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.1), // Red enemy
        metallic: 0.2,
        perceptual_roughness: 0.7,
        ..default()
    });

    // Spawn enemies at various patrol routes
    let enemy_spawns = vec![
        (Vec3::new(-5.0, 1.0, 0.0), Vec3::new(5.0, 1.0, 0.0)),
        (Vec3::new(8.0, 1.0, -10.0), Vec3::new(15.0, 1.0, -10.0)),
        (Vec3::new(-10.0, 1.0, -5.0), Vec3::new(-10.0, 1.0, 5.0)),
        (Vec3::new(0.0, 1.0, -15.0), Vec3::new(10.0, 1.0, -15.0)),
    ];

    for (start, end) in enemy_spawns {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.3, 0.8)),
                material: enemy_material.clone(),
                transform: Transform::from_translation(start),
                ..default()
            },
            Enemy::default(),
            PatrolPoint { start, end },
            RigidBody::KinematicPositionBased,
            Collider::capsule_y(0.4, 0.3),
            Sensor,
            Name::new("Enemy"),
        ))
        .with_children(|parent| {
            // Enemy eyes (white spheres)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: Color::srgb(1.0, 1.0, 1.0).into(),
                    ..default()
                }),
                transform: Transform::from_xyz(0.15, 0.3, 0.25),
                ..default()
            });
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: Color::srgb(1.0, 1.0, 1.0).into(),
                    ..default()
                }),
                transform: Transform::from_xyz(-0.15, 0.3, 0.25),
                ..default()
            });
        });
    }
}

fn enemy_patrol(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Enemy, &PatrolPoint)>,
) {
    for (mut transform, mut enemy, patrol) in query.iter_mut() {
        let direction = (patrol.end - patrol.start).normalize();
        let distance_to_end = transform.translation.distance(patrol.end);
        let distance_to_start = transform.translation.distance(patrol.start);

        // Switch direction when reaching patrol points
        if distance_to_end < 0.5 {
            enemy.patrol_direction = -direction;
        } else if distance_to_start < 0.5 {
            enemy.patrol_direction = direction;
        }

        // Move enemy
        transform.translation += enemy.patrol_direction * enemy.speed * time.delta_seconds();

        // Face movement direction
        if enemy.patrol_direction.length() > 0.01 {
            let look_direction = enemy.patrol_direction;
            transform.look_to(look_direction, Vec3::Y);
        }
    }
}

fn enemy_movement(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Enemy>>,
) {
    for mut transform in query.iter_mut() {
        // Add slight bobbing animation
        let bob = (time.elapsed_seconds() * 4.0 + transform.translation.x).sin() * 0.02;
        transform.translation.y += bob * time.delta_seconds();
    }
}

fn enemy_collision_with_player(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<(&Transform, &Enemy)>,
    time: Res<Time>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for (enemy_transform, enemy) in enemy_query.iter() {
            let distance = player_transform.translation.distance(enemy_transform.translation);
            
            if distance < 1.0 {
                // Check if player is jumping on enemy (from above)
                if player_transform.translation.y > enemy_transform.translation.y + 0.5 {
                    // Player defeats enemy by jumping on it
                    game_state.score += 200;
                    // In a full implementation, we'd despawn the enemy here
                } else {
                    // Enemy hits player
                    if game_state.lives > 0 {
                        game_state.lives -= enemy.damage;
                        // In a full implementation, add invincibility frames and knockback
                    }
                }
            }
        }
    }
}
