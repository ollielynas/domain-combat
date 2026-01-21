
use rapier2d::{na::{Vector, Vector2}, prelude::*};

use crate::game::levels::{self, debug_level::DebugLevel};



pub trait Level {
    fn render_background(&self);
    fn render_forground(&self);
    /// this funciton should be implemented but not called
    fn genetate_colliders_without_tracking_handles(&self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet );
    fn generate_player_foot_pos(&self, number_of_players: usize) -> Vec<(f32,f32)>;

    /// this funciton is the one that should be called when the object is created
    fn genetate_colliders(&self, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet ) -> Vec<RigidBodyHandle> {
        let pre_level_colliders: Vec<RigidBodyHandle> = rigid_body_set.iter_mut().map(|x| x.0.clone()).collect();
        self.genetate_colliders_without_tracking_handles( rigid_body_set, collider_set);
        let mut post_level_colliders: Vec<RigidBodyHandle> = rigid_body_set.iter_mut().map(|x| x.0.clone()).collect();
        post_level_colliders.retain(|x| !pre_level_colliders.contains(x));
        return post_level_colliders;
    }
}
pub fn pick_random_level(number_of_players: usize) -> Box<dyn Level> {
    let mut levels: Vec<Box<dyn Level>> = vec![];
    levels.push(Box::new(DebugLevel::new(number_of_players)));

    return fastrand::choice(levels.drain(0..levels.len())).unwrap_or(Box::new(DebugLevel::new(number_of_players)));
}
