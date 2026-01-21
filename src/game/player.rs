use crate::input_source::{
    dummy_input_device::dummy_input,
    input_device::{self, InputDevice, InputDirectionLeftRight},
};
use rapier2d::{control::*, na::Isometry};
use macroquad::text::draw_text;
use rapier2d::prelude::*;
use macroquad::prelude::*;
use crate::consts::*;

pub struct UniversalPlayerData {
    pub input_device: Box<dyn InputDevice>,
    pub player_name: String,
    pub health: f32,
    pub rigid_object_handle: Option<RigidBodyHandle>,
    pub detect_floor_collider_handle: Option<ColliderHandle>,
    pub hitbox_collider_handle: Option<ColliderHandle>,
    pub on_ground: bool,
    pub jump_buffer: f32,
}

impl UniversalPlayerData {
    pub fn new(input_device: Box<dyn InputDevice>, name: String) -> UniversalPlayerData {


        UniversalPlayerData {
            input_device,
            player_name: name,
            health: 0.0,
            rigid_object_handle: None,
            detect_floor_collider_handle: None,
            hitbox_collider_handle: None,
            on_ground: false,
            jump_buffer: 0.0,
        }
    }

    pub fn dummy() -> UniversalPlayerData {
        UniversalPlayerData::new(dummy_input(), "".to_string())
    }
}
pub trait Player {
    fn get_player_data(&mut self) -> &mut UniversalPlayerData;
    fn get_player_data_ref(&self) -> &UniversalPlayerData;

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

    fn get_input_device_mut(&mut self) -> &mut Box<dyn InputDevice> {
        return &mut self.get_player_data().input_device;
    }
    fn get_input_device_ref(&mut self) ->  &Box<dyn InputDevice> {
        return &self.get_player_data_ref().input_device;
    }

