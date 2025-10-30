use crate::{
    game::{
        player::Player,
        players::player_constructor::{self, PlayerConstructor},
    },
    input_source::{dummy_input_device::dummy_input, input_device::InputDevice}, scenes::scenes::Scene,
};

pub struct CharSelectState {
    input_devices: Vec<Box<dyn InputDevice>>,
    players: Vec<Box<dyn Player>>,
    player_constructor: Option<PlayerConstructor>,
}

impl CharSelectState {
    pub fn new(input_devices: Vec<Box<dyn InputDevice>>) -> CharSelectState {
        return CharSelectState {
            input_devices: input_devices,
            players: vec![],
            player_constructor: None,
        };
    }

    fn process_input(&mut self) {
        if self.player_constructor.is_none() {
            if self.input_devices.len() > 0 {
                self.player_constructor = Some(PlayerConstructor::new(
                    self.input_devices.pop().unwrap_or(dummy_input()),
                    self.players.len() + 1,
                ))
            }
        }


        if let Some(player_constructor) = &mut self.player_constructor {
            if let Some(new_player) = player_constructor.construct_player_if_ready() {
                self.players.push(new_player);
                self.player_constructor = None;
            }
        }

    }


    pub fn switch_scene(&mut self) -> Option<Scene> {
        if self.input_devices.len() == 0 {
            // return Scene::
        }
        return None;
    }

    pub fn render(&mut self) {
        self.process_input();

        if let Some(player_constructor) = &mut self.player_constructor {
            player_constructor.render();
        }
    }
}
