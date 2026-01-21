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


    fn get_player_data_ref(&self) -> &UniversalPlayerData {
        &self.data
    }

    fn render_sprite_at_pos(&mut self, x: f32, y:f32, scale:f32) {
        draw_rectangle(x, y, self.get_width()*scale, self.get_height()*scale, RED);
    }

    fn get_height(&self) -> f32 {
        50.0
    }
    fn get_width(&self) -> f32 {
        25.0
    }


    fn get_name(&mut self) -> String {
        "debug char 2".to_string()
    }

    fn get_max_health(&self) -> f32 {
        120.0
    }
}
