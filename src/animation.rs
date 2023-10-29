use std::{hash::Hash, time::Duration};

use bevy::{prelude::*, utils::HashMap};

use crate::{constants::SCALING_VEC3, GameState};

use self::{controller::{AnimationController, AnimationTimer, update_animation_frames, update_animation_state}, info::AnimationStateInfo};

pub mod controller;
pub mod info;

#[derive(Event)]
pub struct AnimationStateChangeEvent<TState> {
    pub id: Entity,
    pub state_id: TState,
}

pub trait Animation<TState: Clone + Copy> {
    fn get_states() -> Vec<AnimationStateInfo<TState>>;
}

#[derive(Resource)]
pub struct AnimationStateStorage<T: Clone + Copy> {
    pub states: HashMap<T, AnimationStateInfo<T>>,
    pub size: usize,
}

impl<T: Eq + Hash + Clone + Copy> AnimationStateStorage<T> {
    pub fn get(&self, id: T) -> Option<AnimationStateInfo<T>> {
        if self.states.contains_key(&id) {
            Some(self.states[&id])
        } else {
            None
        }
    }
}

impl<T: Eq + Hash + Clone + Copy> PartialEq for AnimationStateInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}



/*
Allows for easier setup of animation systems
*/
pub trait AppAnimationSetup {
    fn add_animation<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash + Animation<T>>(
        &mut self,
    ) -> &mut Self;
}
impl AppAnimationSetup for App {
    fn add_animation<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash + Animation<T>>(
        &mut self,
    ) -> &mut Self {
        let states = T::get_states();
        self.add_systems(
            Update,
            (
                update_animation_frames::<T>,
                update_animation_state::<T>
            ).run_if(in_state(GameState::Playing)),
        )
        .add_event::<AnimationStateChangeEvent<T>>()
        .insert_resource(AnimationStateStorage::<T> {
            states: HashMap::from_iter(states.iter().map(|state| (state.id, *state))),
            size: states.iter().fold(0, |acc, state| acc + state.frames),
        });
        self
    }
}



pub fn animation_bundle<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(
    start_state_id: T,
    animations: &Res<AnimationStateStorage<T>>,
    texture_atlas_handle: Handle<TextureAtlas>,
    position: Vec3,
) -> impl Bundle {
    let start_state = animations.get(start_state_id).unwrap();
    (
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(start_state.start_index),
            transform: Transform {
                translation: position,
                rotation: Quat::IDENTITY,
                scale: SCALING_VEC3,
            },
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(
            start_state.frame_duration.as_secs_f32(),
            TimerMode::Repeating,
        )),
        AnimationController::new(start_state),
    )
}


