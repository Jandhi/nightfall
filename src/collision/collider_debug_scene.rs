use bevy::prelude::*;

use crate::movement::follow_mouse::FollowMouse;

use super::collider::Collider;

pub fn enter_debug_scene(
    mut commands : Commands
) {
    commands
        .spawn(Transform::default())
        .insert(GlobalTransform::default())
        .insert(Collider::new_circle(20.0, default()))
        .insert(Visibility::Visible)
        .insert(ComputedVisibility::default())
        .insert(FollowMouse);

    commands
        .spawn(Transform::from_translation(Vec3 { x: 100., y: 0., z: 0. }))
        .insert(GlobalTransform::default())
        .insert(Visibility::Visible)
        .insert(ComputedVisibility::default())
        .insert(Collider::new_circle(50., Vec2 { x: 100., y: 0. }));

        commands
        .spawn(Transform::from_translation(Vec3 { x: -100., y: 0., z: 0. }))
        .insert(GlobalTransform::default())
        .insert(Visibility::Visible)
        .insert(ComputedVisibility::default())
        .insert(Collider::new_rect(Vec2 { x: 50., y: 50. }, Vec2 { x: -100., y: 0. }));
}