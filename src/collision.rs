use bevy::{prelude::*, utils::HashSet};
use std::{convert, collections::HashMap, fmt::Display};


use crate::GameState;

#[derive(Component, Clone)]
pub struct Collider {
    size : Vec2,
    spatial_coord : (i32, i32),
}

impl Collider {
    pub fn new(size : Vec2, position : Vec2) -> Collider {
        return Collider { size: size, spatial_coord: SpatialGrid::vec2_to_spatial_coord(position) }
    }

    pub fn min(&self, position : Vec2) -> Vec2 {
        return position - self.size / 2.;
    }
    
    pub fn max(&self, position : Vec2) -> Vec2 {
        return position + self.size / 2.;
    }
}

#[derive(Event)]
struct CollisionEvent {
    entity_a : Entity,
    entity_b : Entity
}


const SPATIAL_GRID_SIZE : f32 = 100.;

#[derive(Resource)]
struct SpatialGrid {
    pub segments : HashMap<(i32, i32), Vec<(Entity, Collider, Vec2)>>,
}

impl SpatialGrid {
    pub fn vec3_to_spatial_coord(translation : Vec3) -> (i32, i32) {
        let vec = translation.truncate() / SPATIAL_GRID_SIZE;
        return (vec.x.floor() as i32, vec.y.floor() as i32)
    }

    pub fn vec2_to_spatial_coord(translation : Vec2) -> (i32, i32) {
        let vec = translation / SPATIAL_GRID_SIZE;
        return (vec.x.floor() as i32, vec.y.floor() as i32)
    }
}

impl Display for SpatialGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Spatial Grid: {}", self.segments.len())
    }
}

// Checks if vector b is between a and c
fn is_between(
    a : Vec2,
    b : Vec2,
    c : Vec2,
) -> bool {
    ((a.x >= b.x && b.x >= c.x) && (a.y >= b.y && b.y >= c.y)) ||
        ((a.x >= b.x && b.x >= c.x) && (c.y >= b.y && b.y >= a.y)) ||
        ((c.x >= b.x && b.x >= a.x) && (a.y >= b.y && b.y >= c.y)) ||
        ((c.x >= b.x && b.x >= a.x) && (c.y >= b.y && b.y >= a.y))
}

fn is_colliding(
    collider_a : &Collider,
    collider_b : &Collider,
    position_a : &Vec2,
    position_b : &Vec2
) -> bool {
    return is_between(collider_a.max(*position_a), collider_b.max(*position_b), collider_a.min(*position_a)) || 
    is_between(collider_a.max(*position_a), collider_b.min(*position_b), collider_a.min(*position_a));
}

fn collision_tick (
    mut q_colliders : Query<(Entity, &mut Collider, &Transform)>,
    mut spatial_grid : ResMut<SpatialGrid>,
    mut collision_event : EventWriter<CollisionEvent>,
) {
    let collisions : HashSet<(Entity, Entity)> = HashSet::new();
    spatial_grid.segments = HashMap::new();

    // Update spatial grid
    for (entity, mut collider, transform) in q_colliders.iter_mut() {
        let spatial_coord = SpatialGrid::vec3_to_spatial_coord(transform.translation);

        if !spatial_grid.segments.contains_key(&spatial_coord) {
            spatial_grid.segments.insert(spatial_coord, vec![]);
        }

        // Add entity to updated coordinate
        spatial_grid.segments
            .get_mut(&spatial_coord)
            .unwrap()
            .push((entity, collider.clone(), transform.translation.truncate()));
        collider.spatial_coord = spatial_coord;
    }

    // Find collisions
    for (entity, collider, transform) in q_colliders.iter() {
        let (min_x, min_y) = SpatialGrid::vec2_to_spatial_coord(
            collider.min(transform.translation.truncate()));
        let (max_x, max_y) = SpatialGrid::vec2_to_spatial_coord(
            collider.max(transform.translation.truncate()));

        let mut possible_collisions = vec![];

        // Add all possible collisions
        for x in min_x..max_x+1 {
            for y in min_y..max_y+1 {
                if let Some(entities) = spatial_grid.segments.get(&(x, y)) {
                    for e in entities {
                        possible_collisions.push(e);
                    }
                }
            }
        }

        for (other_entity, other_collider, other_position) in possible_collisions.iter() {
            // No self collision
            if *other_entity == entity {
                continue;
            }

            if is_colliding(collider, other_collider, &transform.translation.truncate(), other_position) {
                if collisions.contains(&(other_entity.clone(), entity.clone())) {
                    continue; // Already logged collision
                } else {
                    collision_event.send(CollisionEvent { entity_a: entity, entity_b: *other_entity });
                }
            }
        }
    }

}

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_tick.run_if(in_state(GameState::Playing)))
            .add_event::<CollisionEvent>()
            .insert_resource(SpatialGrid{ segments :HashMap::new()});
    }
}