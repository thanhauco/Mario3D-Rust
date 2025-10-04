use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::player::Player;
use crate::GameState;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coins)
            .add_systems(Update, (
                coin_rotation,
                collect_coins,
                particle_animation,
            ));
    }
}

#[derive(Component)]
struct CoinParticle {
    lifetime: Timer,
    velocity: Vec3,
}

#[derive(Component)]
pub struct Coin {
    pub value: u32,
}

fn spawn_coins(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    let coin_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.84, 0.0), // Gold color
        metallic: 0.9,
        perceptual_roughness: 0.2,
        emissive: Color::srgb(0.5, 0.42, 0.0).into(),
        ..default()
    });

    // Spawn coins at various locations
    for _ in 0..30 {
        let x = rng.gen_range(-20.0..20.0);
        let y = rng.gen_range(1.0..8.0);
        let z = rng.gen_range(-25.0..5.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(0.3, 0.1)),
                material: coin_material.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            Coin { value: 1 },
            Sensor,
            Collider::cylinder(0.05, 0.3),
            Name::new("Coin"),
        ));
    }
}

fn coin_rotation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Coin>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotate_y(time.delta_seconds() * 2.0);
        // Bobbing animation
        let bob = (time.elapsed_seconds() * 3.0 + transform.translation.x).sin() * 0.1;
        transform.translation.y += bob * time.delta_seconds();
    }
}

fn collect_coins(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform, &Coin)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, coin_transform, coin) in coin_query.iter() {
            let distance = player_transform.translation.distance(coin_transform.translation);
            
            if distance < 1.0 {
                game_state.coins += coin.value;
                game_state.score += coin.value * 100;
                
                // Spawn particle effects when collecting coin
                spawn_coin_particles(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    coin_transform.translation,
                );
                
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spawn_coin_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    let mut rng = rand::thread_rng();
    
    let particle_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.2),
        metallic: 0.8,
        perceptual_roughness: 0.3,
        emissive: Color::srgb(2.0, 1.8, 0.5).into(), // Bright glow
        ..default()
    });

    // Spawn multiple particles in different directions
    for _ in 0..8 {
        let velocity = Vec3::new(
            rng.gen_range(-2.0..2.0),
            rng.gen_range(2.0..4.0),
            rng.gen_range(-2.0..2.0),
        );

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.1)),
                material: particle_material.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            CoinParticle {
                lifetime: Timer::from_seconds(0.5, TimerMode::Once),
                velocity,
            },
        ));
    }
}

fn particle_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut CoinParticle)>,
) {
    for (entity, mut transform, mut particle) in query.iter_mut() {
        particle.lifetime.tick(time.delta());
        
        if particle.lifetime.finished() {
            commands.entity(entity).despawn_recursive();
        } else {
            // Apply velocity and gravity
            transform.translation += particle.velocity * time.delta_seconds();
            particle.velocity.y -= 9.8 * time.delta_seconds(); // Gravity
            
            // Fade out based on lifetime
            let alpha = 1.0 - particle.lifetime.fraction();
            transform.scale = Vec3::splat(alpha);
        }
    }
}
