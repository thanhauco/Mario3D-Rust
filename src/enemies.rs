use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::Player;
use crate::GameState;
use rand::Rng;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (
                enemy_movement,
                enemy_collision_with_player,
                enemy_patrol,
                enemy_death_animation,
                update_combo_timer,
            ));
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32,
    pub patrol_direction: Vec3,
    pub damage: u32,
    pub is_dying: bool,
}

#[derive(Component)]
struct DeathAnimation {
    timer: Timer,
    initial_pos: Vec3,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 2.0,
            patrol_direction: Vec3::new(1.0, 0.0, 0.0),
            damage: 1,
            is_dying: false,
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<(Entity, &Transform, &Velocity), With<Player>>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<DeathAnimation>>,
) {
    if let Ok((player_entity, player_transform, player_velocity)) = player_query.get_single() {
        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            if enemy.is_dying {
                continue;
            }
            
            let distance = player_transform.translation.distance(enemy_transform.translation);
            
            if distance < 1.0 {
                let height_diff = player_transform.translation.y - enemy_transform.translation.y;
                
                // Check if player is jumping on enemy (from above and moving downward)
                if height_diff > 0.3 && player_velocity.linvel.y < 0.0 {
                    // Player defeats enemy by stomping
                    // Combo system: increase combo and apply multiplier
                    game_state.combo += 1;
                    game_state.combo_timer = 3.0; // 3 seconds to get next combo
                    let combo_multiplier = game_state.combo.min(10); // Max 10x
                    let score_gain = 200 * combo_multiplier;
                    game_state.score += score_gain;
                    
                    enemy.is_dying = true;
                    
                    // Add death animation component
                    commands.entity(enemy_entity).insert(DeathAnimation {
                        timer: Timer::from_seconds(0.5, TimerMode::Once),
                        initial_pos: enemy_transform.translation,
                    });
                    
                    // Spawn defeat particles
                    spawn_enemy_defeat_particles(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        enemy_transform.translation,
                    );
                } else if height_diff <= 0.3 {
                    // Enemy hits player from side
                    if game_state.lives > 0 {
                        game_state.lives -= enemy.damage;
                        // Reset combo on taking damage
                        game_state.combo = 0;
                        game_state.combo_timer = 0.0;
                    }
                }
            }
        }
    }
}

fn spawn_enemy_defeat_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    let particle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.1),
        emissive: Color::srgb(1.0, 0.3, 0.1).into(),
        ..default()
    });

    for _ in 0..6 {
        let velocity = Vec3::new(
            rng.gen_range(-3.0..3.0),
            rng.gen_range(3.0..6.0),
            rng.gen_range(-3.0..3.0),
        );

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.15)),
                material: particle_material.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            EnemyParticle {
                lifetime: Timer::from_seconds(1.0, TimerMode::Once),
                velocity,
            },
        ));
    }
}

#[derive(Component)]
struct EnemyParticle {
    lifetime: Timer,
    velocity: Vec3,
}

fn enemy_death_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut DeathAnimation)>,
    mut particle_query: Query<(Entity, &mut Transform, &mut EnemyParticle), Without<DeathAnimation>>,
) {
    // Handle enemy death animation
    for (entity, mut transform, mut death_anim) in enemy_query.iter_mut() {
        death_anim.timer.tick(time.delta());
        
        if death_anim.timer.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            // Squash and fade animation
            let progress = death_anim.timer.fraction();
            transform.scale = Vec3::new(1.0 + progress, 1.0 - progress * 0.8, 1.0 + progress);
            transform.translation.y = death_anim.initial_pos.y - progress * 0.5;
        }
    }
    
    // Handle defeat particles
    for (entity, mut transform, mut particle) in particle_query.iter_mut() {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            transform.translation += particle.velocity * time.delta_seconds();
            particle.velocity.y -= 9.8 * time.delta_seconds();
            
            let alpha = 1.0 - particle.lifetime.fraction();
            transform.scale = Vec3::splat(alpha);
        }
    }
}

fn update_combo_timer(
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
) {
    if game_state.combo_timer > 0.0 {
        game_state.combo_timer -= time.delta_seconds();
        
        if game_state.combo_timer <= 0.0 {
            // Combo expired
            game_state.combo = 0;
            game_state.combo_timer = 0.0;
        }
    }
}
