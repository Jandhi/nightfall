use bevy::prelude::*;

use crate::{constants::SCALING_VEC3, loading::TextureAssets};

use super::health::Health;

const HEALTH_BAR_SEGMENTS: usize = 15;

// If added to a component, the system will spawn a healthbar for it
#[derive(Component)]
pub struct NeedsHealthBar {
    pub offset: Vec2,
    is_done: bool,
}

impl NeedsHealthBar {
    fn with_offset(offset: Vec2) -> NeedsHealthBar {
        NeedsHealthBar {
            offset,
            is_done: false,
        }
    }
}

impl Default for NeedsHealthBar {
    fn default() -> Self {
        Self {
            offset: Vec2::ZERO,
            is_done: false,
        }
    }
}

#[derive(Component)]
pub struct HealthBar {
    entity: Entity,
    is_alive: bool,
    offset: Vec2,
}

pub fn spawn_healthbars(
    mut q_entities: Query<(Entity, &Transform, &Health, &mut NeedsHealthBar)>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    for (entity, _, _, mut needs_health_bar) in q_entities.iter_mut() {
        if needs_health_bar.is_done {
            continue;
        }

        commands.entity(entity).remove::<NeedsHealthBar>();

        let texture_atlas = TextureAtlas::from_grid(
            textures.healthbar.clone(),
            Vec2 { x: 32., y: 32. },
            HEALTH_BAR_SEGMENTS,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands
            .spawn(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: Vec3::new(0., 0., 1.),
                    rotation: Quat::IDENTITY,
                    scale: SCALING_VEC3,
                },
                ..Default::default()
            })
            .insert(HealthBar {
                entity,
                is_alive: true,
                offset: Vec2 { x: 0., y: 0. },
            });
        needs_health_bar.is_done = true;
    }
}

pub fn update_healthbars(
    mut q_healthbars: Query<(
        Entity,
        &mut Transform,
        &mut HealthBar,
        &mut TextureAtlasSprite,
    )>,
    q_entities: Query<(Entity, &Transform, &Health), Without<HealthBar>>,
    mut commands: Commands,
) {
    for (healthbar_entity, mut healthbar_transform, mut healthbar, mut sprite_atlas) in
        q_healthbars.iter_mut()
    {
        if let Ok((_, parent_transform, health)) = q_entities.get(healthbar.entity) {
            let translation_2d = parent_transform.translation.truncate() + healthbar.offset;
            healthbar_transform.translation = Vec3 {
                x: translation_2d.x,
                y: translation_2d.y,
                z: healthbar_transform.translation.z,
            };

            let index = match health.value {
                _ if health.value == health.max => 0,
                _ if health.value == 0 => HEALTH_BAR_SEGMENTS - 1,
                _ => {
                    HEALTH_BAR_SEGMENTS
                        - 2
                        - (health.value as usize * (HEALTH_BAR_SEGMENTS - 2)) / health.max as usize
                }
            };
            sprite_atlas.index = index;
        } else if healthbar.is_alive {
            healthbar.is_alive = false;
            commands.entity(healthbar_entity).despawn();
        }
    }
}
