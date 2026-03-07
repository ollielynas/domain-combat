use macroquad::{input::{is_key_down, is_key_pressed}, time::get_fps};
use std::hash::{Hash, Hasher};
use midi_msg::{MidiMsg, Channel};

use crate::input_source::input_device::{InputDevice, InputDeviceData, InputDirectionLeftRight, InputDirectionUpDown};
use midir::{Ignore, MidiInput, MidiInputConnection};



pub fn get_midi_inputs() -> Vec<Box<dyn InputDevice>> {
    let mut midi_in = MidiInput::new("my-app").unwrap();
    midi_in.ignore(Ignore::None);
    println!("port count {}",midi_in.port_count());
    if midi_in.port_count() < 1 {
        return vec![];
    }
    let connections: Vec<_> = (0..midi_in.port_count()).map(|i|make_connection(format!("device-{i}"), i)).collect();

    return connections;
}

fn make_connection(name: String, port_index: usize) ->  Box<dyn InputDevice> {
    let mut midi_in = MidiInput::new(&name).unwrap();
    midi_in.ignore(Ignore::None);

    let ports = midi_in.ports();
    let port = &ports[port_index];

    let name = midi_in.port_name(&port).unwrap_or(name);

     new_midi_input(midi_in.connect(port, "read", move |timestamp, message, _| {
    }, ()).unwrap(), name)
}

pub struct MidiInputDevice {
    device_data: InputDeviceData,
    midi_connection: MidiInputConnection<()>,
    check_connection_counter: i32,
    name: String,
    connected: bool,
}

pub fn new_midi_input(connection: MidiInputConnection<()>, name: String) -> Box<dyn InputDevice> {
    return Box::new( MidiInputDevice {
        midi_connection: connection,
        device_data: InputDeviceData {enabled: false, ready_to_play: false, score: 0 },
        check_connection_counter: 0,
        name,
        connected: true,
    });
}

fn is_midi_connected(name: &str) -> bool {
    let mut midi_in = MidiInput::new("checker").unwrap();
    midi_in.ignore(Ignore::None);

    midi_in.ports().iter().any(|p| {
        midi_in.port_name(p).ok().as_deref() == Some(name)
    })
}


impl InputDevice for MidiInputDevice {

    fn update(&mut self) {
        self.check_connection_counter -= 1;
        if self.check_connection_counter < 0 {
            self.check_connection_counter += get_fps().max(10);
            if !is_midi_connected(&self.get_name()) {
                self.connected = false;
            }
        }
    }

    fn get_name(&mut self) -> String {
        self.name.to_string()
    }

    fn is_disconnected(&self) -> bool {
        !self.connected
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


    fn get_current_direction_left_right(&mut self) -> super::input_device::InputDirectionLeftRight {

        return InputDirectionLeftRight::Neutral;
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
