use macroquad::shapes::draw_rectangle;
use macroquad::prelude::*;
use crate::game::player::{Player, UniversalPlayerData};



pub struct DebugPlayer2 {
    pub data: UniversalPlayerData,
}

impl Player for DebugPlayer2 {
    fn get_player_data(&mut self) -> &mut crate::game::player::UniversalPlayerData {
        return &mut self.data;
    }

    fn render_sprite_at_pos(&mut self, x: f32, y:f32, scale:f32) {
        draw_rectangle(x, y, 10.0*scale, 20.0*scale, RED);
    }


    fn get_name(&mut self) -> String {
        "debug char 2".to_string()
    }
}