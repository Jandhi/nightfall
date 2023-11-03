use bevy::{prelude::*};
use rand::{rngs::OsRng, seq::SliceRandom};

use crate::GameState;

use self::{pitch_rng::spawn_pitch_rng, rng::GlobalSeed};

pub mod pitch_rng;
pub mod radians;
pub mod rng;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {

        app.add_systems(OnEnter(GameState::Playing), spawn_pitch_rng)
            .insert_resource(GlobalSeed(vec![
                "dawn",
                "sun",
                "moon",
                "blade",
                "ring",
                "lantern",
                "beast",
                "shade",
                "hood",
                "powder",
                "doom",
                "gaze",
                "end",
                "flame",
            ].choose(&mut OsRng).unwrap().to_string()));
    }
}
