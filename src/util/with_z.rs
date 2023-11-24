use bevy::prelude::*;

pub trait WithZ<TOut> {
    fn with_z(self, z : f32) -> TOut;
}

impl WithZ<Vec3> for Vec2 {
    fn with_z(self, z : f32) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: z }
    }
}

impl WithZ<Vec3> for Vec3 {
    fn with_z(self, z : f32) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
}

impl WithZ<Transform> for Transform {
    fn with_z(self, z : f32) -> Transform {
        Transform { translation: self.translation.with_z(z), rotation: self.rotation, scale: self.scale }
    }
}