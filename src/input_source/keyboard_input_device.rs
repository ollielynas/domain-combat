use macroquad::input::{KeyCode, is_key_down, is_key_pressed};
use std::hash::{Hash, Hasher};


use crate::input_source::input_device::{InputDevice, InputDeviceData, DirectionLeftRight, InputDirectionUpDown};




pub struct WasdKeyboardInputDevice {
    device_data: InputDeviceData,
}

pub fn get_all_wasd_keyboards() -> Vec<Box<dyn InputDevice>> {
    return vec![
        Box::new(WasdKeyboardInputDevice {
            device_data: InputDeviceData::default(),
        })
    ]
}



impl InputDevice for WasdKeyboardInputDevice {


    fn get_name(&mut self) -> String {
        "WASD Keyboard".to_string()
    }

    fn get_input_device_data(&mut self) -> &mut InputDeviceData {
        return &mut self.device_data;
    }
    fn get_input_device_data_ref(&self) -> &InputDeviceData {
        return &self.device_data;
    }

    fn should_begin_jump(&mut self) -> bool {
        return is_key_pressed(KeyCode::Space);
    }
    fn is_jump_key_down(&mut self) -> bool {
        return is_key_down(KeyCode::Space)
    }

    fn get_id(&self) -> u64 {
        return 0;
    }



    fn get_current_direction_left_right(&self) -> super::input_device::DirectionLeftRight {

        if is_key_down(KeyCode::A) {
            return DirectionLeftRight::Left;
        }
        if is_key_down(KeyCode::D) {
            return DirectionLeftRight::Right;
        }

        return DirectionLeftRight::Neutral;
    }

    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown {

        if is_key_down(KeyCode::W) {
            return InputDirectionUpDown::Up;
        }
        if is_key_down(KeyCode::S) {
            return InputDirectionUpDown::Down;
        }

        return InputDirectionUpDown::Neutral;
    }

    fn should_begin_dash(&mut self) -> bool {
        return is_key_pressed(KeyCode::Y);
    }

    fn should_begin_short_attack(&mut self) -> bool {
        return is_key_pressed(KeyCode::R);
    }

    fn should_begin_long_attack(&mut self) -> bool {
        return is_key_pressed(KeyCode::T);
    }



    fn get_fast_attack_keybind(&mut self) -> String {
        "r".to_string()
    }

    fn get_jump_keybind(&mut self) -> String {
        "spacebar".to_string()
    }

    fn get_left_keybind(&mut self) -> String {
        "a".to_string()
    }

    fn get_right_keybind(&mut self) -> String {
        "d".to_string()
    }

    fn get_up_keybind(&mut self) -> String {
        "w".to_string()
    }

    fn get_down_keybind(&mut self) -> String {
        "s".to_string()
    }

    fn get_strong_attack_keybind(&mut self) -> String {
        "t".to_string()
    }

    fn should_begin_move_right(&mut self) -> bool {
        is_key_pressed(KeyCode::D)
    }

    fn should_begin_move_left(&mut self) -> bool {
        is_key_pressed(KeyCode::A)
    }
}
