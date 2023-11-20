use bevy::{prelude::*, window::PrimaryWindow};

use crate::collision::collider::Collider;

use super::hoverable::{HoveredEvent, UnhoveredEvent};

#[derive(Component)]
pub struct SelectionGroup {
    pub is_focused: bool,
    pub hovered_index: usize,
    pub is_horizontal: bool,
}

#[derive(Component)]
pub struct SelectionElement {
    pub index: usize,
}

#[derive(Event)]
pub struct SelectionEvent {
    pub parent: Entity,
    pub selected: Entity,
    pub selected_index: usize,
}

pub fn update_selection_groups(
    mut selection_groups: Query<(Entity, &mut SelectionGroup, &Children)>,
    selection_elements: Query<
        (Entity, &Transform, &Parent, &Collider, &SelectionElement),
        Without<SelectionGroup>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut hover: EventWriter<HoveredEvent>,
    mut unhover: EventWriter<UnhoveredEvent>,
    mut select: EventWriter<SelectionEvent>,
    mouse_button: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let window = q_windows.single();

    for (entity, transform, parent, collider, element) in selection_elements.iter() {
        if let Ok((parent_entity, mut group, children)) = selection_groups.get_mut(parent.get()) {
            if let Some(cursor_position) = window.cursor_position() {
                let cursor_point = Vec2::new(
                    cursor_position.x - window.width() / 2.,
                    window.height() / 2. - cursor_position.y,
                );

                if !cursor_point.x.is_nan()
                    && !cursor_point.y.is_nan()
                    && collider.contains_point(transform.translation.truncate(), cursor_point)
                {
                    if group.hovered_index != element.index {
                        unhover.send(UnhoveredEvent {
                            entity: *children.get(group.hovered_index).unwrap(),
                        });
                        group.hovered_index = element.index;
                        hover.send(HoveredEvent { entity });
                    }

                    if mouse_button.just_pressed(MouseButton::Left) {
                        select.send(SelectionEvent {
                            parent: parent_entity,
                            selected: *children.get(group.hovered_index).unwrap(),
                            selected_index: group.hovered_index,
                        })
                    }
                }
            }
        }
    }

    for (entity, mut selection_group, children) in selection_groups.iter_mut() {
        if !selection_group.is_focused {
            continue;
        }

        if selection_group.is_horizontal {
            let left_pressed = keyboard_input.just_pressed(KeyCode::Left)
                || keyboard_input.just_pressed(KeyCode::A);
            let right_pressed = keyboard_input.just_pressed(KeyCode::Right)
                || keyboard_input.just_pressed(KeyCode::D);

            let child_count = children.len();

            if left_pressed && selection_group.hovered_index > 0 {
                unhover.send(UnhoveredEvent {
                    entity: *children.get(selection_group.hovered_index).unwrap(),
                });
                selection_group.hovered_index -= 1;
                hover.send(HoveredEvent {
                    entity: *children.get(selection_group.hovered_index).unwrap(),
                });
            } else if right_pressed && selection_group.hovered_index < child_count - 1 {
                unhover.send(UnhoveredEvent {
                    entity: *children.get(selection_group.hovered_index).unwrap(),
                });
                selection_group.hovered_index += 1;
                hover.send(HoveredEvent {
                    entity: *children.get(selection_group.hovered_index).unwrap(),
                });
            }
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            select.send(SelectionEvent {
                parent: entity,
                selected: *children.get(selection_group.hovered_index).unwrap(),
                selected_index: selection_group.hovered_index,
            })
        }
    }
}
