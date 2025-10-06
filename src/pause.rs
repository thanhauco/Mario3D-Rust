use bevy::prelude::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PauseState { paused: false })
            .add_systems(Update, (toggle_pause, update_pause_ui));
    }
}

#[derive(Resource)]
pub struct PauseState {
    pub paused: bool,
}

#[derive(Component)]
struct PauseMenu;

fn toggle_pause(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut pause_state: ResMut<PauseState>,
    mut time: ResMut<Time<Virtual>>,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        pause_state.paused = !pause_state.paused;
        
        if pause_state.paused {
            // Pause the game
            time.pause();
            
            // Spawn pause menu
            spawn_pause_menu(&mut commands);
        } else {
            // Resume the game
            time.unpause();
            
            // Despawn pause menu
            for entity in pause_menu_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}

fn spawn_pause_menu(commands: &mut Commands) {
    // Semi-transparent overlay
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    position_type: PositionType::Absolute,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            PauseMenu,
        ))
        .with_children(|parent| {
            // Pause menu container
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(40.0)),
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    background_color: Color::srgb(0.2, 0.2, 0.3).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // Title
                    parent.spawn(TextBundle::from_section(
                        "PAUSED",
                        TextStyle {
                            font_size: 60.0,
                            color: Color::WHITE,
                            ..default()
                        },
                    ));

                    // Instructions
                    parent.spawn(TextBundle::from_section(
                        "Press ESC to Resume",
                        TextStyle {
                            font_size: 24.0,
                            color: Color::srgb(0.8, 0.8, 0.8),
                            ..default()
                        },
                    ));

                    // Controls reminder
                    parent.spawn(
                        TextBundle::from_section(
                            "Controls:\nWASD/Arrows - Move\nSpace - Jump\nShift - Sprint",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::srgb(0.7, 0.7, 0.7),
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::top(Val::Px(20.0)),
                            ..default()
                        }),
                    );
                });
        });
}

fn update_pause_ui(
    pause_state: Res<PauseState>,
) {
    // Additional pause UI updates can go here
}
