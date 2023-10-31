use bevy::prelude::*;

use crate::GameState;

use self::{grid::update_grid_elements, selection_group::{update_selection_groups, HoverEvent, UnhoverEvent, SelectionEvent}};

pub mod grid;
pub mod element;
pub mod selection_group;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            update_grid_elements,
            update_selection_groups
        ).run_if(in_state(GameState::Playing)))
        .add_event::<HoverEvent>()
        .add_event::<UnhoverEvent>()
        .add_event::<SelectionEvent>();
    }
}