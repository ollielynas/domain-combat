use crate::{game::game_state::MatchState, scenes::{char_select::{self, CharSelectState}, input_select::InputSelect}};



pub enum Scene {
    InputSelectScene(InputSelect),
    CharacterSelect(CharSelectState),
    MatchScene(MatchState),
}


impl Scene {
    pub fn render(&mut self) {
        match self {
            Scene::InputSelectScene(input_select) => {
                input_select.render();
            },
            Scene::CharacterSelect(char_select) => {
                char_select.render();
            },
            Scene::MatchScene(match_state) => {

            },
        }
    }

    pub fn change_scene(&mut self) -> Option<Scene> {
        match self {
            Scene::InputSelectScene(input_select) => {
                return input_select.switch_scene();
            },
            Scene::CharacterSelect(char_select) => {
                return None;
            },
            Scene::MatchScene(match_state) => {
                return None;
            },
        }
    }
}