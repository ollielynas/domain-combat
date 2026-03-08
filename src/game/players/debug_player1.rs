use crate::{animations::{animation_frames::{AniFrame, AnimationFramesConstructor}, animation_manager::AnimationManager}, game::player::{Player, UniversalPlayerData}};

use macroquad::prelude::*;

pub struct DebugPlayer1 {
    pub data: UniversalPlayerData,
    pub spritesheet: AnimationManager,
}

impl Default for DebugPlayer1 {
    fn default() -> Self {
        Self { data: UniversalPlayerData::dummy(), spritesheet: AnimationManager::new() }
    }
}


impl Player for DebugPlayer1 {
    fn get_player_data(&mut self) -> &mut crate::game::player::UniversalPlayerData {
        return &mut self.data;
    }
    fn get_player_data_ref(&self) -> &UniversalPlayerData {
        &self.data
    }

    fn get_animation_manager(&mut self) -> &mut AnimationManager {
        &mut self.spritesheet
    }
    fn get_animation_manager_ref(&self) -> &AnimationManager {
        &self.spritesheet
    }

    fn render_sprite_at_pos(&mut self, x: f32, y:f32, scale:f32) {
        draw_rectangle(x, y, self.get_width()*scale, self.get_height()*scale, GREEN);
    }

    fn get_name(&mut self) -> String {
        "debug char 1".to_string()
    }

    fn get_height(&self) -> f32 {
        40.0
    }
    fn get_width(&self) -> f32 {
        20.0
    }

    fn get_max_health(&self) -> f32 {
        100.0
    }

    fn ground_speed_multiplier(&self) -> f32 {
        1.0
    }
    fn air_speed_multiplier(&self) -> f32 {
        1.0
    }
    fn jump_multiplier(&self) -> f32 {
        1.0
    }

}
