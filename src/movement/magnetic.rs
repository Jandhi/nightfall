use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

use crate::{player::Player, util::radians::Radian};

use super::velocity::Velocity;

/*
Things that are magnetically attracted to the player
*/

#[derive(Component)]
pub struct Magnetic {
    pub force : f32,
}

pub fn magnet_update(
    q_player : Query<&Transform, (With<Player>, Without<Magnetic>)>,
    mut q_magnetics : Query<(&Magnetic, &mut Velocity, &Transform)>,
    time : Res<Time>,
) {
    let player_transform = q_player.single();
    for (magnet, mut velocity, transform) in q_magnetics.iter_mut() {
        let direction = player_transform.translation.truncate() - transform.translation.truncate();
        // obtain angle to target with respect to x-axis.
        let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
        let direction_vec = angle_to_target.unit_vector();

        let force = magnet.force / direction.length();
        screen_print!("Magnetic force is {}", force);
        velocity.vec += direction_vec * force * time.delta().as_secs_f32();
    }
}