use bevy::prelude::*;

use crate::collision::collider::{IsCollidingEvent, CollisionStartEvent};
use crate::combat::health::HealthType;
use crate::util::radians::Radian;


use super::health::{Dead, Health, TookDamageEvent};
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

#[derive(Event)]
pub struct ProjectileHitEvent {
    pub projectile : Entity,
    pub victim : Entity,
}

pub fn projectile_collision_check(
    mut q_projectiles: Query<&mut Projectile, Without<Dead>>,
    mut q_hittable: Query<(&mut Health, &TeamMember)>,
    mut ev_collision: EventReader<CollisionStartEvent>,
    mut ev_hit: EventWriter<ProjectileHitEvent>,
    mut ev_dmg : EventWriter<TookDamageEvent>,
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
                &mut ev_hit,
                &mut ev_dmg,
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
                &mut ev_hit,
                &mut ev_dmg,
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
    ev_hit : &mut EventWriter<ProjectileHitEvent>,
    ev_dmg : &mut EventWriter<TookDamageEvent>,
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

    ev_hit.send(ProjectileHitEvent { projectile: projectile_entity, victim: hit_entity });

    projectile.entities_hit += 1;
    health.take_damage(hit_entity, ev_dmg, projectile.dmg);

    let is_dead = match projectile.piercing_mode {
        PiercingMode::None => true,
        PiercingMode::Count(count) => projectile.entities_hit >= count,
        PiercingMode::All => false,
    };

    if is_dead {
        commands.entity(projectile_entity).despawn();
        projectile.is_alive = false;
    }
}

