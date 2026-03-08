use std::{collections::{HashMap, HashSet}, io::Error};

use macroquad::{color::*, math::{Vec2, vec2}, text::draw_text, time::get_frame_time};
use rapier2d::{math::Vector, prelude::RigidBodySet};

use crate::{animations::animation_frames::AnimationFrames, consts::*, game::player::UniversalPlayerData, input_source::input_device::DirectionLeftRight};


/// stands for animation state
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug)]
pub enum Ani {
    Error,
    Jump,
    JumpForward,
    Idle,
    IdleLowHealth,
    WalkForward,
    RunForward,
    SlideForward,
    SlideBackward,
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
            Ani::WalkForward => vec![Ani::RunForward, Ani::SlideForward, Ani::Idle],
            Ani::Dash => vec![Ani::SlideForward, Ani::RunForward, Ani::WalkForward, Ani::JumpForward, Ani::SlideBackward],
            Ani::FallDown => vec![Ani::Idle],
            Ani::FallUp => vec![Ani::Idle],
            Ani::FastAttack => vec![Ani::Error],
            Ani::StrongAttack => vec![Ani::Error],
            Ani::RunForward => vec![Ani::WalkForward, Ani::Idle],
            Ani::SlideForward => vec![Ani::Idle],
            Ani::SlideBackward => vec![Ani::Idle],
            Ani::IdleLowHealth => vec![Ani::Idle],
        };
        for f in fallbacks {
            if existing_animations.contains(&f) {return f}
        }
        return Ani::Error
    }

    pub fn animation_priority(&self) -> i32 {
        match self {
            Ani::Dash | Ani::StrongAttack | Ani::FastAttack => 10,
            Ani::Jump | Ani::JumpForward => 5,
            _ => 0,
        }
    }
    pub fn animation_terminates(&self) -> bool {
        match self {
            Ani::Dash | Ani::StrongAttack | Ani::FastAttack |
            Ani::Jump | Ani::JumpForward => true,
            _ => false,
        }
    }

}


pub struct AnimationManager {
    texture_remap: HashMap<Ani, Ani>,
    frames_map: HashMap<Ani, AnimationFrames>,
    current_ani: Ani,
    animation_time: f32,
    facing_direction: DirectionLeftRight,
}

impl AnimationManager {

    pub fn new() -> AnimationManager {
        AnimationManager { facing_direction: DirectionLeftRight::Right, animation_time: 0.0, texture_remap: HashMap::<Ani, Ani>::new(), frames_map:  HashMap::<Ani, AnimationFrames>::new(), current_ani: Ani::Idle }
    }

    pub fn determine_current_animation(&mut self, player: &mut UniversalPlayerData, rigid_body_set: &RigidBodySet) {
        let last_ani = self.current_ani.clone();
        let rbh = player.rigid_object_handle;
        let rb = match rbh {
            Some(a) => {rigid_body_set.get(a)}
            None => None
        };

        if player.input_device.get_current_direction_left_right() != DirectionLeftRight::Neutral {
            self.facing_direction = player.input_device.get_current_direction_left_right();
        }

        let velocity = match rb {
            Some(a) => vec2(a.linvel().x, a.linvel().y),
            None => {self.current_ani = Ani::Idle; return},
        };



        // idle is the default
        self.current_ani = Ani::Idle;

        // next assume only movement
        if player.on_ground {
            self.current_ani = match (velocity.x.abs() > RUN_WALK_BOUNDARY_VELOCITY, &self.facing_direction, player.input_device.get_current_direction_left_right()) {
                (true, DirectionLeftRight::Left, DirectionLeftRight::Neutral) => Ani::SlideBackward,
                (true, DirectionLeftRight::Right, DirectionLeftRight::Neutral) => Ani::SlideForward,
                (false, _, DirectionLeftRight::Neutral) => Ani::Idle,
                (true, _, DirectionLeftRight::Right) if velocity.x > 0.0 => Ani::RunForward,
                (true, _, DirectionLeftRight::Left) if velocity.x < 0.0 => Ani::RunForward,
                (_, _, DirectionLeftRight::Left | DirectionLeftRight::Right) => Ani::WalkForward,
                _ => self.current_ani
            };
        }else {
            self.current_ani = match (velocity.x.abs() > RUN_WALK_BOUNDARY_VELOCITY, &self.facing_direction, velocity.y > 0.0) {
                (true, DirectionLeftRight::Right, _) => Ani::FallForward,
                (_, _, true) => Ani::FallDown,
                (_, _, false) => Ani::FallUp,
                (false, _, _) => Ani::Idle,
                _ => self.current_ani
            };

        }

        if player.health <= LOW_HEATH_CUTOFF && self.current_ani == Ani::Idle {
            self.current_ani = Ani::IdleLowHealth
        }


        if self.current_ani != last_ani {
            self.animation_time = 0.0;
        }
    }

    pub fn render_aniamiton(&mut self, x: f32, y: f32, scale: f32) {
        let flip_x = self.facing_direction == DirectionLeftRight::Left;
        self.animation_time += get_frame_time();
        let mut ani = self.current_ani;
        if !self.frames_map.contains_key(&ani) {
            ani = ani.get_fallback(self.frames_map.keys().cloned().collect())
        }
        match self.frames_map.get_mut(&ani) {
            Some(a) => {
                a.render_frame(x, y, self.animation_time, scale, flip_x);
                draw_text(&format!("{:?}",self.current_ani), x, y-30.0, DEBUG_TEXT_SIZE, BLACK);
            },
            None => {
                draw_text("failed to load animation", x, y, DEBUG_TEXT_SIZE, RED);
            },
        }
    }

    pub fn add_animation(&mut self, frames: AnimationFrames) {
        self.frames_map.insert(frames.anamaiton_state.clone(), frames);
    }

}
