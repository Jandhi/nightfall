use bevy::prelude::Vec3;

pub const DISTANCE_SCALING: f32 = 2.;
pub const SCALING_VEC3: Vec3 = Vec3 {
    x: DISTANCE_SCALING,
    y: DISTANCE_SCALING,
    z: 1.,
};

pub const IS_DEBUG: bool = true;

pub enum SortingLayers {
    UI,
    Front,
    Player,
    Action,
}

impl Into<f32> for SortingLayers {
    fn into(self) -> f32 {
        match self {
            SortingLayers::UI => 10.,
            SortingLayers::Front => 6.,
            SortingLayers::Player => 5.,
            SortingLayers::Action => 3.,
        }
    }
}
