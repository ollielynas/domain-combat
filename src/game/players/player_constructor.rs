use std::mem::swap;

use macroquad::prelude::*;
use crate::consts::*;
use crate::{
     game::{
        player::{self, Player, UniversalPlayerData},
        players::{debug_player1::DebugPlayer1, debug_player2::DebugPlayer2},
    }, input_source::{dummy_input_device::dummy_input, input_device::InputDevice}
};

pub struct PlayerConstructor {
    char_options: Vec<Box<dyn Player>>,
    input_device: Box<dyn InputDevice>,
    player_index: usize,
    selected_char_index: usize,
    ready_to_construct_player: bool,
}

impl PlayerConstructor {
    pub fn new(input_device: Box<dyn InputDevice>, player_index: usize) -> PlayerConstructor {
        let mut players: Vec<Box<dyn Player>> = vec![];

        let debug_player1 = DebugPlayer1 {
            data: UniversalPlayerData::dummy(),
        };
        let debug_player2 = DebugPlayer2 {
            data: UniversalPlayerData::dummy(),
        };

        players.push(Box::new(debug_player1));
        players.push(Box::new(debug_player2));

        for player in players.iter_mut() {
            player.set_health(player.get_max_health());
        }

        return PlayerConstructor {
            char_options: players,
            input_device,
            player_index,
            selected_char_index: 0,
            ready_to_construct_player: false,
        };
    }

    pub fn construct_player(&mut self) -> Box<dyn Player> {
            let mut player = self.char_options.remove(self.selected_char_index);
            let mut new_player_data =
                UniversalPlayerData::new(dummy_input(), format!("Player {}", self.player_index));

            swap(&mut new_player_data.input_device, &mut self.input_device);
            swap(player.get_player_data(), &mut new_player_data);

            return player;
    }

    pub fn is_player_ready_to_be_constructed(&self) -> bool {self.ready_to_construct_player}

    fn process_input(&mut self) {
        if self.input_device.should_begin_move_left() {
            if self.selected_char_index == 0 {
                self.selected_char_index = self.char_options.len() - 1;
            } else {
                self.selected_char_index -= 1;
            }
        }
        if self.input_device.should_begin_move_right() {
            if self.selected_char_index >= self.char_options.len() - 1 {
                self.selected_char_index = 0;
            } else {
                self.selected_char_index += 1;
            }
        }
        if self.input_device.should_begin_jump() {
            self.ready_to_construct_player = !self.ready_to_construct_player;
        }
    }

    pub fn render(&mut self, index: usize, total: usize) {
        self.process_input();

        let left_pos = WINDOW_SIZE.0 as f32 * ((index + 1) as f32 / (total + 2) as f32);
        let top_pos = WINDOW_SIZE.1 as f32 * 0.75;

        let selected_player: &mut (dyn Player + 'static) =
            self.char_options[self.selected_char_index].as_mut();

        let scale = match total {
            0..3 => 3.0,
            3..=5 => 2.0,
            _ => 1.0
        };

        selected_player.render_sprite_at_pos_with_nametag(left_pos, top_pos - selected_player.get_height() * scale, scale, index + 1);
        draw_text(
            &format!(
                "<- {} ->",
                // self.input_device.get_left_keybind(),
                selected_player.get_name(),
                // self.input_device.get_right_keybind()
            ),
            left_pos,
            top_pos + 25.0,
            20.0,
            BLACK,
        );
        draw_text(
            &format!(
                "Ready to play: {} [{}]",
                self.is_player_ready_to_be_constructed(),
                self.input_device.get_jump_keybind(),
            ),
            left_pos,
            top_pos + 50.0,
            20.0,
            BLACK,
        );

    }
}
