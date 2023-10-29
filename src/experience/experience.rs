use bevy::prelude::*;


#[derive(Component)]
pub struct Experience {
    pub curr_experience : u32,
    pub level : u32,
    pub xp_threshold : u32,
    pub xp_pickup_distance : f32,
}