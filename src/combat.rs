use bevy::prelude::*;

use crate::GameState;

use self::{
    health::{check_death, DeathEvent},
    healthbar::{spawn_healthbars, update_healthbars},
    projectile::{projectile_collision_check},
};

pub mod health;
pub mod healthbar;
pub mod projectile;
pub mod teams;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                    projectile_collision_check,
                    update_healthbars,
                    check_death,
                    spawn_healthbars
                ).run_if(in_state(GameState::Playing)),
        )
        .add_event::<DeathEvent>();
    }
}
