use bevy::prelude::*;

use crate::util::with_z::WithZ;

use super::element::{SizeVec2, ParentResizedEvent, PostUILayout};

pub struct OffsetPlugin;

impl Plugin for OffsetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUILayout, update_offset);
    }
}

#[derive(Component)]
pub struct Offset {
    pub amount : SizeVec2,
}

fn update_offset(
    mut q_offsets : Query<(&Offset, &mut Transform)>,
    mut resize_evs : EventReader<ParentResizedEvent>,
) {
    for resize in resize_evs.iter() {
        if let Ok((offset, mut transform)) = q_offsets.get_mut(resize.entity) {
            transform.translation = (
                transform.translation.truncate() + offset.amount.calculate(resize.size)
            ).with_z(transform.translation.z);
        }
    }
}