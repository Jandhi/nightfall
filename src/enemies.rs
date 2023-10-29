

use bevy::prelude::*;

use crate::{
    animation::{AppAnimationSetup},
    GameState,
};

use self::enemy::{death_loop, follow_player, spawn_enemy, EnemyDeathEvent, ImpAnimation, spread_enemies};

pub mod enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyDeathEvent>()
            .add_systems(Update, (
                follow_player,
                death_loop,
                spread_enemies,
            ).run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_animation::<ImpAnimation>();
    }
}
