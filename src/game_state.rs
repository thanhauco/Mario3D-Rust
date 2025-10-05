use bevy::prelude::*;
use crate::player::Player;
use crate::GameState;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            check_death_zone,
            check_game_over,
            respawn_player,
        ));
    }
}

#[derive(Component)]
pub struct DeathZone;

#[derive(Component)]
pub struct RespawnTimer {
    pub timer: Timer,
}

const DEATH_Y: f32 = -10.0;
const SPAWN_POSITION: Vec3 = Vec3::new(0.0, 5.0, 0.0);

fn check_death_zone(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    player_query: Query<(Entity, &Transform), With<Player>>,
) {
    if let Ok((player_entity, transform)) = player_query.get_single() {
        // Check if player fell off the map
        if transform.translation.y < DEATH_Y {
            if game_state.lives > 0 {
                game_state.lives -= 1;
                
                // Add respawn timer
                commands.entity(player_entity).insert(RespawnTimer {
                    timer: Timer::from_seconds(2.0, TimerMode::Once),
                });
            }
        }
    }
}

fn respawn_player(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Transform, Option<&mut RespawnTimer>), With<Player>>,
) {
    if let Ok((entity, mut transform, respawn_timer)) = player_query.get_single_mut() {
        if let Some(mut timer) = respawn_timer {
            timer.timer.tick(time.delta());
            
            if timer.timer.finished() {
                // Respawn player at starting position
                transform.translation = SPAWN_POSITION;
                commands.entity(entity).remove::<RespawnTimer>();
            } else {
                // Move player below map during respawn
                transform.translation.y = DEATH_Y - 5.0;
            }
        }
    }
}

fn check_game_over(
    game_state: Res<GameState>,
) {
    if game_state.lives == 0 {
        // Game over - in full implementation, show game over screen
        info!("GAME OVER! Final Score: {}", game_state.score);
    }
}

// System to display game over UI (placeholder)
pub fn spawn_game_over_ui(
    mut commands: Commands,
    game_state: Res<GameState>,
) {
    if game_state.lives == 0 {
        commands.spawn(
            TextBundle::from_section(
                format!("GAME OVER\nFinal Score: {}\nPress R to Restart", game_state.score),
                TextStyle {
                    font_size: 60.0,
                    color: Color::srgb(1.0, 0.0, 0.0),
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Percent(40.0),
                left: Val::Percent(35.0),
                ..default()
            }),
        );
    }
}
