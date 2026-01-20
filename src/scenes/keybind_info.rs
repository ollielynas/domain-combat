use crate::Scene;
use crate::input_source::input_device::InputDevice;
use crate::scenes::lobby::LobbyState;
use macroquad::prelude::*;
pub struct KeyBindInfoState {
    input_devices: Vec<Box<dyn InputDevice>>,
    current_player: usize,
}

impl KeyBindInfoState {
    pub fn new(input_devices: Vec<Box<dyn InputDevice>>) -> KeyBindInfoState {
        return KeyBindInfoState {
            input_devices: input_devices,
            current_player: 0,
        };
    }
    pub fn get_input_devices_mut(&mut self) -> &mut Vec<Box<dyn InputDevice>> {
        return &mut self.input_devices;
    }

    pub fn switch_scene(&mut self) -> Option<Scene> {
        if self.current_player >= self.input_devices.len() {
            return Some(Scene::Lobby(LobbyState::new(self.input_devices.drain(0..self.input_devices.len()).collect())))
        }
        return None;
    }

    pub fn process_input(&mut self) {
        if self.current_player >= self.input_devices.len() {return;}
        if self.input_devices[self.current_player].should_begin_jump() {
            self.current_player += 1;
        }
    }

    pub fn render(&mut self) {

        self.process_input();

        if self.current_player >= self.input_devices.len() {
            return;
        }

        draw_text(
            &format!("Player {} Input Controls", self.current_player + 1),
            10.0,
            30.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to move left", self.input_devices[self.current_player].get_left_keybind()),
            10.0,
            60.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to move right", self.input_devices[self.current_player].get_right_keybind()),
            10.0,
            90.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to attack upward", self.input_devices[self.current_player].get_up_keybind()),
            10.0,
            120.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to attack downward", self.input_devices[self.current_player].get_down_keybind()),
            10.0,
            150.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to jump", self.input_devices[self.current_player].get_jump_keybind()),
            10.0,
            180.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Input [{}] for fast attacks",
                self.input_devices[self.current_player].get_fast_attack_keybind()
            ),
            10.0,
            210.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Input [{}] for strong attacks",
                self.input_devices[self.current_player].get_strong_attack_keybind()
            ),
            10.0,
            240.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Press [{}] to continue",
                self.input_devices[self.current_player].get_jump_keybind()
            ),
            30.0,
            270.0,
            15.0,
            BLACK,
        );
    }
}
