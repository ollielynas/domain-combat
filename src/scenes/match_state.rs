use macroquad::{color::ORANGE, models::draw_cylinder, shapes::{draw_circle, draw_circle_lines, draw_rectangle_lines, draw_rectangle_lines_ex}, text::draw_text};
use rapier2d::{control::*, parry::{query::contact, simba::scalar::SupersetOf}, prelude::{ColliderSet, PhysicsPipeline, RigidBody, RigidBodySet}};
use rapier2d::prelude::*;
use crate::{ game::{level::{Level, pick_random_level}, player::Player}, input_source::input_device::{InputDirectionLeftRight, InputDirectionUpDown}};
use macroquad::prelude::*;
use crate::consts::*;

/// this class handles the simulation of indivigual matches between players.
pub struct MatchState {
    players: Vec<Box<dyn Player>>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    kinematic_character_controller: KinematicCharacterController,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: DefaultBroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    ccd_solver: CCDSolver,
    level: Box<dyn Level>,
    level_rigid_bodies: Vec<RigidBodyHandle>,
}




impl MatchState {
    pub fn start_match(
        mut players:  Vec<Box<dyn Player>>,
    ) -> MatchState {

        let level = pick_random_level(players.len());

        let mut collider_set = ColliderSet::new();
        let mut rigid_body_set = RigidBodySet::new();
        let island_manager = IslandManager::new();
        let narrow_phase = NarrowPhase::new();
        let broad_phase = DefaultBroadPhase::new();
        let physics_pipeline = PhysicsPipeline::new();
        let impulse_joints = ImpulseJointSet::new();
        let multibody_joints = MultibodyJointSet::new();
        let ccd_solver = CCDSolver::default();
        let level_rigid_bodies = level.genetate_colliders(&mut rigid_body_set, &mut collider_set);



        let player_pos = level.generate_player_foot_pos(players.len());

        for i in 0..player_pos.len() {
            players[i].generate_rigid_body(player_pos[i].0, player_pos[i].1,&mut rigid_body_set,&mut collider_set);
            let health = players[i].get_max_health();
            players[i].set_health(health);
        }

        // add colition objects to level


        let controller = KinematicCharacterController {
                    slide: true,  // Slide along walls instead of stopping
                    autostep: Some(CharacterAutostep::default()),  // Auto-climb stairs
                    max_slope_climb_angle: 45.0_f32.to_radians(),  // Max climbable slope
                    ..Default::default()
                };

        return MatchState {
            level,
            rigid_body_set,
            collider_set,
            kinematic_character_controller: controller,
            players,
            physics_pipeline,
            broad_phase,
            narrow_phase,
            island_manager,
            impulse_joints,
            multibody_joints,
            ccd_solver,
            level_rigid_bodies,
        }


    }


    pub fn render(&mut self) {

        self.simulate();

        self.level.render_background();

        for (i, pl) in self.players.iter_mut().enumerate() {
            pl.render_from_physics(&self.rigid_body_set, i);
        }

        self.level.render_forground();

        self.render_debug();
    }


