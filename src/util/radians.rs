use std::{
    f32::consts::PI,
    ops::{Add, Div, Mul, Sub},
};

use bevy::prelude::Vec2;

#[derive(Clone, Copy)]
pub struct Radian {
    pub angle: f32,
}

impl Radian {
    pub const ZERO: Radian = Radian { angle: 0. };
    pub const HALF: Radian = Radian { angle: PI };
    pub const FULL: Radian = Radian { angle: PI * 2. };

    pub fn from_degrees(degrees: f32) -> Radian {
        Radian::FULL * (degrees / 360.)
    }

    pub fn to_degrees(&self) -> f32 {
        self.angle / Radian::FULL.angle * 360.
    }

    pub fn normalize(self) -> Radian {
        let mut angle = self.angle;

        while angle > Radian::FULL.angle {
            angle -= Radian::FULL.angle;
        }

        while angle < 0. {
            angle += Radian::FULL.angle;
        }

        Radian { angle }
    }

    pub fn normalize_to_half(self) -> Radian {
        let mut angle = self.angle;

        while angle > Radian::HALF.angle {
            angle -= Radian::FULL.angle;
        }

        while angle < Radian::HALF.angle * -1. {
            angle += Radian::FULL.angle;
        }

        Radian { angle }
    }

    pub fn abs(self) -> Radian {
        Radian {
            angle: self.angle.abs(),
        }
    }

    pub fn unit_vector(&self) -> Vec2 {
        Vec2 {
            x: -self.angle.sin(),
            y: self.angle.cos(),
        }
    }
}

impl From<f32> for Radian {
    fn from(value: f32) -> Self {
        Radian { angle: value }
    }
}

impl Add for Radian {
    type Output = Radian;

    fn add(self, rhs: Self) -> Self::Output {
        Radian {
            angle: self.angle + rhs.angle,
        }
        .normalize()
    }
}

impl Sub for Radian {
    type Output = Radian;

    fn sub(self, rhs: Self) -> Self::Output {
        Radian {
            angle: self.angle - rhs.angle,
        }
        .normalize()
    }
}

impl Mul<f32> for Radian {
    type Output = Radian;

    fn mul(self, rhs: f32) -> Self::Output {
        Radian {
            angle: self.angle * rhs,
        }
        .normalize()
    }
}

impl Div<f32> for Radian {
    type Output = Radian;

    fn div(self, rhs: f32) -> Self::Output {
        Radian {
            angle: self.angle / rhs,
        }
        .normalize()
    }
}
