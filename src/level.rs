use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use rand::Rng;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ground, spawn_platforms, spawn_obstacles));
    }
}

fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Main ground plane with grass-like appearance
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.3, 0.7, 0.3), // Grass green
                perceptual_roughness: 0.9,
                metallic: 0.0,
                ..default()
            }),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Collider::cuboid(50.0, 0.1, 50.0),
        RigidBody::Fixed,
        Friction {
            coefficient: 0.7,
            combine_rule: CoefficientCombineRule::Average,
        },
        Name::new("Ground"),
    ));
}

fn spawn_platforms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let platform_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.6, 0.2), // Golden/brick color
        perceptual_roughness: 0.7,
        metallic: 0.1,
        ..default()
    });

    let platform_mesh = meshes.add(Cuboid::new(3.0, 0.5, 3.0));

    // Create a series of platforms
    let platforms = vec![
        (Vec3::new(5.0, 1.0, -5.0), Vec3::new(4.0, 0.5, 4.0)),
        (Vec3::new(10.0, 2.5, -8.0), Vec3::new(3.0, 0.5, 3.0)),
        (Vec3::new(15.0, 4.0, -5.0), Vec3::new(3.5, 0.5, 3.5)),
        (Vec3::new(8.0, 3.0, 0.0), Vec3::new(3.0, 0.5, 3.0)),
        (Vec3::new(-5.0, 1.5, -8.0), Vec3::new(4.0, 0.5, 4.0)),
        (Vec3::new(-10.0, 3.0, -10.0), Vec3::new(3.0, 0.5, 3.0)),
        (Vec3::new(0.0, 5.0, -15.0), Vec3::new(5.0, 0.5, 5.0)),
        (Vec3::new(12.0, 6.0, -15.0), Vec3::new(3.0, 0.5, 3.0)),
    ];

    for (position, size) in platforms {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cuboid::new(size.x, size.y, size.z)),
                material: platform_material.clone(),
                transform: Transform::from_translation(position),
                ..default()
            },
            Collider::cuboid(size.x / 2.0, size.y / 2.0, size.z / 2.0),
            RigidBody::Fixed,
            Friction {
                coefficient: 0.7,
                combine_rule: CoefficientCombineRule::Average,
            },
            Name::new("Platform"),
        ));
    }
}

fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut rng = rand::thread_rng();

    // Question blocks (yellow boxes)
    let question_block_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.0), // Yellow
        perceptual_roughness: 0.5,
        metallic: 0.2,
        emissive: Color::srgb(0.3, 0.2, 0.0).into(),
        ..default()
    });

    let block_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // Spawn question blocks at various locations
    for i in 0..10 {
        let x = rng.gen_range(-15.0..15.0);
        let y = rng.gen_range(2.0..6.0);
        let z = rng.gen_range(-20.0..-5.0);

        commands.spawn((
            PbrBundle {
                mesh: block_mesh.clone(),
                material: question_block_material.clone(),
                transform: Transform::from_xyz(x, y, z),
                ..default()
            },
            Collider::cuboid(0.5, 0.5, 0.5),
            RigidBody::Fixed,
            QuestionBlock { hit: false },
            Name::new("QuestionBlock"),
        ));
    }

    // Pipes (green cylinders)
    let pipe_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.8, 0.2), // Green
        perceptual_roughness: 0.6,
        metallic: 0.1,
        ..default()
    });

    let pipe_positions = vec![
        Vec3::new(-8.0, 1.0, 5.0),
        Vec3::new(6.0, 1.0, 8.0),
        Vec3::new(-15.0, 1.0, -3.0),
    ];

    for pos in pipe_positions {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Cylinder::new(0.8, 2.0)),
                material: pipe_material.clone(),
                transform: Transform::from_translation(pos),
                ..default()
            },
            Collider::cylinder(1.0, 0.8),
            RigidBody::Fixed,
            Name::new("Pipe"),
        ));
    }
}

#[derive(Component)]
pub struct QuestionBlock {
    pub hit: bool,
}
