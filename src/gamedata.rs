use bevy::prelude::*;
use crate::gamestate;
use gamestate::GameState;

pub struct GameData {
    pub game_state: GameState,
    pub score: i32,
    pub move_left: bool,
    pub move_right: bool,
}

pub struct GameDataPlugin;

#[derive(Default)]
pub struct State {
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Vec2,
}

impl Plugin for GameDataPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GameData {
            game_state: GameState::Playing,
            score: 0,
            move_left: false,
            move_right: false,
        }).add_system(move_direction_system.system());
    }
}

fn move_direction_system(
    mut state: Local<State>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    window: Res<WindowDescriptor>,
    mut game_data: ResMut<GameData>,
) {
    if let Some(cursor_moved) = state.cursor_moved_event_reader.latest(&cursor_moved_events) {
        state.cursor_position = cursor_moved.position;
    }

    if mouse_button_input.pressed(MouseButton::Left) {
        if state.cursor_position.x() < window.width as f32 / 2.0 {
            game_data.move_left = true;
        } else {
            game_data.move_right = true;
        }
    } else if keyboard_input.pressed(KeyCode::A) {
        game_data.move_left = true;
    } else if keyboard_input.pressed(KeyCode::D) {
        game_data.move_right = true;
    } else {
        game_data.move_left = false;
        game_data.move_right = false;
    }
}
