use bevy::{prelude::*, transform};
use std::time::Duration;
use crate::enemies::enemy::Enemy;
use crate::cooldown::{Cooldown, self};

struct TowerStats {
    pub range : f32,
    pub cooldown : Duration,
}

impl TowerStats {
    pub fn fire_rate(&self) -> f32 {
        1. / self.cooldown.as_secs_f32()
    }
}

#[derive(Component)]
struct Tower {
    pub stats : TowerStats,
}

fn tower_trigger (
    mut towers: Query<(Entity, &mut Tower, &mut Transform, &mut Cooldown)>,
    mut enemies: Query<(Entity, &mut Enemy, &mut Transform)>,
) {
    for (_, tower, tower_transform, mut tower_cooldown) in towers.iter_mut() {
        if !tower_cooldown.is_ready() {
            continue;
        }
        
        for (_, enemy, enemy_transform) in enemies.iter_mut() {
            if !tower_cooldown.is_ready() {
                continue;
            }
            
            if tower_transform.translation.distance(enemy_transform.translation) <= tower.stats.range {
                // tower can shoot
                tower_cooldown.time_remaining += tower.stats.cooldown;
                
            }
        }
    }
}