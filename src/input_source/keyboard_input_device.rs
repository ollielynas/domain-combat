use macroquad::input::{is_key_down, is_key_pressed};
use std::hash::{Hash, Hasher};


use crate::input_source::input_device::{InputDevice, InputDeviceData, InputDirectionLeftRight, InputDirectionUpDown};




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

    fn should_begin_jump(&mut self) -> bool {
        return is_key_pressed(macroquad::input::KeyCode::Space);
    }

    fn get_id(&self) -> u64 {
        return 0;
    }



    


    

    

    
    fn get_current_direction_left_right(&mut self) -> super::input_device::InputDirectionLeftRight {
        if is_key_pressed(macroquad::input::KeyCode::A) {
            return InputDirectionLeftRight::Left;
        }
        if is_key_pressed(macroquad::input::KeyCode::D) {
            return InputDirectionLeftRight::Right;
        }
        if is_key_down(macroquad::input::KeyCode::A) {
            return InputDirectionLeftRight::Left;
        }
        if is_key_down(macroquad::input::KeyCode::D) {
            return InputDirectionLeftRight::Right;
        }

        return InputDirectionLeftRight::Neutral;
    }
    
    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown {
        if is_key_pressed(macroquad::input::KeyCode::W) {
            return InputDirectionUpDown::Up;
        }
        if is_key_pressed(macroquad::input::KeyCode::S) {
            return InputDirectionUpDown::Down;
        }
        if is_key_down(macroquad::input::KeyCode::W) {
            return InputDirectionUpDown::Up;
        }
        if is_key_down(macroquad::input::KeyCode::S) {
            return InputDirectionUpDown::Down;
        }

        return InputDirectionUpDown::Neutral;
    }
    
    fn should_begin_dash(&mut self) -> bool {
        return is_key_pressed(macroquad::input::KeyCode::Y);
    }
    
    fn should_begin_short_attack(&mut self) -> bool {
        return is_key_pressed(macroquad::input::KeyCode::R);
    }
    
    fn should_begin_long_attack(&mut self) -> bool {
        return is_key_pressed(macroquad::input::KeyCode::T);
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
}