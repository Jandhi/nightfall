use bevy::prelude::*;

use super::{pause::ActionPauseState, velocity::Velocity};

#[derive(Component)]
pub struct Friction {
    pub force: f32,
}

pub fn friction_update(
    mut q_friction: Query<(&Friction, &mut Velocity)>,
    time: Res<Time>,
    pause_state: Res<ActionPauseState>,
) {
    if pause_state.is_paused {
        return;
    }

    for (friction, mut velocity) in q_friction.iter_mut() {
        let force = friction.force * time.delta_seconds();
        let direction_vector = velocity.vec.normalize();

        if velocity.vec.length() < force {
            velocity.vec = Vec2::ZERO;
        } else {
            velocity.vec -= direction_vector * force;
        }
    }
}
