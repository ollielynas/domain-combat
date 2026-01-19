use crate::Scene;
use crate::game::player::Player;
use macroquad::prelude::*;
pub struct KeyBindInfoState {
    players: Vec<Box<dyn Player>>,
    current_player: usize,
}

impl KeyBindInfoState {
    pub fn new(players_vec: Vec<Box<dyn Player>>) -> KeyBindInfoState {
        return KeyBindInfoState {
            players: players_vec,
            current_player: 0,
        };
    }
    pub fn get_players_mut(&mut self) -> &mut Vec<Box<dyn Player>> {
        return &mut self.players;
    }

    pub fn switch_scene(&mut self) -> Option<Scene> {
        return None;
    }

    pub fn render(&mut self) {
        if self.current_player >= self.players.len() {
            return;
        }

        self.players[self.current_player].render_sprite_at_pos(800.0, 45.0, 2.0);
        let p_input = self.players[self.current_player].get_input_device_ref();
        draw_text(
            &format!("Player [{}] Input Controls", self.current_player + 1),
            10.0,
            30.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to move left", p_input.get_left_keybind()),
            10.0,
            60.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to move right", p_input.get_right_keybind()),
            10.0,
            90.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to attack upward", p_input.get_up_keybind()),
            10.0,
            120.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to attack downward", p_input.get_down_keybind()),
            10.0,
            150.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!("Input [{}] to jump", p_input.get_jump_keybind()),
            10.0,
            180.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Input [{}] for fast attacks",
                p_input.get_fast_attack_keybind()
            ),
            10.0,
            210.0,
            15.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Input [{}] for strong attacks",
                p_input.get_strong_attack_keybind()
            ),
            10.0,
            240.0,
            15.0,
            BLACK,
        );
    }
}
