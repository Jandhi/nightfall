use bevy::prelude::*;

use crate::{constants::{SCALING_VEC3, SortingLayers}, loading::TextureAssets};

use super::{health::Health, z_sort::ZSort};

pub const HEALTH_BAR_SEGMENTS: usize = 15;

#[derive(Component)]
pub struct HealthBar;

pub fn update_healthbars(
    mut q_healthbars: Query<(
        Entity, 
        &HealthBar,
        &Parent,
        &mut TextureAtlasSprite,
    )>,
    q_entities: Query<(&Health), Without<HealthBar>>,
    mut commands: Commands,
) {
    for (healthbar_entity, healthbar, parent, mut atlas) in q_healthbars.iter_mut() {
        if let Ok(health) = q_entities.get(parent.get()) {
            let index = match health.value {
                _ if health.value == health.max => 0,
                _ if health.value == 0 => HEALTH_BAR_SEGMENTS - 1,
                _ => {
                    HEALTH_BAR_SEGMENTS
                        - 2
                        - (health.value as usize * (HEALTH_BAR_SEGMENTS - 2)) / health.max as usize
                }
            };
            atlas.index = index;
        }
    }
}
