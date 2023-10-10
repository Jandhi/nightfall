use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

/*
This is the sprite on top of a tower which turns to shoot enemies
 */

#[derive(Component)]
pub struct Turret;

pub fn follow_mouse(
    mut q_turret: Query<(Entity, &mut Turret, &mut Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>
) {

    let window = q_windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        let target = Vec2::new(cursor_position.x - window.width() / 2., cursor_position.y - window.height() / 2.);

        for (_, _, mut transform) in q_turret.iter_mut() {
            let direction = target - transform.translation.truncate();

             // obtain angle to target with respect to x-axis. 
            let angle_to_target = direction.y.atan2(direction.x);
                
            transform.rotation = Quat::from_rotation_z(-PI / 2. - angle_to_target);
        }
    }
}