    fn render_debug(&mut self) {
        for b in self.rigid_body_set.iter() {
            for cl in b.1.colliders() {
            if let Some(c) = self.collider_set.get(*cl) {
                let x = c.position().translation.x;
                let y = c.position().translation.y;
                match c.shape().shape_type() {
                    ShapeType::Capsule => {
                        let cap = c.shape().as_capsule().unwrap();
                        draw_rectangle_lines(x - cap.radius, y - cap.half_height(), cap.radius*2.0, cap.height(), 3.0, ORANGE);

                        draw_circle_lines(x , y - cap.half_height(), cap.radius, 1.5, ORANGE);
                        draw_circle_lines(x , y + cap.half_height(), cap.radius, 1.5, ORANGE);
                    }
                    ShapeType::Cuboid => {
                        let rect = c.shape().as_cuboid().unwrap();
                        draw_rectangle_lines(x - rect.half_extents.x, y - rect.half_extents.y, rect.half_extents.x*2.0, rect.half_extents.y*2.0, 3.0, ORANGE);
                    }
                    ShapeType::Ball => {
                        let ball = c.shape().as_ball().unwrap();
                        draw_circle_lines(x, y, ball.radius, 1.5, ORANGE);
                    }
                    a => {
                        draw_text(&format!("rendering not supported for {:?}", a), x, y, DEBUG_TEXT_SIZE, ORANGE);
                    }
                }
            }

                }
            let xy = b.1.center_of_mass();
            draw_circle(xy.x, xy.y, 3.0, RED);

            draw_text(&format!(" center of mass"), xy.x, xy.y, 15.0, BLACK);
            draw_text(&format!(" mass: {}", b.1.mass()), xy.x, xy.y+15.0, 15.0, BLACK);

            draw_text(&format!(" sleeping: {}", b.1.activation().sleeping), xy.x, xy.y+DEBUG_TEXT_SIZE*2.0, DEBUG_TEXT_SIZE, BLACK);
            draw_line(xy.x, xy.y + b.1.linvel().y, xy.x + b.1.linvel().x, xy.y, 2.0, RED);
            let force =  b.1.user_force();

            draw_line(xy.x, xy.y, xy.x  + force.data.0[0][0] * ACC_DEBUG_ARROW_MULT, xy.y  + force.data.0[0][1] * ACC_DEBUG_ARROW_MULT, 2.0, GREEN);
        }
    for p in &mut self.players {
        if let Some(xy) = p.get_pos(&self.rigid_body_set) {
            draw_text(&format!("({},{})", xy.x, xy.y), xy.x + p.get_width()/2.0, xy.y - p.get_height()/2.0 + DEBUG_TEXT_SIZE, DEBUG_TEXT_SIZE, BLACK);
            if p.get_player_data_ref().jump_buffer>0.0 {
                draw_text(&format!("jump_buffer: {}", p.get_player_data_ref().jump_buffer), xy.x, xy.y+p.get_height()/2.0, DEBUG_TEXT_SIZE, BLACK);
            }else if p.get_on_ground() {
                draw_text("on_ground", xy.x, xy.y+p.get_height()/2.0, DEBUG_TEXT_SIZE, BLACK);
            }

            let inputs = vec![
                ("should_begin_dash", p.get_input_device_mut().should_begin_dash()),
                ("should_begin_jump", p.get_input_device_mut().should_begin_jump()),
                ("should_begin_long_attack", p.get_input_device_mut().should_begin_long_attack()),
                ("should_begin_short_attack", p.get_input_device_mut().should_begin_short_attack()),
                ("should_begin_move_left", p.get_input_device_mut().should_begin_move_left()),
                ("should_begin_move_right", p.get_input_device_mut().should_begin_move_right()),
            ];

            let mut inputs_text = inputs.iter().filter(|x| x.1).map(|x| x.0).collect::<Vec<&str>>().join(", ");

            inputs_text += match p.get_input_device_mut().get_current_direction_left_right() {
                InputDirectionLeftRight::Left => "move_left ",
                InputDirectionLeftRight::Right => "move_right ",
                InputDirectionLeftRight::Neutral => "",
            };
            inputs_text += match p.get_input_device_mut().get_current_direction_up_down() {
                InputDirectionUpDown::Down => "look_down ",
                InputDirectionUpDown::Up => "look_up ",
                InputDirectionUpDown::Neutral => "",
            };

            draw_text(&inputs_text, xy.x + p.get_width()/2.0, xy.y - p.get_height()/2.0 + DEBUG_TEXT_SIZE * 2.0, DEBUG_TEXT_SIZE, BLACK);

            // draw_text(&format!("({},{})", xy.x, xy.y), xy.x + p.get_width()/2.0, xy.y, 15.0, BLACK);
        }
    }
    }

    fn update_players_on_ground(&mut self) {
        for player in &mut self.players {
            if let Some(col2) = player.get_player_data_ref().detect_floor_collider_handle {
                if self.narrow_phase.intersection_pairs_with(col2).count() > 0 {
        for level_obj in &self.level_rigid_bodies {
            if let Some(rb) = self.rigid_body_set.get(*level_obj) {
                for col in rb.colliders() {
                            player.set_on_ground(match self.narrow_phase.intersection_pair(*col, col2) {
                                Some(true) => true,
                                _ => false,
                            });
                        }
                    }

                }
            }
            } else {
                player.set_on_ground(false);
            }
        }

    }

    fn apply_user_input_as_physics(&mut self) {
        for player in &mut self.players {
            player.apply_input_forces(&mut self.rigid_body_set);
        }
    }

    pub fn simulate(&mut self) {

        let (collision_send, collision_recv) = std::sync::mpsc::channel();
        let (contact_force_send, contact_force_recv) = std::sync::mpsc::channel();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);

        self.update_players_on_ground();
        self.apply_user_input_as_physics();

        self.physics_pipeline.step(
            &vector![0.0,GRAVITY],
            &IntegrationParameters {
                dt: 1.0/60.0, // exprox framerate
                min_ccd_dt: (1.0/60.0)/100.0, // 100 ccd substeps per step
                contact_damping_ratio: 0.5, // not really relervent cos there are no joints
                contact_natural_frequency: 30.0,
                length_unit: GAME_PHYSICS_SCALE,
                ..Default::default()

                // normalized_allowed_linear_error: (),
                // normalized_max_corrective_velocity: (),
                // normalized_prediction_distance: (),
                // num_solver_iterations: (),
                // num_internal_pgs_iterations: (),
                // num_internal_stabilization_iterations: (),
                // min_island_size: (),
                // max_ccd_substeps: () },
            },
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            &(),
            &event_handler
        );
        while let Ok(collision_event) = collision_recv.try_recv() {
            // Handle the collision event.
            println!("Received collision event: {:?}", collision_event);
        }

        while let Ok(contact_force_event) = contact_force_recv.try_recv() {
            // Handle the contact force event.
            println!("Received contact force event: {:?}", contact_force_event);
        }

    }

}
