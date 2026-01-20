use macroquad::{color::Color, shapes::draw_rectangle, window::clear_background};
use macroquad::prelude::*;
use rapier2d::prelude::{ColliderBuilder, RigidBody, RigidBodyBuilder};
use crate::{config::window_config::WINDOW_SIZE, game::level::Level};
use rapier2d::prelude::*;

pub struct DebugLevel {
    number_of_players: usize,

}

impl DebugLevel {
    pub fn new(number_of_players: usize) -> DebugLevel {
        return DebugLevel {
            number_of_players,
        }
    }
}

impl Level for DebugLevel {

    fn render_background(&self) {
        clear_background(Color {r: 0.529, g: 0.808, b: 0.922, a:1.0});
        draw_rectangle(0.0,WINDOW_SIZE.1 as f32 - WINDOW_SIZE.1 as f32 / 5.0, WINDOW_SIZE.0 as f32, WINDOW_SIZE.1 as f32 / 5.0 + 10.0, GRAY);
    }
    fn render_forground(&self) {
        draw_text(&format!("{} players", self.number_of_players), 10.0, 30.0, 15.0, RED);
        draw_text(&format!("{}", get_time()), 10.0, 60.0, 15.0, RED);
    }

    fn genetate_colliders(&self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet ) {
        let floor_body = RigidBodyBuilder::fixed()
            .pose(Isometry::new(vector![0.0, WINDOW_SIZE.1 as f32 - WINDOW_SIZE.1 as f32 / 5.0], 0.0))
            .build();
        let floor_collider = ColliderBuilder::cuboid((WINDOW_SIZE.1 as f32 / 2.0), (WINDOW_SIZE.1 as f32 / 10.0))
            .build();
        let floor_body_handle = rigid_body_set.insert(floor_body);
        collider_set.insert_with_parent(floor_collider, floor_body_handle, rigid_body_set);
    }
}
