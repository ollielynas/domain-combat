use macroquad::input::{is_key_down, is_key_pressed};
use std::hash::{Hash, Hasher};


use crate::input_source::input_device::{InputDevice, InputDeviceData, DirectionLeftRight, InputDirectionUpDown};




pub struct DummyInputDevice {
    device_data: InputDeviceData,
}

pub fn dummy_input() -> Box<dyn InputDevice> {
    return Box::new( DummyInputDevice {
        device_data: InputDeviceData { enabled: false, ready_to_play: false, score: 0 }
    });
}



impl InputDevice for DummyInputDevice {


    fn get_name(&mut self) -> String {
        "Dummy".to_string()
    }

    fn is_disconnected(&self) -> bool {
        true
    }

    fn get_input_device_data(&mut self) -> &mut InputDeviceData {
        return &mut self.device_data;
    }
    fn get_input_device_data_ref(&self) -> &InputDeviceData {
        return &self.device_data;
    }

    fn should_begin_jump(&mut self) -> bool {
        false
    }

    fn get_id(&self) -> u64 {
        0
    }











    fn get_current_direction_left_right(&self) -> super::input_device::DirectionLeftRight {

        return DirectionLeftRight::Neutral;
    }

    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown {

        return InputDirectionUpDown::Neutral;
    }

    fn should_begin_dash(&mut self) -> bool {
        false
    }

    fn should_begin_short_attack(&mut self) -> bool {
        false
    }

    fn should_begin_long_attack(&mut self) -> bool {
        false
    }

    fn is_jump_key_down(&mut self) -> bool {
        false
    }



    fn get_fast_attack_keybind(&mut self) -> String {
        "fast attack".to_string()
    }

    fn get_jump_keybind(&mut self) -> String {
        "jump".to_string()
    }

    fn get_left_keybind(&mut self) -> String {
        "left".to_string()
    }

    fn get_right_keybind(&mut self) -> String {
        "right".to_string()
    }

    fn get_up_keybind(&mut self) -> String {
        "up".to_string()
    }

    fn get_down_keybind(&mut self) -> String {
        "down".to_string()
    }

    fn get_strong_attack_keybind(&mut self) -> String {
        "strong".to_string()
    }

    fn should_begin_move_right(&mut self) -> bool {
        false
    }

    fn should_begin_move_left(&mut self) -> bool {
        false
    }
}
