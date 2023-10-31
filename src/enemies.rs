

use bevy::prelude::*;

use crate::{
    animation::AppAnimationSetup,
    GameState,
};

use self::{enemy::{death_loop, spawn_enemy, EnemyDeathEvent, ImpAnimation, spread_enemies}, spawning::{spawn_loop, SpawnInfo}, ai::follow_player};

pub mod enemy;
pub mod spawning;
pub mod ai;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_systems(Update, (
                follow_player,
                death_loop,
                spread_enemies,
                spawn_loop,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_animation::<ImpAnimation>()
            .insert_resource(SpawnInfo{ timer: Timer::from_seconds(6., TimerMode::Repeating), count: 0 });
    }
}
