use bevy::prelude::*;

use crate::collision::collider::Collider;

use super::{
    clickable::{Clickable, ClickedEvent, UnclickedEvent},
    hoverable::{Hoverable, HoveredEvent, UnhoveredEvent},
};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_buttons);
    }
}

#[derive(PartialEq)]
pub enum ButtonState {
    Normal,
    Hovered,
    Pressed,
}

#[derive(Component)]
pub struct Button {
    state: ButtonState,
    has_pressed_state: bool,
}

impl Button {
    pub fn new(has_pressed_state: bool) -> Button {
        Button {
            state: ButtonState::Normal,
            has_pressed_state,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            state: ButtonState::Normal,
            has_pressed_state: true,
        }
    }
}

#[derive(Bundle)]
pub struct ButtonBundle {
    pub button: Button,
    pub sprite: SpriteSheetBundle,
    pub clickable: Clickable,
    pub hoverable: Hoverable,
    pub collider: Collider,
}

fn update_buttons(
    mut q_buttons: Query<(&mut Button, &mut TextureAtlasSprite)>,
    mut clicked: EventReader<ClickedEvent>,
    mut unclicked: EventReader<UnclickedEvent>,
    mut hovered: EventReader<HoveredEvent>,
    mut unhovered: EventReader<UnhoveredEvent>,
) {
    for ev in clicked.iter() {
        match q_buttons.get_mut(ev.entity) {
            Ok((mut button, _)) => {
                button.state = ButtonState::Pressed;
            }
            Err(_) => (),
        }
    }

    for ev in unclicked.iter() {
        match q_buttons.get_mut(ev.entity) {
            Ok((mut button, _)) => {
                if button.state == ButtonState::Pressed {
                    button.state = ButtonState::Hovered;
                }
            }
            Err(_) => (),
        }
    }

    for ev in hovered.iter() {
        match q_buttons.get_mut(ev.entity) {
            Ok((mut button, _)) => {
                if button.state == ButtonState::Normal {
                    button.state = ButtonState::Hovered;
                }
            }
            Err(_) => (),
        }
    }

    for ev in unhovered.iter() {
        match q_buttons.get_mut(ev.entity) {
            Ok((mut button, _)) => {
                if button.state == ButtonState::Hovered {
                    button.state = ButtonState::Normal;
                }
            }
            Err(_) => (),
        }
    }

    for (button, mut atlas) in q_buttons.iter_mut() {
        atlas.index = match button.state {
            ButtonState::Normal => 0,
            ButtonState::Hovered => 1,
            ButtonState::Pressed => match button.has_pressed_state {
                true => 2,
                false => 0,
            },
        };
    }
}
