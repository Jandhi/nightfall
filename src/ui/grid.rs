use bevy::{prelude::*, utils::HashMap};

#[derive(Component)]
pub struct GridElement {
    pub index: IVec2,
}

#[derive(Component, Clone)]
pub struct Grid {
    pub size: Vec2,
    pub grid_size: IVec2,
}

pub fn update_grid_elements(
    mut q_elements: Query<(&GridElement, &mut Transform, &Parent)>,
    q_grids: Query<(&Grid, &Transform), Without<GridElement>>,
) {
    for (element, mut transform, parent) in q_elements.iter_mut() {
        if let Ok((grid, grid_transform)) = q_grids.get(parent.get()) {
            update_grid_element(element, transform, grid, grid_transform);
        }
    }
}

fn update_grid_element(
    element: &GridElement,
    mut element_transform: Mut<'_, Transform>,
    parent: &Grid,
    parent_transform: &Transform,
) {
    let parent_zero = parent_transform.translation.truncate() - parent.size / 2.;
    let x_diff = parent.size.x / (parent.grid_size.x + 1) as f32;
    let y_diff = parent.size.y / (parent.grid_size.y + 1) as f32;
    let position = parent_zero
        + Vec2 {
            x: x_diff * (element.index.x + 1) as f32,
            y: y_diff * (element.index.y + 1) as f32,
        };

    element_transform.translation = Vec3 {
        x: position.x,
        y: position.y,
        z: element_transform.translation.z,
    }
}
