use bevy::prelude::Vec3;

pub const DISTANCE_SCALING : f32 = 2.;
pub const SCALING_VEC3 : Vec3 = Vec3{
    x: DISTANCE_SCALING,
    y: DISTANCE_SCALING,
    z: 1.
};

pub const IS_DEBUG : bool = true;