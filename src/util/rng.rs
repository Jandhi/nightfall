use bevy::prelude::*;
use rand_pcg::Pcg64;
use rand_seeder::Seeder;

#[derive(Resource)]
pub struct GlobalSeed(pub String);

#[derive(Component)]
pub struct RNG(pub Pcg64);

impl RNG {
    pub fn new(global_seed: &str, derivative_seed: &str) -> RNG {
        let mut seed = String::from(global_seed);
        seed += derivative_seed;

        RNG(Seeder::from(seed).make_rng())
    }
}
