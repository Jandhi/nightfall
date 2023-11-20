use bevy::{prelude::*, utils::HashSet};
use bevy_debug_text_overlay::screen_print;
use std::collections::HashMap;

type SpatialCoord = (i32, i32);

#[derive(Component, Clone, Debug)]
pub struct Collider {
    shape: ColliderShape,
    spatial_coord: SpatialCoord,
    initialized: bool,
}

impl Collider {
    pub fn shape(&self) -> ColliderShape {
        self.shape
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ColliderShape {
    Rect(Vec2),
    Circle(f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Collision {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl Collision {
    pub fn contains(&self, entity: Entity) -> bool {
        entity == self.entity_a || entity == self.entity_b
    }
}

#[derive(Event)]
pub struct IsCollidingEvent {
    pub collision: Collision,
}

#[derive(Event)]
pub struct CollisionStartEvent {
    pub collision: Collision,
}

#[derive(Event)]
pub struct CollisionEndEvent {
    pub collision: Collision,
}

// Checks if vector b is between a and c
fn is_between(a: Vec2, b: Vec2, c: Vec2) -> bool {
    ((a.x >= b.x && b.x >= c.x) && (a.y >= b.y && b.y >= c.y))
        || ((a.x >= b.x && b.x >= c.x) && (c.y >= b.y && b.y >= a.y))
        || ((c.x >= b.x && b.x >= a.x) && (a.y >= b.y && b.y >= c.y))
        || ((c.x >= b.x && b.x >= a.x) && (c.y >= b.y && b.y >= a.y))
}

impl Collider {
    pub fn new_rect(size: Vec2) -> Collider {
        Collider {
            shape: ColliderShape::Rect(size),
            spatial_coord: default(),
            initialized: false,
        }
    }

    pub fn new_circle(radius: f32) -> Collider {
        Collider {
            shape: ColliderShape::Circle(radius),
            spatial_coord: default(),
            initialized: false,
        }
    }

    pub fn min_point(&self, position: Vec2) -> Vec2 {
        match self.shape {
            ColliderShape::Rect(size) => position - size / 2.,
            ColliderShape::Circle(radius) => {
                position
                    - Vec2 {
                        x: radius,
                        y: radius,
                    }
            }
        }
    }

    pub fn max_point(&self, position: Vec2) -> Vec2 {
        match self.shape {
            ColliderShape::Rect(size) => position + size / 2.,
            ColliderShape::Circle(radius) => {
                position
                    + Vec2 {
                        x: radius,
                        y: radius,
                    }
            }
        }
    }

    pub fn contains_point(&self, my_position: Vec2, point: Vec2) -> bool {
        if point.x.is_nan() || point.y.is_nan() {
            return false;
        }

        match self.shape {
            ColliderShape::Rect(size) => {
                point.x >= my_position.x - size.x / 2.
                    && point.x <= my_position.x + size.x / 2.
                    && point.y >= my_position.y - size.y / 2.
                    && point.y <= my_position.y + size.y / 2.
            }
            ColliderShape::Circle(radius) => my_position.distance(point) <= radius,
        }
    }

    pub fn is_colliding(&self, position: Vec2, other: &Collider, other_position: Vec2) -> bool {
        match (&self.shape, &other.shape) {
            (ColliderShape::Rect(_), ColliderShape::Rect(_)) => {
                // The shapes collide if the min or max point is between the min and max point of the other collider
                is_between(
                    self.max_point(position),
                    other.max_point(other_position),
                    self.min_point(position),
                ) || is_between(
                    self.max_point(position),
                    other.min_point(other_position),
                    self.min_point(position),
                )
            }
            (ColliderShape::Rect(size), ColliderShape::Circle(other_radius)) => {
                Collider::is_colliding_rect_circle(*size, position, *other_radius, other_position)
            }
            (ColliderShape::Circle(radius), ColliderShape::Rect(other_size)) => {
                Collider::is_colliding_rect_circle(*other_size, other_position, *radius, position)
            }
            (ColliderShape::Circle(radius), ColliderShape::Circle(other_radius)) => {
                position.distance(other_position) < radius + other_radius
            }
        }
    }

    fn is_colliding_rect_circle(size: Vec2, rect_pos: Vec2, radius: f32, circle_pos: Vec2) -> bool {
        let bottom_left = rect_pos - size / 2.;
        let top_right = rect_pos + size / 2.;
        let top_left = Vec2 {
            x: bottom_left.x,
            y: top_right.y,
        };
        let bottom_right = Vec2 {
            x: top_right.x,
            y: bottom_left.y,
        };
        let corners = vec![bottom_left, top_right, top_left, bottom_right];

        // Trivial case: if the center of the circle is in the rectangle there is a collision
        if is_between(rect_pos + size / 2., circle_pos, rect_pos - size / 2.) {
            true
        // If it contains corners
        } else if corners
            .iter()
            .any(|corner| circle_pos.distance(*corner) <= radius)
        {
            true
        // Otherwise, start with case where circle is to the left
        } else if circle_pos.x < rect_pos.x
            && circle_pos.y > bottom_left.y
            && circle_pos.y < top_right.y
        {
            bottom_left.x - circle_pos.x <= radius
        // To the right
        } else if circle_pos.x > rect_pos.x
            && circle_pos.y > bottom_left.y
            && circle_pos.y < top_right.y
        {
            circle_pos.x - top_right.x <= radius
        // Above
        } else if circle_pos.y > rect_pos.y
            && circle_pos.x > bottom_left.x
            && circle_pos.x < top_right.x
        {
            circle_pos.y - top_right.y <= radius
        // Below
        } else if circle_pos.y < rect_pos.y
            && circle_pos.x > bottom_left.x
            && circle_pos.x < top_right.x
        {
            bottom_left.y - circle_pos.y <= radius
        } else {
            false
        }
    }
}

const SPATIAL_GRID_SIZE: f32 = 100.;

pub fn vec3_to_spatial_coord(translation: Vec3) -> SpatialCoord {
    let vec = translation.truncate() / SPATIAL_GRID_SIZE;
    (vec.x.floor() as i32, vec.y.floor() as i32)
}

pub fn vec2_to_spatial_coord(translation: Vec2) -> SpatialCoord {
    let vec = translation / SPATIAL_GRID_SIZE;
    (vec.x.floor() as i32, vec.y.floor() as i32)
}

#[derive(Resource)]
pub struct PreviousCollisions {
    pub collisions: HashSet<(Entity, Entity)>,
}

pub fn collision_tick(
    mut q_colliders: Query<(Entity, &mut Collider, &GlobalTransform)>,
    mut collision_started_event: EventWriter<CollisionStartEvent>,
    mut collision_event: EventWriter<IsCollidingEvent>,
    mut collision_ended_event: EventWriter<CollisionEndEvent>,
    mut prev_collisions: ResMut<PreviousCollisions>,
) {
    let collisions: HashSet<(Entity, Entity)> = HashSet::new();
    let mut spatial_grid = HashMap::new();

    // Update spatial grid
    for (entity, mut collider, transform) in q_colliders.iter_mut() {
        let spatial_coord = vec3_to_spatial_coord(transform.translation());

        spatial_grid
            .entry(spatial_coord)
            .or_insert_with(std::vec::Vec::new);

        // Add entity to updated coordinate
        spatial_grid.get_mut(&spatial_coord).unwrap().push((
            entity,
            collider.clone(),
            transform.translation().truncate(),
        ));
        collider.spatial_coord = spatial_coord;
        collider.initialized = true;
    }

    // Find collisions
    for (entity, collider, transform) in q_colliders.iter() {
        let (min_x, min_y) =
            vec2_to_spatial_coord(collider.min_point(transform.translation().truncate()));
        let (max_x, max_y) =
            vec2_to_spatial_coord(collider.max_point(transform.translation().truncate()));

        let mut possible_collisions = vec![];

        // Add all possible collisions
        for x in min_x..max_x + 1 {
            for y in min_y..max_y + 1 {
                if let Some(entities) = spatial_grid.get(&(x, y)) {
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

            if collider.is_colliding(
                transform.translation().truncate(),
                other_collider,
                *other_position,
            ) {
                if collisions.contains(&(*other_entity, entity)) {
                    continue; // Already logged collision
                } else {
                    let collision = Collision {
                        entity_a: entity,
                        entity_b: *other_entity,
                    };

                    let previously_collided = prev_collisions
                        .collisions
                        .contains(&(entity, *other_entity))
                        || prev_collisions
                            .collisions
                            .contains(&(*other_entity, entity));

                    if !previously_collided {
                        collision_started_event.send(CollisionStartEvent { collision })
                    }

                    collision_event.send(IsCollidingEvent { collision });
                }
            }
        }
    }

    // Collision Ending
    for collision in &prev_collisions.collisions {
        let (a, b) = *collision;
        let other_collision = &(b, a);

        if collisions.contains(collision) || collisions.contains(other_collision) {
            continue;
        }

        collision_ended_event.send(CollisionEndEvent {
            collision: Collision {
                entity_a: a,
                entity_b: b,
            },
        });
    }

    prev_collisions.collisions = collisions;
}
