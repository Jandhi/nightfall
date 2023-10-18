use std::time::Duration;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

#[derive(Component)]
pub struct AnimationController {
    pub state : AnimationState,
    pub is_facing_right : bool,
}

#[derive(Clone, Copy, PartialEq)]
pub struct AnimationState {
    pub start_index : usize,
    pub frames : usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationController,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (controller, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index >= controller.state.start_index + controller.state.frames - 1 {
                controller.state.start_index
            } else {
                sprite.index + 1
            };

            if controller.is_facing_right {
                sprite.flip_x = false;   
            } else {
                sprite.flip_x = true;
            }
        }
    }
}