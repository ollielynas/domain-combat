
use rapier2d::prelude::*;

use crate::game::levels::{self, debug_level::DebugLevel};



pub trait Level {
    fn render_background(&self);
    fn render_forground(&self);
    fn genetate_colliders(&self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet );
}
pub fn pick_random_level(number_of_players: usize) -> Box<dyn Level> {
    let mut levels: Vec<Box<dyn Level>> = vec![];
    levels.push(Box::new(DebugLevel::new(number_of_players)));

    return fastrand::choice(levels.drain(0..levels.len())).unwrap_or(Box::new(DebugLevel::new(number_of_players)));
}
