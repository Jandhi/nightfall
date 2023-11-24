use bevy::prelude::*;

use crate::{animation::AppAnimationSetup, GameState};

use self::{
    fire::{fire_update, FireAnimation, extinguish_errant_fire},
    health::{check_death, DeathEvent, TookDamageEvent},
    healthbar::update_healthbars,
    knockback::knockback_update,
    projectile::{projectile_collision_check, ProjectileHitEvent},
    z_sort::update_z_sort,
};

pub mod fire;
pub mod health;
pub mod healthbar;
pub mod knockback;
pub mod projectile;
pub mod teams;
pub mod z_sort;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                projectile_collision_check,
                update_healthbars,
                check_death,
                fire_update,
                extinguish_errant_fire,
                update_z_sort,
                knockback_update.after(projectile_collision_check),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_animation::<FireAnimation>()
        .add_event::<DeathEvent>()
        .add_event::<TookDamageEvent>()
        .add_event::<ProjectileHitEvent>();
    }
}
