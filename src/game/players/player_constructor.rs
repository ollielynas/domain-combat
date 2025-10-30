use crate::{
    game::{
        player::{Player, UniversalPlayerData},
        players::{debug_player1::DebugPlayer1, debug_player2::DebugPlayer2},
    },
    input_source::input_device::InputDevice,
};

pub struct PlayerConstructor {
    player_options: Vec<Box<dyn Player>>,
    input_device: Box<dyn InputDevice>,
    player_index: usize,
    selected_player_type_index: usize,
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
            player_options: players,
            input_device,
            player_index,
            selected_player_type_index: 0,
        };
    }


    fn process_input(&mut self) {
        // if self.input_device
    }

    pub fn render(&mut self) {



    }
}
