use bevy::prelude::*;


pub mod tower;
use crate::{towers::tower::tower_trigger, GameState};

pub struct TowersPlugin;

impl Plugin for TowersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tower_trigger.run_if(in_state(GameState::Playing)));
    }
}