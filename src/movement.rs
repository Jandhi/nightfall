use bevy::prelude::*;

use crate::GameState;
use self::{magnetic::magnet_update, velocity::velocity_update, friction::friction_update, pause::ActionPauseState};

pub mod magnetic;
pub mod velocity;
pub mod friction;
pub mod pause;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                velocity_update,
                magnet_update,
                friction_update,
            ).run_if(in_state(GameState::Playing)),)
            .insert_resource(ActionPauseState{ is_paused: false });
    }
}