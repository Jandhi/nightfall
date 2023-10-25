use bevy::{prelude::*, utils::HashSet};

pub mod collider;

use self::collider::{collision_tick, IsCollidingEvent, PreviousCollisions};
use crate::GameState;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_tick.run_if(in_state(GameState::Playing)))
            .add_event::<IsCollidingEvent>()
            .insert_resource(PreviousCollisions {
                collisions: HashSet::new(),
            });
    }
}
