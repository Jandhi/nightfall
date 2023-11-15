use bevy::prelude::*;

use crate::{animation::AppAnimationSetup, GameState};

use self::{
    ability_selection::{
        ability_frame_update, create_ability_selection_rng, on_select_ability,
        start_ability_selection, AbilityFrameAnimation,
    },
    experience::{experience_update, LevelUpEvent},
    taken_abilities::{update_description, update_taken_positions},
    xp_bar::{manage_xp_bar_sprites, spawn_xp_bar, XPBarAnimation},
    xp_crystal::{create_xp_crystal_rng, drop_crystals, xp_crystal_update},
};

pub mod ability_selection;
pub mod experience;
pub mod taken_abilities;
pub mod xp_bar;
pub mod xp_crystal;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (
                create_xp_crystal_rng,
                create_ability_selection_rng,
                spawn_xp_bar,
            ),
        )
        .add_systems(
            Update,
            (
                drop_crystals,
                xp_crystal_update,
                manage_xp_bar_sprites,
                experience_update,
                start_ability_selection,
                ability_frame_update,
                update_taken_positions,
                on_select_ability,
                update_description,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_animation::<XPBarAnimation>()
        .add_animation::<AbilityFrameAnimation>()
        .add_event::<LevelUpEvent>();
    }
}
