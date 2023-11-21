use bevy::prelude::*;

use super::element::{UIElement, ParentResizedEvent, UILayout, Sized};

pub struct AlignmentPlugin;

impl Plugin for AlignmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(UILayout, alignment_layout);
    }
}

pub enum Alignment {
    TopLeft,
    TopCenter,
    TopRight,

    MiddleLeft,
    MiddleCenter,
    MiddleRight,

    BottomLeft,
    BottomCenter,
    BottomRight,
}

impl Alignment {
    pub fn vertical(&self) -> VerticalAlignment {
        match self {
            Self::TopLeft    | Self::TopCenter    | Self::TopRight    => VerticalAlignment::Top,
            Self::MiddleLeft | Self::MiddleCenter | Self::MiddleRight => VerticalAlignment::Middle,
            Self::BottomLeft | Self::BottomCenter | Self::BottomRight => VerticalAlignment::Bottom,
        }
    }

    pub fn horizontal(&self) -> HorizontalAlignment {
        match self {
            Self::TopLeft   | Self::MiddleLeft   | Self::BottomLeft   => HorizontalAlignment::Left,
            Self::TopCenter | Self::MiddleCenter | Self::BottomCenter => HorizontalAlignment::Center,
            Self::TopRight  | Self::MiddleRight  | Self::BottomRight  => HorizontalAlignment::Right,
        }
    }
}

pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom
}

pub enum HorizontalAlignment {
    Left,
    Center,
    Right
}

#[derive(Component)]
pub struct Aligned {
    alignment: Alignment
}

impl Default for Aligned {
    fn default() -> Self {
        Self { alignment: Alignment::MiddleCenter }
    }
}

#[derive(Bundle)]
pub struct AlignedBundle {
    ui_element : UIElement,
    aligned : Aligned,
    sized : Sized,
}

impl Default for AlignedBundle {
    fn default() -> Self {
        Self { ui_element: Default::default(), aligned: Default::default(), sized: Default::default() }
    }
}

impl AlignedBundle {
    pub fn new(alignment : Alignment, size : Vec2) -> AlignedBundle {
        AlignedBundle {
            ui_element: UIElement::default(),
            aligned: Aligned { alignment },
            sized: Sized { size },
        }
    }
}

fn alignment_layout(
    mut q_aligned : Query<(&mut Transform, &Aligned, &Sized)>,
    mut layout_ev : EventReader<ParentResizedEvent>,
) {
    for layout in layout_ev.iter() {
        if let Ok((mut transform, aligned, sized)) = q_aligned.get_mut(layout.entity) {
            

            let x = match aligned.alignment.horizontal() {
                HorizontalAlignment::Left => sized.size.x / 2.0,
                HorizontalAlignment::Center => layout.size.x / 2.0,
                HorizontalAlignment::Right => layout.size.x - sized.size.x / 2.0,
            };
            
            let y = match aligned.alignment.vertical() {
                VerticalAlignment::Top => layout.size.y - sized.size.y / 2.0,
                VerticalAlignment::Middle => layout.size.y / 2.0,
                VerticalAlignment::Bottom => sized.size.y / 2.0,
            };

            transform.translation = Vec3 {
                x: x + layout.offset.x,
                y: y + layout.offset.y,
                z: transform.translation.z,
            };

            info!("Alignment layout to {:?} so my pos is now {}", layout, transform.translation);
        }
    }
}