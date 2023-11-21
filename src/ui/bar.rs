use bevy::prelude::*;

use crate::collision::collider::Collider;

use super::{
    button::{Button, ButtonBundle},
    clickable::{Clickable, ClickedEvent},
    hoverable::Hoverable,
};

pub struct BarPlugin;

impl Plugin for BarPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BarUpdatedEvent>()
            .add_systems(Update, update_bars);
    }
}

#[derive(Component)]
pub struct Bar {
    val: u32,
    max_val: u32,
}

#[derive(Event)]
pub struct BarUpdatedEvent {
    pub entity: Entity,
    pub old_val: u32,
    pub new_val: u32,
}

#[derive(Bundle)]
pub struct BarBundle {
    bar: Bar,
    sprite: SpriteSheetBundle,
}

pub struct BarButtonInfo {
    pub sprite: SpriteSheetBundle,
    pub collider: Collider,
    pub has_pressed_state: bool,
}

impl Bar {
    pub fn new(initial_val: u32, max_val: u32) -> Bar {
        Bar {
            val: initial_val,
            max_val,
        }
    }

    pub fn spawn(
        initial_val: u32,
        max_val: u32,
        bar_sprite: SpriteSheetBundle,
        prev_button: Option<BarButtonInfo>,
        next_button: Option<BarButtonInfo>,
        additional_content: Option<impl Bundle>,
        commands: &mut Commands,
    ) {
        let mut entity = commands.spawn(BarBundle {
            bar: Bar::new(initial_val, max_val),
            sprite: bar_sprite,
        });

        entity.with_children(|parent| {
            if let Some(info) = prev_button {
                parent
                    .spawn(ButtonBundle {
                        button: Button::new(info.has_pressed_state),
                        sprite: info.sprite,
                        clickable: Clickable::new(),
                        hoverable: Hoverable::new(),
                        collider: info.collider,
                    })
                    .insert(BarPreviousButton);
            }

            if let Some(info) = next_button {
                parent
                    .spawn(ButtonBundle {
                        button: Button::new(info.has_pressed_state),
                        sprite: info.sprite,
                        clickable: Clickable::new(),
                        hoverable: Hoverable::new(),
                        collider: info.collider,
                    })
                    .insert(BarNextButton);
            }
        });

        if let Some(bundle) = additional_content {
            entity.insert(bundle);
        }
    }
}

#[derive(Component)]
struct BarPreviousButton;

#[derive(Component)]
struct BarNextButton;

fn update_bars(
    mut q_bars: Query<(&mut Bar, &mut TextureAtlasSprite)>,
    q_prev_buttons: Query<&Parent, (With<BarPreviousButton>, Without<Bar>)>,
    q_next_buttons: Query<&Parent, (With<BarNextButton>, Without<Bar>)>,
    mut click_ev: EventReader<ClickedEvent>,
    mut bar_update: EventWriter<BarUpdatedEvent>,
) {
    for ev in click_ev.iter() {
        if let Ok(parent) = q_prev_buttons.get(ev.entity) {
            if let Ok((mut bar, _)) = q_bars.get_mut(parent.get()) {
                if bar.val > 0 {
                    bar.val -= 1;
                    bar_update.send(BarUpdatedEvent {
                        entity: parent.get(),
                        old_val: bar.val + 1,
                        new_val: bar.val,
                    });
                }
            }
        }

        if let Ok(bar_entity) = q_next_buttons.get(ev.entity) {
            if let Ok((mut bar, _)) = q_bars.get_mut(bar_entity.get()) {
                if bar.val < bar.max_val {
                    bar.val += 1;
                    bar_update.send(BarUpdatedEvent {
                        entity: bar_entity.get(),
                        old_val: bar.val - 1,
                        new_val: bar.val,
                    });
                }
            }
        }
    }

    for (bar, mut atlas) in q_bars.iter_mut() {
        atlas.index = bar.val as usize;
    }
}
