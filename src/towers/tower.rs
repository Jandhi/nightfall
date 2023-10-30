use crate::collision::collider::Collider;
use crate::combat::health::Health;
use crate::combat::projectile::{DamageTarget, PiercingMode, Projectile};
use crate::combat::teams::Team;
use crate::constants::{DISTANCE_SCALING, SCALING_VEC3};
use crate::cooldown::Cooldown;
use crate::enemies::enemy::Enemy;
use crate::loading::TextureAssets;
use crate::movement::velocity::Velocity;
use crate::towers::turret::Turret;
use crate::util::radians::Radian;
use bevy::prelude::*;
use std::f32::consts::PI;
use std::time::Duration;

use super::targeting::{Target, Targeting};

pub struct TowerStats {
    pub range: f32,
    pub cooldown: Duration,
    pub targeting: Targeting,
    pub rotation_speed: Radian,
}

impl TowerStats {
    pub fn fire_rate(&self) -> f32 {
        1. / self.cooldown.as_secs_f32()
    }
}

#[derive(Component)]
pub struct Tower {
    pub stats: TowerStats,
    pub rotation: Radian, // in radians
}

pub fn tower_trigger(
    mut towers: Query<(Entity, &mut Tower, &mut Transform, &mut Cooldown)>,
    mut enemies: Query<(Entity, &Enemy, &Transform, &Health), Without<Tower>>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    time: Res<Time>,
) {
    for (_, mut tower, tower_transform, mut tower_cooldown) in towers.iter_mut() {
        let mut possible_targets = vec![];
        for (enemys_entity, enemy, enemy_transform, enemy_health) in enemies.iter_mut() {
            let distance_to_enemy = tower_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate());
            if distance_to_enemy <= tower.stats.range * DISTANCE_SCALING {
                possible_targets.push(Target {
                    entity: enemys_entity,
                    enemy: enemy.clone(),
                    transform: *enemy_transform,
                    health: enemy_health.clone(),
                });
            }
        }

        if let Some(target) = tower.stats.targeting.find_best_target(&possible_targets) {
            let direction =
                target.transform.translation.truncate() - tower_transform.translation.truncate();

            // obtain angle to target with respect to x-axis.
            let angle_to_target =
                Radian::from(direction.y.atan2(direction.x) - PI / 2.).normalize_to_half();

            let angle_diff = (tower.rotation - angle_to_target).normalize_to_half();
            let allowed_rotation = tower.stats.rotation_speed * time.delta().as_secs_f32();

            if angle_diff.abs().angle > allowed_rotation.angle {
                let multiplier = match angle_diff.angle > 0. {
                    true => -1.,
                    false => 1.,
                };

                let rotation = allowed_rotation * multiplier;
                tower.rotation = (tower.rotation + rotation).normalize()
            } else {
                tower.rotation = angle_to_target;

                if !tower_cooldown.is_ready() {
                    continue;
                }

                let direction_vec = Vec2 {
                    x: -angle_to_target.angle.sin(),
                    y: angle_to_target.angle.cos(),
                };
                let bullet_translation = tower_transform.translation
                    + Vec3 {
                        x: direction_vec.x,
                        y: direction_vec.y,
                        z: 0.,
                    } * 30.
                    + Vec3::Z * 5.;

                // Shoot!
                commands
                    .spawn(SpriteBundle {
                        texture: textures.bullet_small.clone(),
                        transform: Transform {
                            translation: bullet_translation,
                            scale: SCALING_VEC3,
                            rotation: Quat::IDENTITY,
                        },
                        ..Default::default()
                    })
                    .insert(Projectile {
                        dmg: 1,
                        damage_target: DamageTarget::Team(Team::Enemy),
                        piercing_mode: PiercingMode::None,
                        entities_hit: 0,
                        is_alive: true,
                    })
                    .insert(Velocity {
                        vec: direction_vec * 600.,
                    })
                    .insert(Collider::new_circle(5., bullet_translation.truncate()));
                tower_cooldown.time_remaining += tower.stats.cooldown;
            }
        }
    }
}

pub fn spawn_tower(mut commands: Commands, textures: Res<TextureAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: textures.tower.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                rotation: Quat::IDENTITY,
                scale: SCALING_VEC3,
            },
            ..Default::default()
        })
        // Collider
        .insert(Collider::new_rect(
            Vec2 { x: 1000., y: 1000. },
            Vec2 { x: 0., y: 0. },
        ))
        // Tower
        .insert(Tower {
            stats: TowerStats {
                range: 200.,
                cooldown: Duration::from_secs_f32(1.),
                targeting: Targeting::First,
                rotation_speed: Radian { angle: PI * 2. },
            },
            rotation: Radian::ZERO,
        })
        .insert(Cooldown {
            time_remaining: Duration::ZERO,
        })
        .with_children(|parent| {
            let parent_entity = parent.parent_entity();
            parent
                .spawn(SpriteBundle {
                    texture: textures.turret.clone(),
                    transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
                    ..Default::default()
                })
                .insert(Turret {
                    parent: parent_entity,
                });
        });
}
