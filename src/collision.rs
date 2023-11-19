use bevy::{prelude::*, utils::HashSet, ecs::schedule::ScheduleLabel};

pub mod collider;
pub mod collider_debug;

use self::{collider::{
    collision_tick, CollisionEndEvent, CollisionStartEvent, IsCollidingEvent, PreviousCollisions,
}, collider_debug::{ColliderDebugSpriteState, update_collider_sprites, spawn_colliders_sprites, despawn_colliders_sprites}};
use crate::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ColliderDebugSpriteState>()
            .add_systems(OnEnter(ColliderDebugSpriteState::On), spawn_colliders_sprites)
            .add_systems(OnEnter(ColliderDebugSpriteState::Off), despawn_colliders_sprites)
            .add_systems(Update, collision_tick.run_if(in_state(GameState::Playing)))
            .add_systems(Update, update_collider_sprites
                .run_if(in_state(GameState::Playing))
                .run_if(in_state(ColliderDebugSpriteState::On))
            )
            .add_event::<IsCollidingEvent>()
            .add_event::<CollisionStartEvent>()
            .add_event::<CollisionEndEvent>()
            .insert_resource(PreviousCollisions {
                collisions: HashSet::new(),
            });
    }
}