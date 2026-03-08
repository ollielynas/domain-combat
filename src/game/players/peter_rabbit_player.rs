use crate::{animations::{animation_frames::AnimationFramesConstructor, animation_manager::{Ani, AnimationManager}}, game::player::{Player, UniversalPlayerData}};
use crate::animations::animation_manager;
use macroquad::prelude::*;

pub struct PeterRabbitPlayer {
    pub data: UniversalPlayerData,
    pub spritesheet: AnimationManager,
}

impl Default for PeterRabbitPlayer {
    fn default() -> Self {
        let mut ani_m = AnimationManager::new();

        let mut idle = AnimationFramesConstructor::new();
        idle.add_frame(include_bytes!(".././../img/PeterRabbit/idle.png"));
        idle.align_bottom();

        idle.add_frame(include_bytes!(".././../img/PeterRabbit/idle.png"));
        idle.set_scale(1.2);
        idle.align_bottom();

        ani_m.add_animation(idle.build(true, false, Ani::Idle, 1.0));

        Self {
            data: UniversalPlayerData::dummy(),
            spritesheet: ani_m,
        }
    }
}




impl Player for PeterRabbitPlayer {
    fn get_player_data(&mut self) -> &mut crate::game::player::UniversalPlayerData {
        return &mut self.data;
    }
    fn get_player_data_ref(&self) -> &UniversalPlayerData {
        &self.data
    }

    fn get_animation_manager(&mut self) -> &mut crate::animations::animation_manager::AnimationManager {
        &mut self.spritesheet
    }
    fn get_animation_manager_ref(&self) -> &AnimationManager {
        &self.spritesheet
    }

    fn get_name(&mut self) -> String {
        "Peter Rabbit".to_string()
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
