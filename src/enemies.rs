

use bevy::prelude::*;

use crate::{
    animation::AppAnimationSetup,
    GameState,
};

use self::{enemy::{death_loop, initial_spawn, EnemyDeathEvent, spread_enemies}, spawning::{spawn_loop, SpawnInfo}, ai::{follow_player, move_and_shoot_ai, ShootEvent, ChargeShootEvent}, imp::ImpAnimation, beholder::{BeholderAnimation, beholder_update, BeholderProjectileAnimation}};

pub mod enemy;
pub mod spawning;
pub mod ai;
pub mod imp;
pub mod zombie;
pub mod beholder;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_systems(Update, (
                follow_player,
                move_and_shoot_ai,
                death_loop,
                spread_enemies,
                spawn_loop,
                beholder_update,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), initial_spawn)
            .add_animation::<ImpAnimation>()
            .add_animation::<BeholderAnimation>()
            .add_animation::<BeholderProjectileAnimation>()
            .add_event::<ShootEvent>()
            .add_event::<ChargeShootEvent>()
            .insert_resource(SpawnInfo{ timer: Timer::from_seconds(6., TimerMode::Repeating), count: 0 });
    }
}
