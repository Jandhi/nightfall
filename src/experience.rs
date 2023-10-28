use bevy::prelude::*;

use crate::{GameState, player::spawn_player};

use self::xp_crystal::{create_rng, drop_crystals, xp_crystal_update};

mod xp_crystal;
pub mod experience_meter;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
            create_rng,
        )).add_systems(Update, (
            drop_crystals,
            xp_crystal_update
        ).run_if(in_state(GameState::Playing)));
    }
}