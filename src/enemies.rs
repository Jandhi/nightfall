use bevy::{prelude::*, time::Stopwatch};

use crate::{animation::AppAnimationSetup, GameState};

use self::{
    ai::{follow_player, move_and_shoot_ai, ChargeShootEvent, ShootEvent},
    beholder::{beholder_update, BeholderAnimation, BeholderProjectileAnimation},
    enemy::{death_loop, initial_spawn, spread_enemies, EnemyDeathEvent},
    imp::ImpAnimation,
    reaper::{reaper_blade_update, reaper_update, ReaperAnimation, ReaperBladeAnimation},
    spawning::{spawn_loop, spawn_spawn_rng, SpawnInfo},
};

pub mod ai;
pub mod beholder;
pub mod enemy;
pub mod imp;
pub mod reaper;
pub mod spawning;
pub mod zombie;

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
                )
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                OnEnter(GameState::Playing),
                (initial_spawn, spawn_spawn_rng),
            )
            .add_animation::<ImpAnimation>()
            .add_animation::<BeholderAnimation>()
            .add_animation::<BeholderProjectileAnimation>()
            .add_animation::<ReaperAnimation>()
            .add_animation::<ReaperBladeAnimation>()
            .add_event::<ShootEvent>()
            .add_event::<ChargeShootEvent>()
            .insert_resource(SpawnInfo {
                timer: Timer::from_seconds(3., TimerMode::Repeating),
                game: Stopwatch::new(),
                count: 0,
            });
    }
}
