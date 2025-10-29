use macroquad::input::{is_key_down, is_key_pressed};

use crate::input_source::input_device::{InputDevice, InputDeviceData, InputDirectionLeftRight, InputDirectionUpDown};




pub struct NumpadInputDevice {
    device_data: InputDeviceData,
}



impl InputDevice for NumpadInputDevice {
    
    fn get_input_device_data(&mut self) -> &mut InputDeviceData {
        return &mut self.device_data;
    }

    fn should_begin_jump(&mut self) -> bool {
        return is_key_pressed(macroquad::input::KeyCode::Zero);
    }



    fn detect_and_return_all() -> Vec<Box<Self>> {
        vec![
            Box::new(NumpadInputDevice { 
                device_data: InputDeviceData::default()
            })
        ]
    }


    

    

    fn get_toggle_enable_text() -> String {
        return "Press Spacebar to enable/disable".to_owned();
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
        todo!()
    }
    
    fn should_begin_short_attack(&mut self) -> bool {
        todo!()
    }
    
    fn should_begin_long_attack(&mut self) -> bool {
        todo!()
    }
}