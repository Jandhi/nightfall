use bevy::prelude::*;

use super::rng::{GlobalSeed, RNG};

#[derive(Resource)]
pub struct PitchRNG(pub RNG);

pub fn spawn_pitch_rng(seed: Res<GlobalSeed>, mut commands: Commands) {
    commands.insert_resource(PitchRNG(RNG::new(&seed.0, "pitch")))
}
