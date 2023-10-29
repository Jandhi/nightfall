use std::time::Duration;

use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct AnimationStateInfo<TState: Clone + Copy> {
    pub id: TState,
    pub start_index: usize,
    pub frames: usize,
    pub frame_duration: Duration,
}

pub fn build_animation_state_info<TState: Clone + Copy>(info : Vec<(TState)>) -> Vec<AnimationStateInfo<TState>> {
    vec![]
}