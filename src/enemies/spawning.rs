use std::time::Duration;

use bevy::{prelude::*, time::Stopwatch, window::PrimaryWindow};
use rand::{seq::IteratorRandom, Rng};

use crate::{
    animation::AnimationStateStorage,
    constants::SortingLayers,
    loading::TextureAssets,
    movement::pause::ActionPauseState,
    util::rng::{GlobalSeed, RNG},
};

use super::{
    beholder::{spawn_beholder, spawn_beholder_prince, BeholderAnimation},
    enemy::EnemyType,
    imp::{spawn_imp, spawn_imp_queen, ImpAnimation},
    reaper::{spawn_reaper, ReaperAnimation},
};

#[derive(Resource)]
pub struct SpawnInfo {
    pub timer: Timer,
    pub game: Stopwatch,
    pub count: u32,
}

#[derive(Event)]
pub struct EnemySpawnEvent {
    pub enemy_type : EnemyType,
    pub position : Vec2,
}

#[derive(Resource)]
pub struct SpawningRNG(pub RNG);

pub fn spawn_spawn_rng(
    seed: Res<GlobalSeed>,
    mut spawn_info: ResMut<SpawnInfo>,
    mut commands: Commands,
) {
    // Make enemies spawn fast
    spawn_info.timer.set_elapsed(Duration::from_secs_f32(5.0));

    commands.insert_resource(SpawningRNG(RNG::new(&seed.0, "spawning")))
}

pub fn spawn_loop(
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut spawn_info: ResMut<SpawnInfo>,
    time: Res<Time>,
    pause: Res<ActionPauseState>,
    mut rng: ResMut<SpawningRNG>,
    mut spawn_ev : EventWriter<EnemySpawnEvent>,
) {
    if pause.is_paused {
        return;
    }

    let window = q_window.single();
    spawn_info.timer.tick(time.delta());
    spawn_info.game.tick(time.delta());

    let scaling_factor: f32 = 1.01;
    let needed_difficulty =
        2. + 3. * scaling_factor.powf(spawn_info.count as f32) * spawn_info.count as f32;

    if spawn_info.timer.just_finished() {
        spawn_info.timer.reset();
        let mut curr_difficulty = 0.;
        spawn_info.count += 1;

        while curr_difficulty < needed_difficulty {
            curr_difficulty *= 1.5; // Scaling for multiple enemies

            let position = match rng.0 .0.gen_range(0..4) {
                0 => Vec3 {
                    x: rng
                        .0
                         .0
                        .gen_range((window.width() / -2.)..(window.width() / 2.)),
                    y: window.height() / 2. + 32.,
                    z: SortingLayers::Action.into(),
                },
                1 => Vec3 {
                    x: rng
                        .0
                         .0
                        .gen_range((window.width() / -2.)..(window.width() / 2.)),
                    y: window.height() / -2. - 32.,
                    z: SortingLayers::Action.into(),
                },
                2 => Vec3 {
                    x: window.width() / -2. - 32.,
                    y: rng
                        .0
                         .0
                        .gen_range((window.height() / -2.)..(window.height() / 2.)),
                    z: SortingLayers::Action.into(),
                },
                _ => Vec3 {
                    x: window.width() / 2. + 32.,
                    y: rng
                        .0
                         .0
                        .gen_range((window.height() / -2.)..(window.height() / 2.)),
                    z: SortingLayers::Action.into(),
                },
            };

            let all_enemies = &EnemyType::all();
            let available = all_enemies
                .iter()
                .filter(|enemy| enemy.difficulty() + curr_difficulty < needed_difficulty + 10.);

            match available.choose(&mut rng.0 .0) {
                Some(enemy) => {
                    curr_difficulty += enemy.difficulty();
                    info!("Spawning {:?}, difficult {}/{}", enemy, curr_difficulty, needed_difficulty);
                    spawn_ev.send(EnemySpawnEvent { enemy_type: *enemy, position: position.truncate() });
                }
                None => return,
            }
        }
    }
}
