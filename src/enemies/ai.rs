use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{player::Player, movement::{pause::ActionPauseState, velocity::Velocity}, constants::DISTANCE_SCALING, util::radians::Radian};

use super::enemy;


#[derive(Component)]
pub struct FollowPlayerAI {
    // The desired speed
    pub speed: f32,

    // How quick the velocity corrects
    pub corrective_force: f32,
}

pub fn follow_player(
    mut q_enemies: Query<(&Transform, &FollowPlayerAI, &mut Velocity)>,
    q_player: Query<&Transform, (With<Player>, Without<FollowPlayerAI>)>,
    pause : Res<ActionPauseState>,
) {
    if pause.is_paused {
        return;
    }
    
    let player_transform = q_player.single();

    for (transform, ai, mut velocity) in q_enemies.iter_mut() {
        let direction = player_transform.translation.truncate() - transform.translation.truncate();
        // obtain angle to target with respect to x-axis.
        let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
        let direction_vec = angle_to_target.unit_vector();

        let desired_velocity = direction_vec * ai.speed;
        let diff = desired_velocity - velocity.vec;

        if diff.length() < ai.corrective_force {
            velocity.vec = desired_velocity;
        } else {
            velocity.vec += diff.normalize() * ai.corrective_force;
        }
    }
}