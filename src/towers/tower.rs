use crate::collision::collider::Collider;
use crate::cooldown::Cooldown;
use crate::enemies::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::towers::turret::Turret;
use bevy::prelude::*;
use std::time::Duration;

pub struct TowerStats {
    pub range: f32,
    pub cooldown: Duration,
}

impl TowerStats {
    pub fn fire_rate(&self) -> f32 {
        1. / self.cooldown.as_secs_f32()
    }
}

#[derive(Component)]
pub struct Tower {
    pub stats: TowerStats,
    pub rotation: Quat,
}

pub fn tower_trigger(
    mut towers: Query<(Entity, &mut Tower, &mut Transform, &mut Cooldown), Without<Enemy>>,
    mut enemies: Query<(Entity, &mut Enemy, &mut Transform), Without<Tower>>,
) {
    for (_, tower, tower_transform, mut tower_cooldown) in towers.iter_mut() {
        if !tower_cooldown.is_ready() {
            continue;
        }

        for (_, _, enemy_transform) in enemies.iter_mut() {
            if !tower_cooldown.is_ready() {
                continue;
            }

            if tower_transform
                .translation
                .distance(enemy_transform.translation)
                <= tower.stats.range
            {
                // tower can shoot
                tower_cooldown.time_remaining += tower.stats.cooldown;
            }
        }
    }
}

pub fn spawn_tower(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.texture_tower.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                rotation: Quat::IDENTITY,
                scale: 2.
                    * Vec3 {
                        x: 1.,
                        y: 1.,
                        z: 1.,
                    },
            },
            ..Default::default()
        })
        // Collider
        .insert(Collider::new_rect(
            Vec2{x:100., y: 100.}, 
            Vec2 { x: 0., y: 0. }
        ))
        // Tower
        .insert(Tower {
            stats: TowerStats {
                range: 1.,
                cooldown: Duration::from_secs_f32(1.),
            },
            rotation: Quat::default(),
        })
        .with_children(|parent| {
            parent
                .spawn(SpriteBundle {
                    texture: textures.texture_turret.clone(),
                    transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                    ..Default::default()
                })
                .insert(Turret);
        });
}
