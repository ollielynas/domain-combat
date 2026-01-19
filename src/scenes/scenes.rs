use crate::{
    game::game_state::MatchState,
    scenes::{
        char_select::CharSelectState, input_select::InputSelect, keybind_info::KeyBindInfoState,
    },
};

pub enum Scene {
    InputSelectScene(InputSelect),
    CharacterSelect(CharSelectState),
    MatchScene(MatchState),
    KeybindInfoScene(KeyBindInfoState),
}

impl Scene {
    pub fn render(&mut self) {
        match self {
            Scene::InputSelectScene(input_select) => {
                input_select.render();
            }
            Scene::CharacterSelect(char_select) => {
                char_select.render();
            }
            Scene::KeybindInfoScene(keyboard_info) => {
                keyboard_info.render();
            }
            Scene::MatchScene(match_state) => {}
        }
    }

    pub fn change_scene(&mut self) -> Option<Scene> {
        match self {
            Scene::InputSelectScene(input_select) => {
                return input_select.switch_scene();
            }
            Scene::CharacterSelect(char_select) => {
                return char_select.switch_scene();
            }
            Scene::MatchScene(match_state) => {
                return None;
            }
            Scene::KeybindInfoScene(keyboard_info) => {
                return None;
            }
        }
    }
}
