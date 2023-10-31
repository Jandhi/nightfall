

use bevy::prelude::*;



#[derive(Component)]
pub struct SelectionGroup {
    pub is_focused : bool,
    pub hovered_index : usize,
    pub is_horizontal : bool
}

#[derive(Event)]
pub struct UnhoverEvent {
    pub parent : Entity,
    pub unhovered : Entity,
}

#[derive(Event)]
pub struct HoverEvent {
    pub parent : Entity,
    pub hovered : Entity,
}

#[derive(Event)]
pub struct SelectionEvent {
    pub parent : Entity,
    pub selected : Entity,
    pub selected_index : usize,
}

pub fn update_selection_groups(
    mut selection_groups : Query<(Entity, &mut SelectionGroup, &Children)>,
    mut hover : EventWriter<HoverEvent>,
    mut unhover : EventWriter<UnhoverEvent>,
    mut select : EventWriter<SelectionEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (entity, mut selection_group, children) in selection_groups.iter_mut() {
        if !selection_group.is_focused {
            continue;
        }

        if selection_group.is_horizontal {
            let left_pressed = keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A);
            let right_pressed = keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D);
        
            let child_count = children.len();

            if left_pressed && selection_group.hovered_index > 0 {
                unhover.send(UnhoverEvent { parent: entity, unhovered: *children.get(selection_group.hovered_index).unwrap() });
                selection_group.hovered_index -= 1;
                hover.send(HoverEvent { parent: entity, hovered: *children.get(selection_group.hovered_index).unwrap() })
            } else if right_pressed && selection_group.hovered_index < child_count - 1 {
                unhover.send(UnhoverEvent { parent: entity, unhovered: *children.get(selection_group.hovered_index).unwrap() });
                selection_group.hovered_index += 1;
                hover.send(HoverEvent { parent: entity, hovered: *children.get(selection_group.hovered_index).unwrap() })
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