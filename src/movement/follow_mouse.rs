use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Component)]
pub struct FollowMouse;

pub fn follow_mouse_update (
    mut q_followers : Query<&mut Transform, With<FollowMouse>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();
    let cursor_position = match window.cursor_position() {
        Some(position) => position,
        None => {
            return;
        },
    };
    let cursor_point = Vec2::new(
        cursor_position.x - window.width() / 2.,
        window.height() / 2. - cursor_position.y,
    );

    for mut follower in q_followers.iter_mut() {
        follower.translation = Vec3 {
            x : cursor_point.x,
            y : cursor_point.y,
            z : follower.translation.z
        };
    }
}