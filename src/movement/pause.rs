use bevy::prelude::*;

#[derive(Resource)]
pub struct ActionPauseState {
    pub is_paused : bool
}