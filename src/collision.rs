use bevy::{prelude::*, utils::HashSet};

pub mod collider;

use self::collider::{collision_tick, IsCollidingEvent, PreviousCollisions, CollisionStartEvent, CollisionEndEvent};
use crate::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_tick.run_if(in_state(GameState::Playing)))
            .add_event::<IsCollidingEvent>()
            .add_event::<CollisionStartEvent>()
            .add_event::<CollisionEndEvent>()
            .insert_resource(PreviousCollisions {
                collisions: HashSet::new(),
            });
    }
}
