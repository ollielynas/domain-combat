use crate::{game::player::Player, input_source::input_device::InputDevice};




pub struct CharSelectState {
    input_devices: Vec<Box<dyn InputDevice>>,
    players: Vec<Box<dyn Player>>
}

impl CharSelectState {
    pub fn new(input_devices: Vec<Box<dyn InputDevice>>) -> CharSelectState {
        return CharSelectState { input_devices: input_devices, players: vec![] }
    }
}