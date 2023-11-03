use bevy::prelude::*;

use crate::GameState;

use self::{
    grid::update_grid_elements,
    selection_group::{update_selection_groups, HoverEvent, SelectionEvent, UnhoverEvent}, game_timer::{update_game_timer, spawn_game_timer},
};

pub mod element;
pub mod grid;
pub mod selection_group;
pub mod game_timer;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_grid_elements, update_selection_groups, update_game_timer).run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            OnEnter(GameState::Playing),
            spawn_game_timer,
        )
        .add_event::<HoverEvent>()
        .add_event::<UnhoverEvent>()
        .add_event::<SelectionEvent>();
    }
}
