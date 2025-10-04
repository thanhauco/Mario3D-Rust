use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;
use crate::player::Player;
use crate::GameState;

pub struct CollectiblesPlugin;

impl Plugin for CollectiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coins)
            .add_systems(Update, (coin_rotation, collect_coins));
    }
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
    player_query: Query<&Transform, With<Player>>,
    coin_query: Query<(Entity, &Transform, &Coin)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (entity, coin_transform, coin) in coin_query.iter() {
            let distance = player_transform.translation.distance(coin_transform.translation);
            
            if distance < 1.0 {
                game_state.coins += coin.value;
                game_state.score += coin.value * 100;
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}
