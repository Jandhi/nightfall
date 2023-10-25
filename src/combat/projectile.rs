use bevy::prelude::*;

use crate::collision::collider::IsCollidingEvent;
use crate::combat::health::HealthType;
use crate::constants::DISTANCE_SCALING;

use super::health::{Dead, Health};
use super::teams::{Team, TeamMember};

pub enum DamageTarget {
    All,
    Team(Team),
}

pub enum PiercingMode {
    None,
    Count(usize),
    All,
}

#[derive(Component)]
pub struct Projectile {
    pub damage_target: DamageTarget,
    pub dmg: HealthType,
    pub piercing_mode: PiercingMode,
    pub entities_hit: usize,
    pub is_alive: bool,
}

pub fn projectile_collision_check(
    mut q_projectiles: Query<&mut Projectile, Without<Dead>>,
    mut q_hittable: Query<(&mut Health, &TeamMember)>,
    mut ev_collision: EventReader<IsCollidingEvent>,
    mut commands: Commands,
) {
    for ev_is_colliding in ev_collision.iter() {
        if let (Ok(bullet), Ok((health, member))) = (
            q_projectiles.get_mut(ev_is_colliding.collision.entity_a),
            q_hittable.get_mut(ev_is_colliding.collision.entity_b),
        ) {
            handle_projectile_collision(
                ev_is_colliding.collision.entity_a,
                bullet,
                ev_is_colliding.collision.entity_b,
                health,
                member.team,
                &mut commands,
            );
        }
        if let (Ok(bullet), Ok((health, member))) = (
            q_projectiles.get_mut(ev_is_colliding.collision.entity_b),
            q_hittable.get_mut(ev_is_colliding.collision.entity_a),
        ) {
            handle_projectile_collision(
                ev_is_colliding.collision.entity_b,
                bullet,
                ev_is_colliding.collision.entity_a,
                health,
                member.team,
                &mut commands,
            );
        }
    }
}

fn handle_projectile_collision(
    projectile_entity: Entity,
    mut projectile: Mut<Projectile>,
    hit_entity: Entity,
    mut health: Mut<Health>,
    hit_team: Team,
    commands: &mut Commands,
) {
    if !projectile.is_alive {
        return; // This projectile should be dead
    }

    match projectile.damage_target {
        DamageTarget::All => {
            // Will hit
        }
        DamageTarget::Team(team_to_hit) => {
            let is_obstacle = hit_team == Team::None;
            let can_hit_team = team_to_hit == hit_team;

            if !is_obstacle && !can_hit_team {
                // Can't hit this team
                return;
            }
        }
    }

    commands.entity(projectile_entity).despawn();
    health.take_damage(projectile.dmg);
    projectile.is_alive = false;
}

#[derive(Component)]
pub struct StraightMovement {
    pub angle: Vec2,
    pub velocity: f32,
}

pub fn straight_movement(
    mut q_projectiles: Query<(Entity, &StraightMovement, &mut Transform)>,
    time: Res<Time>,
) {
    for (_, movement, mut transform) in q_projectiles.iter_mut() {
        let diff = movement.angle * time.delta_seconds() * movement.velocity * DISTANCE_SCALING;
        transform.translation += Vec3 {
            x: diff.x,
            y: diff.y,
            z: 0.,
        };
    }
}