    fn render_sprite_at_pos_with_nametag(&mut self, x: f32, y: f32, scale: f32, player_index: usize) {
        self.render_sprite_at_pos(x, y, scale);
        // get_text_center(&format!("Player {}", player_index), None, 10.0*scale as u32, 1.0, 0.0);
        let size = draw_text(&format!("Player {}", player_index), 0.0, 0.0, PLAYER_TEXT_NAME_SIZE*scale, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 });
        draw_text(&format!("Player {}", player_index), x + self.get_width()*scale/2.0 - size.width/2.0, y - size.height * 1.3, PLAYER_TEXT_NAME_SIZE*scale, BLACK);
        draw_text(&format!("Hp: {}/{}", self.get_current_health_int(), self.get_max_health() as i32), x + self.get_width()*scale/2.0 - size.width/2.0, y - size.height * 0.3, PLAYER_TEXT_NAME_SIZE*scale, RED);
    }


    fn generate_rigid_body(&mut self,x: f32, y: f32, rigid_body_set: &mut RigidBodySet, collider_set: &mut ColliderSet) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x,y])
            .lock_rotations()
            .can_sleep(false)
            .build();
        let collider = ColliderBuilder::capsule_y(self.get_height() / 2.0 - self.get_width()/2.0, self.get_width()/2.0)
            .friction(PLAYER_FRICTION)
            .build()
            ;
        let detect_floor = ColliderBuilder::ball(self.get_width()*COLLIDER_BALL_SCALE_SIZE)
            .sensor(true)
            .mass(0.0)
        .position(
            Isometry { rotation: Rotation::new(0.0), translation: Translation::new(0.0, self.get_height()/2.0 - self.get_width() * 0.4 + 3.0 ) }
        ).build();
        let rigid_body_handle = rigid_body_set.insert(rigid_body);
        let hitbox_collider_handle = collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);
        let detect_floor_collider_handle = collider_set.insert_with_parent(detect_floor, rigid_body_handle, rigid_body_set);

        self.get_player_data().rigid_object_handle = Some(rigid_body_handle);
        self.get_player_data().hitbox_collider_handle = Some(hitbox_collider_handle);
        self.get_player_data().detect_floor_collider_handle = Some(detect_floor_collider_handle);
    }

    fn get_rigid_body_handle(&mut self) -> Option<RigidBodyHandle> {
        self.get_player_data().rigid_object_handle
    }

    fn render_from_physics(&mut self, rigid_body_set: &RigidBodySet, player_index: usize) {
        if let Some(h) = self.get_rigid_body_handle() {
            if let Some(b) = rigid_body_set.get(h) {
                let pos = b.position();
                self.render_sprite_at_pos_with_nametag(
                    pos.translation.x - self.get_width() * 0.5,
                    pos.translation.y - self.get_height() * 0.5,
                    1.0,
                    player_index
                );
            }
        }
    }

    fn air_speed_multiplier(&self) -> f32 {1.0}
    fn ground_speed_multiplier(&self) -> f32 {1.0}
    fn jump_multiplier(&self) -> f32 {1.0}

    fn apply_input_forces(&mut self, rigid_body_set: &mut RigidBodySet) {

        let rbh = self.get_rigid_body_handle();
        let rb = match rbh {
            Some(a) => {rigid_body_set.get_mut(a)}
            None => None
        };

        let rigid_body: &mut RigidBody = match rb {
            Some(a) => a,
            None => {return}
        };

        let velocity = rigid_body.linvel().clone();


        if self.get_input_device_mut().should_begin_jump() {
            self.get_player_data().jump_buffer = JUMP_BUFFER_TIME
        }

        if self.get_player_data().jump_buffer > 0.0 {
            self.get_player_data().jump_buffer -= get_frame_time();
        }

        if self.get_on_ground() && self.get_player_data().jump_buffer > 0.0  {
            self.get_player_data().jump_buffer = 0.0;
            rigid_body.set_linvel(vector![rigid_body.linvel().x, 0.0], true);
             rigid_body.apply_impulse(vector![0.0,-JUMP_FORCE_REL_TO_GRAVITY_AND_MASS * GRAVITY * rigid_body.mass() * self.jump_multiplier() ], true);
        }

        if self.get_input_device_mut().should_begin_move_left() && velocity.x >= 0.0 {
            rigid_body.apply_impulse(vector![-FIRST_ACC_BOOST * rigid_body.mass(), 0.0], true);
        }
        if self.get_input_device_mut().should_begin_move_right() && velocity.x <= 0.0 {
            rigid_body.apply_impulse(vector![FIRST_ACC_BOOST * rigid_body.mass(), 0.0], true);
        }

        let direction_mod = match self.get_input_device_mut().get_current_direction_left_right() {
            InputDirectionLeftRight::Left => -1.0,
            InputDirectionLeftRight::Right => 1.0,
            InputDirectionLeftRight::Neutral => 0.0,
        };

        rigid_body.reset_forces(true);
        if direction_mod != 0.0 {
        if self.get_on_ground() {
            rigid_body.add_force(vector![
                self.ground_speed_multiplier() * WALK_FORCE_ON_GROUND_REL_TO_MASS * rigid_body.mass() * direction_mod
                ,0.0], true);
        }else {
            rigid_body.add_force(vector![
                self.air_speed_multiplier() * WALK_FORCE_ON_AIR_REL_TO_MASS * rigid_body.mass() * direction_mod
                ,0.0], true);
        }
        }

    }

    fn get_mass(&self, rigid_body_set: &RigidBodySet) -> f32 {
        if let Some(handle) = self.get_player_data_ref().rigid_object_handle {
            if let Some(rb) = rigid_body_set.get(handle) {
                return rb.mass()
            }
        }
        return 1.0;
    }

    fn set_on_ground(&mut self, on_ground: bool) {
       self.get_player_data().on_ground = on_ground;
    }

    fn get_on_ground(&self) -> bool {
        self.get_player_data_ref().on_ground
    }

    fn get_pos(&self, rigid_body_set: &RigidBodySet) -> Option<Translation<f32>> {
        if let Some(handle) = self.get_player_data_ref().rigid_object_handle {
        if let Some(p) = rigid_body_set.get(handle) {
            return Some(p.position().translation);
            }
        }
        return None;
    }
}
