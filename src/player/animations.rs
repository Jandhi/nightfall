use std::time::Duration;

use crate::animation::{AnimationStateStorage, Animation, info::AnimationStateInfo};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerAnimationState {
    Idle,
    Running,
}

impl Animation<PlayerAnimationState> for PlayerAnimationState {
    fn get_states() -> Vec<AnimationStateInfo<PlayerAnimationState>> {
        vec![
            AnimationStateInfo {
                id: PlayerAnimationState::Idle,
                start_index: 0,
                frames: 2,
                frame_duration: Duration::from_secs_f32(1. / 2.),
            },
            AnimationStateInfo {
                id: PlayerAnimationState::Running,
                start_index: 2,
                frames: 4,
                frame_duration: Duration::from_secs_f32(1. / 10.),
            },
        ]
    }
}

pub type PlayerAnimations = AnimationStateStorage<PlayerAnimationState>;
