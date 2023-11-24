use bevy::prelude::*;

use super::element::{UILayout, UIElement, ParentResizedEvent, ToBeResized, Sized, SizeBundle};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(UILayout, grid_update);
    }
}

#[derive(Component, Clone)]
pub struct Grid {
    pub grid_size: IVec2,
}

impl Default for Grid {
    fn default() -> Self {
        Self { grid_size: IVec2 { x: 0, y: 0 } }
    }
}

#[derive(Bundle)]
pub struct GridBundle {
    pub ui_element : UIElement,
    pub grid : Grid,
    pub size : SizeBundle,
    pub transform : Transform,
    pub global_transform : GlobalTransform,
    pub visibility : Visibility,
    pub computed_visibility : ComputedVisibility,
}

impl Default for GridBundle {
    fn default() -> Self {
        Self { ui_element: default(), grid: default(), size: default(), visibility: Visibility::Inherited, computed_visibility: default(), transform: default(), global_transform: default() }
    }
}

fn grid_update(
    q_grids : Query<(&Grid, &Sized, &Children)>,
    mut resize_ev : EventReader<ParentResizedEvent>,
    mut to_be_resized : ResMut<ToBeResized>
) {
    for resized in resize_ev.iter() {
        if let Ok((grid, sized, children)) = q_grids.get(resized.entity) {
            let cell_size = Vec2 {
                x : sized.size.x / grid.grid_size.x as f32,
                y : sized.size.y / grid.grid_size.y as f32,
            };

            for (index, child) in children.iter().enumerate() {
                let x_pos = index as i32 % grid.grid_size.x;
                let y_pos = (grid.grid_size.y - 1) - (index as i32 / grid.grid_size.x);
                
                to_be_resized.elements.push(ParentResizedEvent { 
                    entity: *child, 
                    size: cell_size, 
                    offset: Vec2 { x: x_pos as f32 * cell_size.x, y: y_pos as f32 * cell_size.y } - sized.size / 2.0
                });
            }
        }
    }
}