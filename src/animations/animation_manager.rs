use std::{collections::{HashMap, HashSet}, io::Error};

use macroquad::{color::RED, math::vec2, text::draw_text, time::get_frame_time};
use rapier2d::{math::Vector, prelude::RigidBodySet};

use crate::{animations::animation_frames::AnimationFrames, consts::DEBUG_TEXT_SIZE, game::player::UniversalPlayerData};


/// stands for animation state
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Ani {
    Error,
    Jump,
    JumpForward,
    Idle,
    WalkForward,
    FallForward,
    Dash,
    FallDown,
    FallUp,
    FastAttack,
    StrongAttack,
}


impl Ani {
    pub fn get_fallback(&mut self, existing_animations: HashSet<Ani>) -> Ani {
        let fallbacks = match self {
            Ani::Error => panic!(),
            Ani::FallForward => vec![Ani::FallDown, Ani::Idle],
            Ani::Jump => vec![Ani::FallUp, Ani::Idle],
            Ani::JumpForward => vec![Ani::Jump, Ani::FallUp, Ani::Idle],
            Ani::Idle => vec![],
            Ani::WalkForward => todo!(),
            Ani::Dash => vec![Ani::WalkForward, Ani::JumpForward],
            Ani::FallDown => vec![Ani::Idle],
            Ani::FallUp => vec![Ani::Idle],
            Ani::FastAttack => vec![Ani::Error],
            Ani::StrongAttack => vec![Ani::Error],
        };
        for f in fallbacks {
            if existing_animations.contains(&f) {return f}
        }
        return Ani::Error
    }
}


pub struct AnimationManager {
    texture_remap: HashMap<Ani, Ani>,
    frames_map: HashMap<Ani, AnimationFrames>,
    current_ani: Ani,
    animation_time: f32,

}

impl AnimationManager {

    pub fn new() -> AnimationManager {
        AnimationManager { animation_time: 0.0, texture_remap: HashMap::<Ani, Ani>::new(), frames_map:  HashMap::<Ani, AnimationFrames>::new(), current_ani: Ani::Idle }
    }

    pub fn determine_current_animation(&mut self, player: UniversalPlayerData, rigid_body_set: &mut RigidBodySet) {
        let last_ani = self.current_ani.clone();
        let rbh = player.rigid_object_handle;
        let rb = match rbh {
            Some(a) => {rigid_body_set.get_mut(a)}
            None => None
        };
        // let velocity = match rb {
        //     Some(a) => a.linvel(),
        //     None => {panic!()},
        // };

        // match (player.on_ground,) {

        // }

        self.current_ani = Ani::Idle;

        if self.current_ani != last_ani {
            self.animation_time = 0.0;
        }
    }

    pub fn render_aniamiton(&mut self, x: f32, y: f32, scale: f32) {
        self.animation_time += get_frame_time();

        match self.frames_map.get_mut(&self.current_ani) {
            Some(a) => a.render_frame(x, y, self.animation_time, scale),
            None => {
                draw_text("failed to load animation", x, y, DEBUG_TEXT_SIZE, RED);
            },
        }
    }

    pub fn add_animation(&mut self, frames: AnimationFrames) {
        self.frames_map.insert(frames.anamaiton_state.clone(), frames);
    }

}
