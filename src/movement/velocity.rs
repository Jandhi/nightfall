use bevy::prelude::*;

use crate::constants::DISTANCE_SCALING;

use super::pause::ActionPauseState;

#[derive(Component)]
pub struct Velocity {
    pub vec : Vec2,
}

pub fn velocity_update(
    mut q_velocity : Query<(&Velocity, &mut  Transform)>,
    time : Res<Time>,
    pause_state : Res<ActionPauseState>,
) {
    if pause_state.is_paused {
        return;
    }

    for (velocity, mut transform) in q_velocity.iter_mut() {
        transform.translation += Vec3 {
            x: velocity.vec.x,
            y: velocity.vec.y,
            z: 0.,
        } * time.delta_seconds() * DISTANCE_SCALING;
    }
}

impl From<Vec2> for Velocity {
    fn from(value: Vec2) -> Self {
        Velocity { vec: value }
    }
}