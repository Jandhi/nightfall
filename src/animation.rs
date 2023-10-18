
use std::{time::Duration, hash::Hash};

use bevy::{prelude::*, utils::HashMap};

use crate::{GameState, constants::SCALING_VEC3};

/*
The component used to control a spritesheet's animation
*/
#[derive(Component)]
pub struct AnimationController<T : Clone + Copy> {
    state : AnimationStateInfo<T>,
    is_facing_right : bool,
}

impl<T : Copy> AnimationController<T> {
    pub fn is_facing_right(&self) -> bool {
        self.is_facing_right
    }

    pub fn get_state(&self) -> T {
        self.state.id
    }

    pub fn set_facing_right(&mut self, is_facing_right : bool) {
        self.is_facing_right = is_facing_right;
    }

    pub fn new(start_state : AnimationStateInfo<T>) -> AnimationController<T> {
        AnimationController { state: start_state, is_facing_right: true }
    }
}

#[derive(Event)]
pub struct AnimationStateChangeEvent<T>{
    pub id : Entity,
    pub state_id : T, 
}

#[derive(Clone, Copy)]
pub struct AnimationStateInfo<T : Clone + Copy> {
    pub id : T,
    pub start_index : usize,
    pub frames : usize,
    pub frame_duration : Duration,
}

#[derive(Resource)]
pub struct AnimationStateStorage<T : Clone + Copy> {
    pub states : HashMap<T, AnimationStateInfo<T>>,
    pub size : usize,
}

impl<T : Eq + Hash + Clone + Copy> AnimationStateStorage<T> {
    pub fn get(&self, id : T) -> Option<AnimationStateInfo<T>> {
        if self.states.contains_key(&id) {
            Some(self.states[&id])
        } else {
            None
        }
    }
}

impl<T : Eq + Hash + Clone + Copy> PartialEq for AnimationStateInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);


/*
Allows for easier setup of animation systems
*/
pub trait AppAnimationSetup {
    fn add_animation<T : Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(&mut self, states : Vec<AnimationStateInfo<T>>);
}
impl AppAnimationSetup for App {
    fn add_animation<T : Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(&mut self, states : Vec<AnimationStateInfo<T>>) {
        self.add_systems(Update, update_animation_frames::<T>.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_animation_state::<T>.run_if(in_state(GameState::Playing)))
            .add_event::<AnimationStateChangeEvent<T>>()
            .insert_resource(AnimationStateStorage::<T>{
                states: HashMap::from_iter(states.iter().map(|state| (state.id , state.clone()))),
                size: states.iter().fold(0, |acc, state| acc + state.frames)
            });
    }
}

/*
Picks up on animation state change events and updates the corresponding sprites
*/
pub fn update_animation_state<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(
    animation_storage : Res<AnimationStateStorage<T>>,
    mut animation_changes : EventReader<AnimationStateChangeEvent<T>>,
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
            
            controller.state = animation_storage.states[&change_event.state_id].clone();
            timer.set_duration(controller.state.frame_duration);
            timer.set_elapsed(Duration::ZERO);
            atlas.index = controller.state.start_index;
            atlas.flip_x = !controller.is_facing_right;
        }
    }
}

pub fn make_animation_bundle<T: Send + std::marker::Sync + 'static + Clone + Copy + Eq + Hash>(
    start_state_id : T,
    animations : Res<AnimationStateStorage<T>>,
    texture_atlas_handle : Handle<TextureAtlas>,
) -> impl Bundle {
    let start_state = animations.get(start_state_id).unwrap();
    (
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(start_state.start_index),
            transform: Transform { translation: Vec3::new(0., 0., 1.), rotation: Quat::IDENTITY, scale: SCALING_VEC3 },
            ..Default::default()
        },
        AnimationTimer(Timer::from_seconds(start_state.frame_duration.as_secs_f32(), TimerMode::Repeating)),
        AnimationController::new(start_state)
    )
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
            
            sprite.index = if sprite.index >= controller.state.start_index + controller.state.frames - 1 {
                controller.state.start_index
            } else {
                sprite.index + 1
            };

            sprite.flip_x = !controller.is_facing_right;
        }
    }
}