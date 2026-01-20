use crate::input_source::{
    dummy_input_device::dummy_input,
    input_device::{self, InputDevice},
};
use rapier2d::control::*;
use macroquad::text::draw_text;
use rapier2d::prelude::*;
use macroquad::prelude::*;

pub struct UniversalPlayerData {
    pub input_device: Box<dyn InputDevice>,
    pub player_name: String,
    pub health: f32,
}

impl UniversalPlayerData {
    pub fn new(input_device: Box<dyn InputDevice>, name: String) -> UniversalPlayerData {


        UniversalPlayerData {
            input_device,
            player_name: name,
            health: 0.0,
        }
    }

    pub fn dummy() -> UniversalPlayerData {
        UniversalPlayerData::new(dummy_input(), "".to_string())
    }
}
pub trait Player {
    fn get_player_data(&mut self) -> &mut UniversalPlayerData;

    fn render_sprite_at_pos(&mut self, x: f32, y: f32, scale: f32);
    fn get_height(&self) -> f32;
    fn get_width(&self) -> f32;
    fn get_name(&mut self) -> String;

    fn get_max_health(&self) -> f32;

    fn get_current_health(&mut self) -> f32 {
        return (self.get_player_data().health).clamp(0.0, self.get_max_health());
    }
    fn get_current_health_int(&mut self) -> i32 {
        return (self.get_player_data().health).clamp(0.0, self.get_max_health()).round() as i32;
    }

    fn is_alive(&mut self) -> bool {
        return  self.get_current_health() >= 0.5;
    }

    fn remove_health(&mut self, val: f32) {
        self.get_player_data().health -= val;
        self.get_player_data().health = self.get_current_health();
    }
    fn add_health(&mut self, val: f32) {
        self.get_player_data().health += val;
        self.get_player_data().health = self.get_current_health();
    }
    fn set_health(&mut self, val: f32) {
        self.get_player_data().health = val;
        self.get_player_data().health = self.get_current_health();
    }

    fn get_input_device_ref(&mut self) -> &mut Box<dyn InputDevice> {
        return &mut self.get_player_data().input_device;
    }

    fn render_sprite_at_pos_with_nametag(&mut self, x: f32, y: f32, scale: f32, player_index: usize) {
        self.render_sprite_at_pos(x, y, scale);
        // get_text_center(&format!("Player {}", player_index), None, 10.0*scale as u32, 1.0, 0.0);
        let size = draw_text(&format!("Player {}", player_index), 0.0, 0.0, 10.0*scale, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
        draw_text(&format!("Player {}", player_index), x + self.get_width()*scale/2.0 - size.width/2.0, y - size.height * 1.3, 10.0*scale, BLACK);
        draw_text(&format!("Hp: {}/{}", self.get_current_health_int(), self.get_max_health() as i32), x + self.get_width()*scale/2.0 - size.width/2.0, y - size.height * 0.3, 10.0*scale, BLACK);
    }
}
