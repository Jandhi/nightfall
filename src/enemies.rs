use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::{AnimationStateInfo, AppAnimationSetup},
    GameState,
};

use self::enemy::{death_loop, follow_player, spawn_enemy, EnemyDeathEvent, ImpAnimationState};

pub mod enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, death_loop.run_if(in_state(GameState::Playing)))
            .add_event::<EnemyDeathEvent>()
            .add_systems(Update, follow_player.run_if(in_state(GameState::Playing)))
            .add_systems(OnEnter(GameState::Playing), spawn_enemy)
            .add_animation(vec![AnimationStateInfo {
                id: ImpAnimationState::FLYING,
                start_index: 0,
                frames: 4,
                frame_duration: Duration::from_secs_f32(1. / 8.),
            }]);
    }
}
