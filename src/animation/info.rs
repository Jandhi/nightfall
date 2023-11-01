use std::time::Duration;

#[derive(Clone, Copy)]
pub struct AnimationStateInfo<TState: Clone + Copy> {
    pub id: TState,
    pub start_index: usize,
    pub frame_count: usize,
    pub frame_duration: Duration,
}

pub struct AnimationInfoBuilder<TState: Clone + Copy> {
    infos: Vec<AnimationStateInfoBlock<TState>>,
}

impl<TState: Clone + Copy> AnimationInfoBuilder<TState> {
    pub fn new() -> Self {
        Self { infos: vec![] }
    }

    pub fn add_single(&mut self, state: TState) -> &mut Self {
        self.infos.push(AnimationStateInfoBlock::Single(state));
        self
    }

    pub fn add_frames(
        &mut self,
        state: TState,
        frame_count: usize,
        duration: Duration,
    ) -> &mut Self {
        self.infos.push(AnimationStateInfoBlock::Frames(
            state,
            frame_count,
            duration,
        ));
        self
    }

    pub fn build(&self) -> Vec<AnimationStateInfo<TState>> {
        build_animation_state_info(&self.infos)
    }
}

#[derive(Clone, Copy)]
enum AnimationStateInfoBlock<TState: Clone + Copy> {
    Single(TState),
    Frames(TState, usize, Duration),
}

fn build_animation_state_info<TState: Clone + Copy>(
    blocks: &Vec<AnimationStateInfoBlock<TState>>,
) -> Vec<AnimationStateInfo<TState>> {
    let mut infos: Vec<AnimationStateInfo<TState>> = blocks
        .iter()
        .map(|block| match block {
            AnimationStateInfoBlock::Single(state) => AnimationStateInfo {
                id: *state,
                start_index: 0,
                frame_count: 1,
                frame_duration: Duration::ZERO,
            },
            AnimationStateInfoBlock::Frames(state, count, duration) => AnimationStateInfo {
                id: *state,
                start_index: 0,
                frame_count: *count,
                frame_duration: *duration,
            },
        })
        .collect();

    // fix indices
    let mut index = 0;
    for info in infos.iter_mut() {
        info.start_index = index;
        index += info.frame_count;
    }

    infos
}
