use bevy::{prelude::*, window::PrimaryWindow};
use rand::{Rng, seq::IteratorRandom};

use crate::{movement::pause::ActionPauseState, animation::AnimationStateStorage, loading::TextureAssets, constants::SortingLayers, util::rng::{RNG, GlobalSeed}};

use super::{imp::{spawn_imp, ImpAnimation, spawn_imp_queen}, beholder::{spawn_beholder, BeholderAnimation, spawn_beholder_prince}, enemy::EnemyType};

#[derive(Resource)]
pub struct SpawnInfo{
    pub timer : Timer,
    pub count : u32,
}

#[derive(Resource)]
pub struct SpawningRNG(pub RNG);

pub fn spawn_spawn_rng(
    seed : Res<GlobalSeed>,
    mut commands: Commands,
) {
    commands.insert_resource(SpawningRNG(RNG::new(&seed.0, "spawning")))
}

pub fn spawn_loop(
    q_window : Query<&Window, With<PrimaryWindow>>,
    mut spawn_info : ResMut<SpawnInfo>,
    time : Res<Time>,
    pause : Res<ActionPauseState>,
    imp_animations: Res<AnimationStateStorage<ImpAnimation>>,
    beholder_animations: Res<AnimationStateStorage<BeholderAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rng : ResMut<SpawningRNG>,
    mut commands: Commands,

) {
    if pause.is_paused {
        return;
    }

    let window = q_window.single();
    spawn_info.timer.tick(time.delta());

    let scaling_factor : f32 = 1.01;
    let needed_difficulty = 2. + 3. * scaling_factor.powf(spawn_info.count as f32) * spawn_info.count as f32;

    if spawn_info.timer.just_finished() {
        spawn_info.timer.reset();
        let mut curr_difficulty = 0.; 
        spawn_info.count += 1;

        while curr_difficulty < needed_difficulty {
            curr_difficulty *= 1.5; // Scaling for multiple enemies

            let position = match rng.0.0.gen_range(0..4) {
                0 => {
                    Vec3{ x: rng.0.0.gen_range((window.width() / -2.)..(window.width() / 2.)), y: window.height() / 2. + 32., z: SortingLayers::Action.into() }
                },
                1 => {
                    Vec3{ x: rng.0.0.gen_range((window.width() / -2.)..(window.width() / 2.)), y: window.height() / -2. - 32., z: SortingLayers::Action.into() }
                },
                2 => {
                    Vec3{ x: window.width() / -2. - 32., y: rng.0.0.gen_range((window.height() / -2.)..(window.height() / 2.)), z: SortingLayers::Action.into() }
                },
                _ => {
                    Vec3{ x: window.width() / 2. + 32., y: rng.0.0.gen_range((window.height() / -2.)..(window.height() / 2.)), z: SortingLayers::Action.into() }
                }
            };

            let all_enemies = &EnemyType::all();
            let available = all_enemies
                .iter()
                .filter(|enemy| enemy.difficulty() + curr_difficulty < needed_difficulty + 10.);

            match available.choose(&mut rng.0.0) {
                Some(enemy) => { 
                    curr_difficulty += enemy.difficulty();

                    match enemy {
                        EnemyType::Imp => spawn_imp(position, &imp_animations, &textures, &mut texture_atlases, &mut commands),
                        EnemyType::ImpQueen => spawn_imp_queen(position, &imp_animations, &textures, &mut texture_atlases, &mut commands),
                        EnemyType::Beholder => spawn_beholder(position, &beholder_animations, &textures, &mut texture_atlases, &mut commands),
                        EnemyType::BeholderPrince => spawn_beholder_prince(position, &beholder_animations, &textures, &mut texture_atlases, &mut commands),
                    }
                },
                None => return,
            }
            
        }
    }
}