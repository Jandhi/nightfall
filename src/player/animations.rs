use crate::animation::AnimationStateStorage;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerAnimationState {
    Idle,
    Running
}

pub type PlayerAnimations = AnimationStateStorage<PlayerAnimationState>;