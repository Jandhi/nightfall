use std::time::Duration;

use crate::animation::{
    info::{AnimationInfoBuilder, AnimationStateInfo},
    Animation, AnimationStateStorage,
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerAnimationState {
    Idle,
    Running,
}

impl Animation<PlayerAnimationState> for PlayerAnimationState {
    fn get_states() -> Vec<AnimationStateInfo<PlayerAnimationState>> {
        AnimationInfoBuilder::new()
            .add_frames(
                PlayerAnimationState::Idle,
                2,
                Duration::from_secs_f32(1. / 2.),
            )
            .add_frames(
                PlayerAnimationState::Running,
                4,
                Duration::from_secs_f32(1. / 10.),
            )
            .build()
    }
}

pub type PlayerAnimations = AnimationStateStorage<PlayerAnimationState>;
