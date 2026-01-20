use rapier2d::{control::*, prelude::{ColliderSet, RigidBody, RigidBodySet}};

use crate::game::{level::{Level, pick_random_level}, player::Player};




pub struct MatchState {
    players: Vec<Box<dyn Player>>,
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    kinematic_character_controller: KinematicCharacterController,
    level: Box<dyn Level>,
}




impl MatchState {
    pub fn start_match(
        players:  Vec<Box<dyn Player>>,
    ) -> MatchState {

        let mut collider_set = ColliderSet::new();
        let mut rigid_body_set = RigidBodySet::new();

        let level = pick_random_level(players.len());

        // add colition objects to level
        level.genetate_colliders(&mut rigid_body_set, &mut collider_set);

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
        }
    }


    pub fn render(&mut self) {
        self.level.render_background();
        self.level.render_forground();
    }
}
