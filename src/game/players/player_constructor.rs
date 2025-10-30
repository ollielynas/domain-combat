use std::mem::swap;

use macroquad::prelude::*;

use crate::{
    game::{
        player::{Player, UniversalPlayerData},
        players::{debug_player1::DebugPlayer1, debug_player2::DebugPlayer2},
    },
    input_source::{dummy_input_device::dummy_input, input_device::InputDevice},
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

        return PlayerConstructor {
            char_options: players,
            input_device,
            player_index,
            selected_char_index: 0,
            ready_to_construct_player: false,
        };
    }

    pub fn construct_player_if_ready(&mut self) -> Option<Box<dyn Player>> {
        if self.ready_to_construct_player {
            let mut player = self.char_options.remove(self.selected_char_index);
            let mut new_player_data =
                UniversalPlayerData::new(dummy_input(), format!("Player {}", self.player_index));
            
            swap(&mut new_player_data.input_device, &mut self.input_device);
            swap(player.get_player_data(), &mut new_player_data);


            return Some(player);
        }

        return None;
    }

    fn process_input(&mut self) {
        if self.input_device.should_begin_move_left() {
            if self.selected_char_index == 0 {
                self.selected_char_index = self.char_options.len() - 1;
            } else {
                self.selected_char_index -= 1;
            }
        }
        if self.input_device.should_begin_move_left() {
            if self.selected_char_index == 0 {
                self.selected_char_index = self.char_options.len() - 1;
            } else {
                self.selected_char_index -= 1;
            }
        }
        if self.input_device.should_begin_jump() {
            self.ready_to_construct_player = true;
        }
    }

    pub fn render(&mut self) {
        self.process_input();

        let selected_player: &mut (dyn Player + 'static) =
            self.char_options[self.selected_char_index].as_mut();

        draw_text(
            &format!("current player: Player{}", self.player_index),
            10.0,
            30.0,
            20.0,
            BLACK,
        );
        draw_text(
            &format!("selected character {}", selected_player.get_name()),
            10.0,
            60.0,
            20.0,
            BLACK,
        );

        selected_player.render_sprite_at_pos(20.0, 80.0, 1.0);
    }
}
