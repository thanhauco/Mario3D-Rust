use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::player::Player;
use crate::GameState;

pub struct PowerUpsPlugin;

impl Plugin for PowerUpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_powerups)
            .add_systems(Update, (
                powerup_animation,
                collect_powerups,
            ));
    }
}

#[derive(Component)]
pub enum PowerUpType {
    Mushroom,
    FireFlower,
    Star,
}

#[derive(Component)]
pub struct PowerUp {
    pub powerup_type: PowerUpType,
}

fn spawn_powerups(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Mushroom material (red with white spots)
    let mushroom_cap_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.1, 0.1), // Red
        metallic: 0.2,
        perceptual_roughness: 0.6,
        ..default()
    });

    let mushroom_stem_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.95, 0.95, 0.85), // Cream white
        metallic: 0.1,
        perceptual_roughness: 0.7,
        ..default()
    });

    let spot_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        metallic: 0.1,
        perceptual_roughness: 0.5,
        ..default()
    });

    // Spawn mushrooms at various locations
    for _ in 0..8 {
        let x = rng.gen_range(-18.0..18.0);
        let y = 1.5;
        let z = rng.gen_range(-20.0..0.0);

        commands.spawn((
            SpatialBundle {
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            PowerUp {
                powerup_type: PowerUpType::Mushroom,
            },
            Sensor,
            Collider::cylinder(0.3, 0.4),
            Name::new("Mushroom"),
        ))
        .with_children(|parent| {
            // Mushroom cap (hemisphere)
            parent.spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(0.4)),
                material: mushroom_cap_material.clone(),
                transform: Transform::from_xyz(0.0, 0.3, 0.0)
                    .with_scale(Vec3::new(1.0, 0.6, 1.0)),
                ..default()
            });

            // Mushroom stem
            parent.spawn(PbrBundle {
                mesh: meshes.add(Cylinder::new(0.25, 0.4)),
                material: mushroom_stem_material.clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            });

            // White spots on cap
            let spot_positions = vec![
                Vec3::new(0.2, 0.4, 0.2),
                Vec3::new(-0.2, 0.4, 0.2),
                Vec3::new(0.0, 0.5, -0.2),
                Vec3::new(0.15, 0.45, -0.15),
            ];

            for pos in spot_positions {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(Sphere::new(0.08)),
                    material: spot_material.clone(),
                    transform: Transform::from_translation(pos),
                    ..default()
                });
            }
        });
    }

    // Fire Flower material (orange and yellow)
    let flower_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.5, 0.0), // Orange
        metallic: 0.3,
        perceptual_roughness: 0.4,
        emissive: Color::srgb(0.5, 0.2, 0.0).into(),
        ..default()
    });

    // Spawn a few fire flowers
    for _ in 0..3 {
        let x = rng.gen_range(-15.0..15.0);
        let y = 2.0;
        let z = rng.gen_range(-18.0..-5.0);

        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere::new(0.3)),
                material: flower_material.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            PowerUp {
                powerup_type: PowerUpType::FireFlower,
            },
            Sensor,
            Collider::ball(0.3),
            Name::new("FireFlower"),
        ));
    }
}

fn powerup_animation(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<PowerUp>>,
) {
    for mut transform in query.iter_mut() {
        // Bobbing animation
        let bob = (time.elapsed_seconds() * 2.0 + transform.translation.x).sin() * 0.15;
        transform.translation.y += bob * time.delta_seconds();
        
        // Slow rotation
        transform.rotate_y(time.delta_seconds() * 0.5);
    }
}

fn collect_powerups(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<&Transform, With<Player>>,
    powerup_query: Query<(Entity, &Transform, &PowerUp)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, powerup_transform, powerup) in powerup_query.iter() {
            let distance = player_transform.translation.distance(powerup_transform.translation);
            
            if distance < 1.2 {
                match powerup.powerup_type {
                    PowerUpType::Mushroom => {
                        game_state.score += 1000;
                        // In full implementation: grow player, add health
                    }
                    PowerUpType::FireFlower => {
                        game_state.score += 1500;
                        // In full implementation: enable fire shooting
                    }
                    PowerUpType::Star => {
                        game_state.score += 2000;
                        // In full implementation: invincibility mode
                    }
                }
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
