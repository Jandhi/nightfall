use bevy::{prelude::*, window::PrimaryWindow};

use crate::collision::collider::Collider;

pub struct ClickablePlugin;

impl Plugin for ClickablePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClickedEvent>()
            .add_event::<UnclickedEvent>()
            .add_systems(Update, update_clickables);
    }
}

#[derive(Component)]
pub struct Clickable {
    is_clicked: bool,
}

impl Clickable {
    pub fn new() -> Clickable {
        Clickable { is_clicked: false }
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }
}

impl Default for Clickable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Event)]
pub struct ClickedEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct UnclickedEvent {
    pub entity: Entity,
}

fn update_clickables(
    mut q_clickables: Query<(Entity, &mut Clickable, &Collider, &GlobalTransform), Without<Window>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut clicked: EventWriter<ClickedEvent>,
    mut unclicked: EventWriter<UnclickedEvent>,
    mouse_button: Res<Input<MouseButton>>,
) {
    let window = q_windows.single();
    let cursor_position = match window.cursor_position() {
        Some(position) => position,
        None => {
            return;
        }
    };
    let cursor_point = Vec2::new(
        cursor_position.x - window.width() / 2.,
        window.height() / 2. - cursor_position.y,
    );

    for (entity, mut clickable, collider, transform) in q_clickables.iter_mut() {
        if collider.contains_point(transform.translation().truncate(), cursor_point) {
            if mouse_button.just_pressed(MouseButton::Left) {
                if !clickable.is_clicked {
                    clickable.is_clicked = true;
                    clicked.send(ClickedEvent { entity });
                }
            } else if mouse_button.just_released(MouseButton::Left) && clickable.is_clicked {
                clickable.is_clicked = false;
                unclicked.send(UnclickedEvent { entity });
            }
        } else if clickable.is_clicked {
            clickable.is_clicked = false;
            unclicked.send(UnclickedEvent { entity });
        }
    }
}
