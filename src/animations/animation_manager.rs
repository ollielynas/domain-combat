use std::{collections::{HashMap, HashSet}, io::Error};

use macroquad::math::vec2;
use rapier2d::{math::Vector, prelude::RigidBodySet};

use crate::{animations::animation_frames::AnimationFrames, game::player::UniversalPlayerData};


/// stands for animation state
#[derive(Hash, PartialEq, Eq)]
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
}

impl AnimationManager {

    pub fn determine_current_animaiton(&mut self, player: UniversalPlayerData, rigid_body_set: &mut RigidBodySet) {
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
    }

    pub fn render_aniamiton(&mut self, player_data: UniversalPlayerData) {

    }

}
