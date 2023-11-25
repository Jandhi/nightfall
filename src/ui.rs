use bevy::prelude::*;

use crate::GameState;

use self::{
    bar::BarPlugin,
    button::ButtonPlugin,
    clickable::ClickablePlugin,
    game_timer::{spawn_game_timer, update_game_timer},
    hoverable::HoverPlugin,
    selection_group::{update_selection_groups, SelectionEvent}, element::UIElementPlugin, alignment::AlignmentPlugin, grid::GridPlugin, offset::OffsetPlugin,
};

pub mod bar;
pub mod button;
pub mod clickable;
pub mod element;
pub mod game_timer;
pub mod grid;
pub mod hoverable;
pub mod selection_group;
pub mod alignment;
pub mod offset;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_selection_groups,
                update_game_timer,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Playing), spawn_game_timer)
        .add_event::<SelectionEvent>()
        .add_plugins((HoverPlugin, ClickablePlugin, ButtonPlugin, BarPlugin, UIElementPlugin, AlignmentPlugin, GridPlugin, OffsetPlugin));
    }
}

