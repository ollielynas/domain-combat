use macroquad::{color::GRAY, text::draw_text, time::get_fps, window::screen_width};

use crate::consts::*;
use crate::scenes::{
        input_select::InputSelect, keybind_info::KeyBindInfoState, lobby::LobbyState, match_state::MatchState
    };
/// This holds all of the scenes in the game. The state of the game must be passed through the constructors of the scenes
/// each time the scene changes. You shuld not construct a scene unless you plan on emetly switching
/// the data that passed between the scenes should not be duplicated.
pub enum Scene {
    InputSelectScene(InputSelect),
    Lobby(LobbyState),
    KeybindInfoScene(KeyBindInfoState),
    MatchScene(MatchState),

}
///
impl Scene {
    // each scene should expose its own render function
    pub fn render(&mut self) {
        match self {
            Scene::InputSelectScene(input_select) => {
                input_select.render();
            }
            Scene::Lobby(char_select) => {
                char_select.render();
            }
            Scene::KeybindInfoScene(keyboard_info) => {
                keyboard_info.render();
            }
            Scene::MatchScene(match_state) => {
                match_state.render();
            },
        }
        draw_text(&format!("{} fps", get_fps()), screen_width() - 100.0, 30.0, 15.0, GRAY);
    }
    // each scene should have a funciton which will retun a new scene when a transition should occur
    pub fn change_scene(&mut self) -> Option<Scene> {
        match self {
            Scene::InputSelectScene(input_select) => {
                return input_select.switch_scene();
            }
            Scene::Lobby(char_select) => {
                return char_select.switch_scene();
            }

            Scene::KeybindInfoScene(keyboard_info) => {
                return keyboard_info.switch_scene();
            }
            Self::MatchScene(game_state) => {
                return None;
            }
        }
    }
}
