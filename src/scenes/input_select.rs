use macroquad::prelude::*;
use macroquad::{text::draw_text, time::get_frame_time};

use crate::consts::*;
use crate::consts::WINDOW_SIZE;
use crate::gui::gui_button::{self, GuiButtonStyle, draw_gui_button};
use crate::gui::gui_window::{GuiWindowStyle, draw_gui_window};
use crate::input_source::dummy_input_device::dummy_input;
use crate::input_source::http_input_device::{create_http_input_server, get_new_adb_serials_and_setup_reverse};
use crate::input_source::input_device::{InputDevice, update_inputs_devices};
use crate::scenes::keybind_info::KeyBindInfoState;
use crate::scenes::lobby::LobbyState;
use crate::scenes::scenes::Scene;

use std::sync::mpsc::{self, Receiver};

pub struct InputSelect {
    input_devices: Vec<Box<dyn InputDevice>>,
    update_timer: f32,
    /// Receives ports set up by the ADB background thread
    adb_receiver: Option<Receiver<Vec<i32>>>,
    adb_scanning: bool,
}

impl InputSelect {
    pub fn add_http_device(&mut self) {
        self.input_devices.push(create_http_input_server(8080));
    }

    /// Spawns a background thread to run ADB detection so the game doesn't hang.
    /// The thread only does the slow adb reverse calls and sends back the ports.
    /// Devices are constructed on the main thread from those ports.
    pub fn start_adb_scan(&mut self) {
        if self.adb_scanning {
            return;
        }
        self.adb_scanning = true;
        let (tx, rx) = mpsc::channel();
        self.adb_receiver = Some(rx);
        std::thread::spawn(move || {
            let ports = get_new_adb_serials_and_setup_reverse();
            tx.send(ports).ok();
        });
    }

    /// Polls the ADB background thread for results. Call every frame.
    fn poll_adb_receiver(&mut self) {
        let Some(rx) = &self.adb_receiver else { return };
        match rx.try_recv() {
            Ok(ports) => {
                for port in ports {
                    self.input_devices.push(create_http_input_server(port));
                }
                self.adb_receiver = None;
                self.adb_scanning = false;
            }
            Err(mpsc::TryRecvError::Empty) => {} // still scanning
            Err(mpsc::TryRecvError::Disconnected) => {
                self.adb_receiver = None;
                self.adb_scanning = false;
            }
        }
    }

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
            return Some(Scene::KeybindInfoScene(KeyBindInfoState::new(new_device_array)));
        } else {
            return None;
        }
    }

    pub fn render(&mut self) {
        // Poll for ADB results every frame
        self.poll_adb_receiver();

        self.update_timer -= get_frame_time();
        if self.update_timer <= 0.0 {
            self.update_timer = 5.0;
            update_inputs_devices(&mut self.input_devices);
        }

        if draw_gui_button("add http input", MARGIN, WINDOW_SIZE.1 as f32 - 40.0, GuiButtonStyle::Basic).pressed {
            self.add_http_device();
        }

        let adb_label = if self.adb_scanning { "scanning for adb device..." } else { "connect adb device" };
        let adb_button_style = if self.adb_scanning { GuiButtonStyle::Basic } else { GuiButtonStyle::Basic };
        if draw_gui_button(adb_label, MARGIN, WINDOW_SIZE.1 as f32 - 80.0, adb_button_style).pressed && !self.adb_scanning {
            self.start_adb_scan();
        }

        // make sure input is calculated before number of enabled devices is calculated
        for input_device in self.input_devices.iter_mut() {
            input_device.as_mut().update_enabled_toggle();
            input_device.as_mut().update_start_game_toggle();
            input_device.as_mut().update();
        }
        self.input_devices.sort_by_key(|a| {
            (if a.is_enabled() { 0 } else { 1 } +
             if a.is_ready_to_play() { 0 } else { 1 })
        });
        let number_of_enabled_devices = self.input_devices.iter().filter(|x| x.is_enabled()).count();

        for (input_device_index, input_device) in self.input_devices.iter_mut().enumerate() {
            let enable_text = input_device.as_mut().enable_controller_instruction_text();
            let start_text = input_device.as_mut().start_game_instruction_text();
            let name = input_device.as_mut().get_name();

            if !input_device.is_enabled() {
                draw_gui_window(
                    &format!("{name}"),
                    MARGIN, 100.0 * (input_device_index - number_of_enabled_devices) as f32 + MARGIN * 5.0,
                    50.0,
                    WINDOW_SIZE.0 as f32 / 2.0 - MARGIN * 2.0 - 20.0,
                    GuiWindowStyle::Basic);
                draw_text(
                    &format!("{enable_text}"),
                    MARGIN * 2.0, 100.0 * (input_device_index - number_of_enabled_devices) as f32 + MARGIN * 5.0 + GUI_WINDOW_TITLE_HEIGHT,
                    30.0,
                    BLACK,
                );
            } else {
                draw_gui_window(
                    &format!("{name}"),
                    MARGIN + screen_width() / 2.0, 100.0 * (input_device_index) as f32 + MARGIN * 5.0,
                    50.0,
                    WINDOW_SIZE.0 as f32 / 2.0 - MARGIN * 2.0 - 20.0,
                    GuiWindowStyle::Basic);
                draw_text(
                    &format!("{start_text}"),
                    MARGIN * 2.0 + screen_width() / 2.0, 100.0 * (input_device_index) as f32 + MARGIN * 5.0 + GUI_WINDOW_TITLE_HEIGHT + MARGIN,
                    30.0,
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
            adb_receiver: None,
            adb_scanning: false,
        }
    }
}
