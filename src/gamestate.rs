use crate::gamedata;
use bevy::prelude::*;
use gamedata::GameData;

#[derive(std::cmp::PartialEq)]
pub enum GameState {
    Menu,
    Playing,
    Dead,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(handle_gamestate_system.system());
    }
}

fn handle_gamestate_system(mut game_data: ResMut<GameData>, keyboard_input: Res<Input<KeyCode>>) {
    match game_data.game_state {
        GameState::Menu => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
                println!("Playing");
            }
        }
        GameState::Playing => {}
        GameState::Dead => {
            if keyboard_input.just_pressed(KeyCode::Space) {
                game_data.game_state = GameState::Playing;
                println!("Playing");
            }
        }
    }
}
