use bevy::{prelude::*, utils::HashSet};

pub mod collider;
pub mod collider_debug;
pub mod collider_debug_scene;

use self::{
    collider::{
        collision_tick, CollisionEndEvent, CollisionStartEvent, IsCollidingEvent,
        PreviousCollisions,
    },
    collider_debug::{
        despawn_colliders_sprites, spawn_colliders_sprites, update_collider_sprites,
        ColliderDebugSpriteState,
    },
    collider_debug_scene::enter_debug_scene,
};
use crate::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ColliderDebugSpriteState>()
            .add_systems(
                OnEnter(ColliderDebugSpriteState::On),
                spawn_colliders_sprites,
            )
            .add_systems(
                OnEnter(ColliderDebugSpriteState::Off),
                despawn_colliders_sprites,
            )
            .add_systems(Update, collision_tick)
            .add_systems(
                Update,
                update_collider_sprites.run_if(in_state(ColliderDebugSpriteState::On)),
            )
            .add_systems(OnEnter(GameState::DebugCollision), enter_debug_scene)
            .add_event::<IsCollidingEvent>()
            .add_event::<CollisionStartEvent>()
            .add_event::<CollisionEndEvent>()
            .insert_resource(PreviousCollisions {
                collisions: HashSet::new(),
            });
    }
}
