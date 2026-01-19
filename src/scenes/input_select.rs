use macroquad::prelude::*;
use macroquad::{text::draw_text, time::get_frame_time};

use crate::input_source::dummy_input_device::dummy_input;
use crate::input_source::input_device::{InputDevice, update_inputs_devices};
use crate::scenes::char_select::CharSelectState;
use crate::scenes::scenes::Scene;

pub struct InputSelect {
    input_devices: Vec<Box<dyn InputDevice>>,
    update_timer: f32,
}

impl InputSelect {
    pub fn switch_scene(&mut self) -> Option<Scene> {
        let mut all_ready = false;
        for input in &mut self.input_devices {
            if input.is_enabled() {
                all_ready = true;
            }
        }

        for input in &mut self.input_devices {
            if input.is_enabled() && !input.is_ready_to_play() {
                all_ready = false;
            }
        }

        if all_ready {
            let mut new_device_array = vec![];
            self.input_devices.retain_mut(|f| f.is_ready_to_play());
            while self.input_devices.len() > 0 {
                new_device_array.push(self.input_devices.pop().unwrap_or(dummy_input()));
            }
            return Some(Scene::CharacterSelect(CharSelectState::new(
                new_device_array,
            )));
        } else {
            return None;
        }
    }

    pub fn render(&mut self) {
        self.update_timer -= get_frame_time();
        if self.update_timer <= 0.0 {
            self.update_timer = 5.0;
            update_inputs_devices(&mut self.input_devices);
        }
        let mut player_index = 0;
        for (i, input_device) in self.input_devices.iter_mut().enumerate() {
            input_device.as_mut().update_enabled_toggle();
            input_device.as_mut().update_start_game_toggle();

            let enable_text = input_device.as_mut().enable_controller_instruction_text();
            let start_text = input_device.as_mut().start_game_instruction_text();
            let name = input_device.as_mut().get_name();

            if input_device.is_enabled() {
                player_index += 1;
            }

            draw_text(
                &format!(
                    "{i}.) {name}: {enable_text}, {start_text}, enabled:{} ready to start:{}",
                    input_device.as_mut().is_enabled(),
                    input_device.as_mut().is_ready_to_play()
                ),
                10.0,
                i as f32 * 40.0 + 30.0,
                15.0,
                BLACK,
            );
            if input_device.is_enabled() {
                draw_text(
                    &format!("player: {player_index}",),
                    10.0,
                    i as f32 * 40.0 + 50.0,
                    15.0,
                    BLACK,
                );
            }
        }
    }
}

impl Default for InputSelect {
    fn default() -> Self {
        InputSelect {
            input_devices: vec![],
            update_timer: 0.0,
        }
    }
}
