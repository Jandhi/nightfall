use bevy::prelude::*;

use crate::{enemies::enemy::Enemy, combat::health::Health};

pub enum Targeting {
    First,
    Last,
    MostHealth,
    LeastHealth
}

#[derive(Clone)]
pub struct Target {
    pub entity : Entity,
    pub enemy : Enemy,
    pub transform : Transform,
    pub health : Health
}

impl Targeting {
    pub fn find_best_target<'a>(&self, 
        enemies : &'a Vec<Target>) -> Option<Target> {
        if enemies.len() == 0 {
            None
        } else {
            Some(
                match self {
                    Targeting::First => {
                        enemies.iter()
                            .max_by_key(|target| (target.enemy.track_progress * 100000.) as i32)
                            .unwrap()
                            .clone()
                    },
                    Targeting::Last => {
                        enemies.iter()
                            .min_by_key(|target| (target.enemy.track_progress * 100000.) as i32)
                            .unwrap()
                            .clone()
                    },
                    Targeting::MostHealth => {
                        enemies.iter()
                            .max_by_key(|target| target.health.value)
                            .unwrap()
                            .clone()
                    },
                    Targeting::LeastHealth => {
                        enemies.iter()
                            .min_by_key(|target| target.health.value)
                            .unwrap()
                            .clone()
                    },
                }
            )
        }
    }
}