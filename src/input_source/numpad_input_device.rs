use macroquad::input::{KeyCode, is_key_down, is_key_pressed};

use crate::input_source::input_device::{InputDevice, InputDeviceData, InputDirectionLeftRight, InputDirectionUpDown};




pub struct NumpadInputDevice {
    device_data: InputDeviceData,
}


pub fn get_all_numpads() -> Vec<Box<dyn InputDevice>> {
    return vec![
        Box::new(NumpadInputDevice {
            device_data: InputDeviceData::default(),
        })
    ]
}


impl InputDevice for NumpadInputDevice {

    fn get_name(&mut self) -> String {
        "Numpad".to_string()
    }
    
    fn get_input_device_data(&mut self) -> &mut InputDeviceData {
        return &mut self.device_data;
    }

    fn should_begin_jump(&mut self) -> bool {
        
        return is_key_pressed(KeyCode::Kp0);
    }

    fn get_id(&self) -> u64 {
        return 0;
    }


    fn get_current_direction_left_right(&mut self) -> super::input_device::InputDirectionLeftRight {
        
        if is_key_down(KeyCode::Kp5) && is_key_down(KeyCode::KpAdd) {
            return InputDirectionLeftRight::Neutral;
        }

        if is_key_down(KeyCode::Kp5) {
            return InputDirectionLeftRight::Left;
        }
        if is_key_down(KeyCode::KpAdd) {
            return InputDirectionLeftRight::Right;
        }

        return InputDirectionLeftRight::Neutral;
    }
    
    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown {
        

        if is_key_down(KeyCode::Kp9) && is_key_down(KeyCode::Kp6) {
            return InputDirectionUpDown::Neutral;
        }

        if is_key_down(KeyCode::Kp9) {
            return InputDirectionUpDown::Up;
        }
        if is_key_down(KeyCode::Kp6) {
            return InputDirectionUpDown::Down;
        }

        return InputDirectionUpDown::Neutral;
    }
    
    fn should_begin_dash(&mut self) -> bool {
        return is_key_pressed(KeyCode::Kp7);
    }
    
    fn should_begin_short_attack(&mut self) -> bool {
        return is_key_pressed(KeyCode::Kp1);
    }
    
    fn should_begin_long_attack(&mut self) -> bool {
        return is_key_pressed(KeyCode::Kp4);
    }
    
    fn get_fast_attack_keybind(&mut self) -> String {
        return "Numpad 1".to_string();
    }
    fn get_strong_attack_keybind(&mut self) -> String {
        return "Numpad 4".to_string();
    }
    fn get_jump_keybind(&mut self) -> String {
        return "Numpad 0".to_string();
    }
    
    fn get_left_keybind(&mut self) -> String {
        return "Numpad 5".to_string();
    }
    
    fn get_right_keybind(&mut self) -> String {
        return "Numpad Plus".to_string();
    }
    
    fn get_up_keybind(&mut self) -> String {
        return "Numpad 9".to_string();
    }
    
    fn get_down_keybind(&mut self) -> String {
        return "Numpad 6".to_string();
    }
    
    fn should_begin_move_right(&mut self) -> bool {
        is_key_pressed(KeyCode::KpAdd)
    }
    
    fn should_begin_move_left(&mut self) -> bool {
        is_key_pressed(KeyCode::Kp5)
    }
    
}