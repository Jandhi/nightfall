use bevy::{prelude::*, time::Stopwatch};

use crate::{animation::AppAnimationSetup, GameState};

use self::{
    ai::{follow_player, move_and_shoot_ai, ChargeShootEvent, ShootEvent},
    beholder::{beholder_update, BeholderAnimation, BeholderProjectileAnimation, spawn_beholder_prince, spawn_beholder},
    enemy::{death_loop, spread_enemies, EnemyDeathEvent},
    imp::{ImpAnimation, spawn_imp_queen, spawn_imp},
    reaper::{reaper_blade_update, reaper_update, ReaperAnimation, ReaperBladeAnimation, spawn_reaper},
    spawning::{spawn_loop, spawn_spawn_rng, SpawnInfo, EnemySpawnEvent}, spawn_menu::SpawnMenuPlugin,
};

pub mod ai;
pub mod beholder;
pub mod enemy;
pub mod imp;
pub mod reaper;
pub mod spawning;
pub mod zombie;
pub mod spawn_menu;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_systems(
                Update,
                (
                    follow_player,
                    move_and_shoot_ai,
                    death_loop,
                    spread_enemies,
                    spawn_loop,
                    beholder_update,
                    reaper_update,
                    reaper_blade_update,

                    spawn_imp,
                    spawn_imp_queen,
                    spawn_beholder,
                    spawn_beholder_prince,
                    spawn_reaper,
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnEnter(GameState::Playing), spawn_spawn_rng)
            .add_animation::<ImpAnimation>()
            .add_animation::<BeholderAnimation>()
            .add_animation::<BeholderProjectileAnimation>()
            .add_animation::<ReaperAnimation>()
            .add_animation::<ReaperBladeAnimation>()
            .add_event::<EnemySpawnEvent>()
            .add_event::<ShootEvent>()
            .add_event::<ChargeShootEvent>()
            .insert_resource(SpawnInfo {
                timer: Timer::from_seconds(3., TimerMode::Repeating),
                game: Stopwatch::new(),
                count: 0,
            }).add_plugins(SpawnMenuPlugin);
    }
}
