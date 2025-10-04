use bevy::prelude::*;
use crate::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct CoinsText;

#[derive(Component)]
struct LivesText;

fn setup_ui(mut commands: Commands) {
    // UI Root
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                padding: UiRect::all(Val::Px(20.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            // Top left - Score and Coins
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Score
                    parent.spawn((
                        TextBundle::from_section(
                            "Score: 0",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::bottom(Val::Px(10.0)),
                            ..default()
                        }),
                        ScoreText,
                    ));

                    // Coins
                    parent.spawn((
                        TextBundle::from_section(
                            "Coins: 0",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::srgb(1.0, 0.84, 0.0),
                                ..default()
                            },
                        ),
                        CoinsText,
                    ));
                });

            // Top right - Lives
            parent.spawn((
                TextBundle::from_section(
                    "Lives: 3",
                    TextStyle {
                        font_size: 32.0,
                        color: Color::srgb(1.0, 0.3, 0.3),
                        ..default()
                    },
                ),
                LivesText,
            ));
        });

    // Controls hint at bottom
    commands.spawn(
        TextBundle::from_section(
            "Controls: WASD/Arrows to move | Space to jump",
            TextStyle {
                font_size: 20.0,
                color: Color::srgba(1.0, 1.0, 1.0, 0.7),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
    );
}

fn update_ui(
    game_state: Res<GameState>,
    mut score_query: Query<&mut Text, (With<ScoreText>, Without<CoinsText>, Without<LivesText>)>,
    mut coins_query: Query<&mut Text, (With<CoinsText>, Without<ScoreText>, Without<LivesText>)>,
    mut lives_query: Query<&mut Text, (With<LivesText>, Without<ScoreText>, Without<CoinsText>)>,
) {
    if game_state.is_changed() {
        if let Ok(mut text) = score_query.get_single_mut() {
            text.sections[0].value = format!("Score: {}", game_state.score);
        }

        if let Ok(mut text) = coins_query.get_single_mut() {
            text.sections[0].value = format!("Coins: {}", game_state.coins);
        }

        if let Ok(mut text) = lives_query.get_single_mut() {
            text.sections[0].value = format!("Lives: {}", game_state.lives);
        }
    }
}
