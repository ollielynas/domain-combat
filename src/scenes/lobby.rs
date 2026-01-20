use std::mem::swap;

use crate::{
    game::{
        player::Player,
        players::player_constructor::{self, PlayerConstructor},
    },
    input_source::{dummy_input_device::dummy_input, input_device::InputDevice},
    scenes::{
        keybind_info::{self, KeyBindInfoState}, match_state::MatchState, scenes::Scene
    },
};

pub struct LobbyState {
    player_constructors: Vec<PlayerConstructor>,
    players: Vec<Box<dyn Player>>,
}

impl LobbyState {
    pub fn new(input_devices: Vec<Box<dyn InputDevice>>) -> LobbyState {

        return LobbyState {
            players: vec![],
            player_constructors: input_devices.into_iter().enumerate().map(|(i, x)| PlayerConstructor::new(x, i)).collect(),
        };
    }

    fn process_input(&mut self) {
        if self.player_constructors.iter().all(|x| (x).is_player_ready_to_be_constructed()) {
            let mut player_constructors_temp = vec![];
            swap(&mut player_constructors_temp, &mut self.player_constructors);
            self.players = player_constructors_temp.into_iter().map(|mut x| x.construct_player()).collect();
        }
    }
    // gets called every frame and returns if no input devices are left
    pub fn switch_scene(&mut self) -> Option<Scene> {

        if self.player_constructors.len() == 0 {
            let player_count = self.players.len();
            return Some(Scene::MatchScene(MatchState::start_match(self.players.drain(0..player_count).collect())));
        }

        return None;
    }

    pub fn render(&mut self) {
        self.process_input();
        let total = self.player_constructors.len();
        for (i, player_constructor) in self.player_constructors.iter_mut().enumerate() {
            player_constructor.render(i as usize, total);
        }
    }
}
