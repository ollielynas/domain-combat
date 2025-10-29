pub enum InputDirectionLeftRight {
    Left,
    Right,
    Neutral,
}
pub enum InputDirectionUpDown {
    Up,
    Down,
    Neutral
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
}

impl Default for InputDeviceData {
    fn default() -> Self {
        InputDeviceData { 
            enabled: false,
            ready_to_play: false,
        }
    }
}


pub trait InputDevice {


    fn get_input_device_data(&mut self) -> &mut InputDeviceData;

    fn is_enabled(&mut self) -> bool {
        return self.get_input_device_data().enabled
    }

    fn is_ready_to_play(&mut self) -> bool {
        return self.get_input_device_data().ready_to_play;
    }


    fn detect_and_return_all() -> Vec<Box<Self>>;

    fn get_current_direction_left_right(&mut self) -> InputDirectionLeftRight;
    fn get_current_direction_up_down(&mut self) -> InputDirectionUpDown;

    fn should_begin_dash(&mut self) -> bool;
    fn should_begin_short_attack(&mut self) -> bool;
    fn should_begin_long_attack(&mut self) -> bool;
    fn should_begin_jump(&mut self) -> bool;

    fn get_attack_keybind() -> String;
    fn get_jump_keybind() -> String;
    fn get_left_keybind() -> String;
    fn get_right_keybind() -> String;
    fn get_up_keybind() -> String;
    fn get_down_keybind() -> String;

    fn enable_controller_instruction_text() -> String {
        return format!("to enable player press : [{}]", Self::get_jump_keybind());
    }
    fn start_game_instruction_text() -> String {
        return format!("if you are ready to start press : [{}]", Self::get_attack_keybind());
    }
    
    fn update_start_game_toggle(&mut self) {
        if self.should_begin_jump() {
            self.get_input_device_data().enabled = !self.get_input_device_data().enabled;
        }
    }

    fn update_enabled_toggle(&mut self) {
        if self.should_begin_jump() {
            self.get_input_device_data().enabled = !self.get_input_device_data().enabled;
        }
    }
}
