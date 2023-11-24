use bevy::{prelude::*, window::PrimaryWindow};

use crate::collision::collider::Collider;

pub struct HoverPlugin;

impl Plugin for HoverPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HoveredEvent>()
            .add_event::<UnhoveredEvent>()
            .add_systems(Update, update_hoverables);
    }
}

#[derive(Component)]
pub struct Hoverable {
    is_hovered: bool,
}

impl Hoverable {
    pub fn new() -> Hoverable {
        Hoverable { is_hovered: false }
    }

    pub fn is_hovered(&self) -> bool {
        self.is_hovered
    }
}

impl Default for Hoverable {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Event)]
pub struct UnhoveredEvent {
    pub entity: Entity,
}

#[derive(Event)]
pub struct HoveredEvent {
    pub entity: Entity,
}

#[derive(Bundle)]
pub struct HoverBundle {
    pub collider: Collider,
    pub hoverable: Hoverable,
}

fn update_hoverables(
    mut q_hoverables: Query<(Entity, &mut Hoverable, &Collider, &GlobalTransform), Without<Window>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut hover: EventWriter<HoveredEvent>,
    mut unhover: EventWriter<UnhoveredEvent>,
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

    for (entity, mut hoverable, collider, transform) in q_hoverables.iter_mut() {
        if collider.contains_point(transform.translation().truncate(), cursor_point) {
            if !hoverable.is_hovered {
                hoverable.is_hovered = true;
                hover.send(HoveredEvent { entity });
            }
        } else if hoverable.is_hovered {
            hoverable.is_hovered = false;
            unhover.send(UnhoveredEvent { entity });
        }
    }
}
