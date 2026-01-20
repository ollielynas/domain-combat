use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
};

use crate::input_source::{
    keyboard_input_device::{WasdKeyboardInputDevice, get_all_wasd_keyboards},
    numpad_input_device::{NumpadInputDevice, get_all_numpads},
};

pub enum InputDirectionLeftRight {
    Left,
    Right,
    Neutral,
}
pub enum InputDirectionUpDown {
    Up,
    Down,
    Neutral,
}

pub enum InputButton {
    FastAttack,
    LongAttack,
    Dash,
    Jump,
}

pub struct InputDeviceData {
    pub enabled: bool,
    pub ready_to_play: bool,
    pub score: i32,
}

impl Default for InputDeviceData {
    fn default() -> Self {
        InputDeviceData {
            enabled: false,
            ready_to_play: false,
            score: 0,
        }
    }
}

pub fn update_inputs_devices(devices: &mut Vec<Box<dyn InputDevice>>) {
    let mut new_devices = vec![];
    for device_type in [get_all_wasd_keyboards(), get_all_numpads()] {
        for device in device_type {
            new_devices.push(device);
        }
    }
    let current_hashes: HashSet<u64> = devices.iter().map(|d| d.get_id()).collect();
    let new_hashes: HashSet<u64> = new_devices.iter().map(|d| d.get_id()).collect();
    devices.retain(|d| new_hashes.contains(&d.get_id()));
    for dev in new_devices {
        if !current_hashes.contains(&dev.get_id()) {
            devices.push(dev);
        }
    }
}

pub trait InputDevice {
    fn get_id(&self) -> u64;

    fn get_input_device_data(&mut self) -> &mut InputDeviceData;

    fn is_enabled(&mut self) -> bool {
        return self.get_input_device_data().enabled;
    }

    fn is_ready_to_play(&mut self) -> bool {
        return self.get_input_device_data().ready_to_play;
    }

    fn get_current_direction_left_right(&mut self) -> InputDirectionLeftRight;
    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown;

    fn should_begin_move_right(&mut self) -> bool;
    fn should_begin_move_left(&mut self) -> bool;

    fn should_begin_dash(&mut self) -> bool;
    fn should_begin_short_attack(&mut self) -> bool;
    fn should_begin_long_attack(&mut self) -> bool;
    fn should_begin_jump(&mut self) -> bool;

    fn get_fast_attack_keybind(&mut self) -> String;
    fn get_strong_attack_keybind(&mut self) -> String;
    fn get_jump_keybind(&mut self) -> String;
    fn get_left_keybind(&mut self) -> String;
    fn get_right_keybind(&mut self) -> String;
    fn get_up_keybind(&mut self) -> String;
    fn get_down_keybind(&mut self) -> String;

    fn get_name(&mut self) -> String;

    fn enable_controller_instruction_text(&mut self) -> String {
        return format!("to enable player press : [{}]", self.get_jump_keybind());
    }

    fn start_game_instruction_text(&mut self) -> String {
        return format!(
            "if you are ready to start press : [{}]",
            self.get_fast_attack_keybind()
        );
    }

    fn update_start_game_toggle(&mut self) {
        if self.should_begin_short_attack() {
            self.get_input_device_data().ready_to_play =
                !self.get_input_device_data().ready_to_play;
        }
        if !self.get_input_device_data().enabled {
            self.get_input_device_data().ready_to_play = false;
        }
    }

    fn update_enabled_toggle(&mut self) {
        if self.should_begin_jump() {
            self.get_input_device_data().enabled = !self.get_input_device_data().enabled;
        }
    }
}
