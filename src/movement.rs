use bevy::prelude::*;

use self::{
    edge_teleport::edge_teleporting, fake_magnetic::fake_magnet_update, friction::friction_update,
    magnetic::magnet_update, pause::{ActionPauseState, update_pause_menu, pause_keypress, TogglePauseMenu, PauseMenuState, click_unpause}, velocity::velocity_update,
};
use crate::GameState;

pub mod edge_teleport;
pub mod fake_magnetic;
pub mod friction;
pub mod magnetic;
pub mod pause;
pub mod velocity;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                velocity_update,
                magnet_update,
                fake_magnet_update,
                friction_update,
                edge_teleporting,
                pause_keypress,
                update_pause_menu,
                click_unpause,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_event::<TogglePauseMenu>()
        .insert_resource(PauseMenuState(false))
        .insert_resource(ActionPauseState { is_paused: false });
    }
}
