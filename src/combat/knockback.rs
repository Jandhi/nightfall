use bevy::prelude::*;

use crate::movement::velocity::Velocity;

use super::projectile::ProjectileHitEvent;

#[derive(Component)]
pub struct Knockback {
    pub force: f32,
}

pub fn knockback_update(
    q_knock: Query<(&Knockback, &Velocity)>,
    mut q_hit: Query<&mut Velocity, Without<Knockback>>,
    mut ev_hits: EventReader<ProjectileHitEvent>,
) {
    for hit in ev_hits.iter() {
        if let Ok((knock, projectile_velocity)) = q_knock.get(hit.projectile) {
            if let Ok(mut hit_velocity) = q_hit.get_mut(hit.victim) {
                hit_velocity.vec += projectile_velocity.vec.normalize() * knock.force;
            }
        }
    }
}
