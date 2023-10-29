use bevy::prelude::*;

use crate::{GameState, animation::AppAnimationSetup};

use self::{xp_crystal::{create_rng, drop_crystals, xp_crystal_update}, xp_bar::{spawn_xp_bar, manage_bullet_ui_sprites, XPBarAnimation}};

mod xp_crystal;
pub mod xp_bar;
pub mod experience;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
            create_rng,
            spawn_xp_bar
        )).add_systems(Update, (
            drop_crystals,
            xp_crystal_update,
            manage_bullet_ui_sprites
        ).run_if(in_state(GameState::Playing)))
        .add_animation::<XPBarAnimation>();
    }
}