use bevy::prelude::*;

use crate::{GameState, animation::AppAnimationSetup};

use self::{xp_crystal::{create_xp_crystal_rng, drop_crystals, xp_crystal_update}, xp_bar::{spawn_xp_bar, manage_xp_bar_sprites, XPBarAnimation}, experience::{LevelUpEvent, experience_update}, ability_selection::{create_ability_selection_rng, start_ability_selection, AbilityFrameAnimation, ability_frame_update}};

mod xp_crystal;
pub mod xp_bar;
pub mod experience;
pub mod ability_selection;

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
            create_xp_crystal_rng,
            create_ability_selection_rng,
            spawn_xp_bar
        )).add_systems(Update, (
            drop_crystals,
            xp_crystal_update,
            manage_xp_bar_sprites,
            experience_update,
            start_ability_selection,
            ability_frame_update,
        ).run_if(in_state(GameState::Playing)))
        .add_animation::<XPBarAnimation>()
        .add_animation::<AbilityFrameAnimation>()
        .add_event::<LevelUpEvent>();
    }
}