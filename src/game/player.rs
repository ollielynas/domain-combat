use crate::input_source::{
    dummy_input_device::dummy_input,
    input_device::{self, InputDevice},
};
use rapier2d::prelude::*;

pub struct UniversalPlayerData {
    pub input_device: Box<dyn InputDevice>,
    pub player_name: String,
}

impl UniversalPlayerData {
    pub fn new(input_device: Box<dyn InputDevice>, name: String) -> UniversalPlayerData {
        UniversalPlayerData {
            input_device,
            player_name: name,
        }
    }

    pub fn dummy() -> UniversalPlayerData {
        UniversalPlayerData::new(dummy_input(), "".to_string())
    }
}
pub trait Player {
    fn get_player_data(&mut self) -> &mut UniversalPlayerData;
    fn render_sprite_at_pos(&mut self, x: f32, y: f32, scale: f32);
    fn get_name(&mut self) -> String;
    fn get_input_device_ref(&mut self) -> &mut Box<dyn InputDevice> {
        return &mut self.get_player_data().input_device;
    }
}
