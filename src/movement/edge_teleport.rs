use bevy::{prelude::*, window::PrimaryWindow};

const EDGE_GRACE: f32 = 50.0;

#[derive(Component)]
pub struct EdgeTeleports;

pub fn edge_teleporting(
    mut q_transforms: Query<&mut Transform, With<EdgeTeleports>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();

    for mut transform in q_transforms.iter_mut() {
        if transform.translation.x < -window.width() / 2. - EDGE_GRACE {
            transform.translation.x = window.width() / 2. + EDGE_GRACE / 2.;
        } else if transform.translation.x > window.width() / 2. + EDGE_GRACE {
            transform.translation.x = -window.width() / 2. - EDGE_GRACE / 2.;
        } else if transform.translation.y < -window.height() / 2. - EDGE_GRACE {
            transform.translation.y = window.height() / 2. + EDGE_GRACE / 2.;
        } else if transform.translation.y > window.height() / 2. + EDGE_GRACE {
            transform.translation.y = -window.height() / 2. - EDGE_GRACE / 2.;
        }
    }
}
