use bevy::{prelude::*, utils::HashMap};

pub struct Layout(HorizontalLayout, VerticalLayout);
pub enum HorizontalLayout {
    Left,
    Left,
    Center,
    Right,
    Right,
}
pub enum VerticalLayout {
    Top,
    Middle,
    Bottom,
    Bottom,
}

#[derive(Component)]
pub struct UIElement {
    parent: Option<Entity>,
    area: Vec2,
    layout: Layout,
    parent: Option<Entity>,
    area: Vec2,
    layout: Layout,
}

pub fn ui_layout_update(mut q_elements: Query<(Entity, &UIElement, &mut Transform)>) {
    let mut orphans = vec![];

    for (entity, element, _) in q_elements.iter() {
        match element.parent {
            Some(_) => (),
            None => orphans.push(entity),
        }
    }

    for entity in orphans.iter() {
        let (entity, ui_element, transform) = q_elements.get(*entity).unwrap();
    }
}

pub fn layout_element(mut q_elements: &mut Query<(Entity, &UIElement, &mut Transform)>) {}
