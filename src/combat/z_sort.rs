use bevy::{prelude::*, window::PrimaryWindow};


#[derive(Component)]
pub struct ZSort{
    pub layer : f32   
}

pub fn update_z_sort(
    mut q_zsorts : Query<(&mut Transform, &ZSort)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();

    for (mut transform, sort) in q_zsorts.iter_mut() {
        let percent_down = 0.5 - transform.translation.y / window.height();
        transform.translation = Vec3 {
            x : transform.translation.x,
            y : transform.translation.y,
            z : sort.layer + percent_down * 0.01, 
        };
    }
}