use std::{hash::Hash, time::Duration};

use bevy::prelude::*;

use super::{AnimationStateInfo, AnimationStateStorage, AnimationStateChangeEvent};

/*
The component used to control a spritesheet's animation
*/
#[derive(Component)]
pub struct AnimationController<TState: Clone + Copy> {
    state: AnimationStateInfo<TState>,
    is_facing_right: bool,
}


impl<TState: Copy> AnimationController<TState> {
    pub fn is_facing_right(&self) -> bool {
        self.is_facing_right
    }

    pub fn get_state(&self) -> TState {
        self.state.id
    }

    pub fn set_facing_right(&mut self, is_facing_right: bool) {
        self.is_facing_right = is_facing_right;
    }

    pub fn new(start_state: AnimationStateInfo<TState>) -> AnimationController<TState> {
        AnimationController {
            state: start_state,
            is_facing_right: true,
        }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/*
Picks up on animation state change events and updates the corresponding sprites
*/
pub fn update_animation_state<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(
    animation_storage: Res<AnimationStateStorage<T>>,
    mut animation_changes: EventReader<AnimationStateChangeEvent<T>>,
    mut query: Query<(
        &mut AnimationController<T>,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for change_event in animation_changes.iter() {
        if let Ok((mut controller, mut timer, mut atlas)) = query.get_mut(change_event.id) {
            // Already in state
            if controller.state.id == change_event.state_id {
                return;
            }

            controller.state = animation_storage.states[&change_event.state_id];
            timer.set_duration(controller.state.frame_duration);
            timer.set_elapsed(Duration::ZERO);
            atlas.index = controller.state.start_index;
            atlas.flip_x = !controller.is_facing_right;
        }
    }
}

/*
Does the animation on each sprite
*/
pub fn update_animation_frames<T: Send + std::marker::Sync + 'static + Clone + Copy>(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationController<T>,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (controller, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());

        if timer.just_finished() {
            sprite.index =
                if sprite.index >= controller.state.start_index + controller.state.frames - 1 {
                    controller.state.start_index
                } else {
                    sprite.index + 1
                };

            sprite.flip_x = !controller.is_facing_right;
        }
    }
}