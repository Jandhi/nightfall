use std::f32::consts::PI;

use bevy::prelude::*;


use crate::{player::Player, util::radians::Radian};

use super::pause::ActionPauseState;

/*
Things that are magnetically attracted to the player
*/

#[derive(Component)]
pub struct FakeMagnetic {
    pub force : f32,
}

const MINIMUM_FORCE_THRESHOLD : f32 = 0.8;
pub fn fake_magnet_update(
    q_player : Query<&Transform, (With<Player>, Without<FakeMagnetic>)>,
    mut q_magnetics : Query<(&FakeMagnetic, &mut Transform)>,
    time : Res<Time>,
    pause_state : Res<ActionPauseState>,
) {
    if pause_state.is_paused {
        return;
    }

    let player_transform = q_player.single();
    for (magnet, mut transform) in q_magnetics.iter_mut() {

        let force = magnet.force * time.delta_seconds() / player_transform.translation.distance_squared(transform.translation);

        if force >= player_transform.translation.distance(transform.translation) {
            transform.translation.x = player_transform.translation.x;
            transform.translation.y = player_transform.translation.y;
        } else if force < MINIMUM_FORCE_THRESHOLD { 
            continue;
        } else {
            let direction = player_transform.translation.truncate() - transform.translation.truncate();
            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
            let direction_vec = angle_to_target.unit_vector();

            transform.translation += force * Vec3{
                x: direction_vec.x,
                y: direction_vec.y,
                z: 0.,
            };
        }
    }
}