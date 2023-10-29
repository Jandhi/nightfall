use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};


use crate::{
    animation::{
        animation_bundle, AnimationStateChangeEvent,
        AnimationStateStorage, Animation, info::{AnimationStateInfo, AnimationInfoBuilder},
    },
    loading::TextureAssets, combat::health::Health,
};

use super::Player;

#[derive(Component)]
pub struct HealthUISprite {
    index: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum HealthUIAnimationState {
    Available,
    Unavailable,
}

impl Animation<HealthUIAnimationState> for HealthUIAnimationState {
    fn get_states() -> Vec<AnimationStateInfo<HealthUIAnimationState>> {
        AnimationInfoBuilder::new()
            .add_single(HealthUIAnimationState::Available)
            .add_single(HealthUIAnimationState::Unavailable)
            .build()
    }
}

pub type HealthUIAnimations = AnimationStateStorage<HealthUIAnimationState>;

#[derive(Resource)]
pub struct HealthUICount(pub u32);

pub fn manage_health_ui_sprites(
    q_player: Query<&Health, (With<Player>, Without<HealthUISprite>)>,
    mut q_hearts: Query<
        (Entity, &HealthUISprite, &TextureAtlasSprite, &mut Transform),
        Without<Player>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut animation_state_change: EventWriter<AnimationStateChangeEvent<HealthUIAnimationState>>,
    animations: Res<HealthUIAnimations>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut heart_count: ResMut<HealthUICount>,
    mut commands: Commands,
) {
    let health = q_player.single();
    let window = q_windows.single();

    while heart_count.0 < health.max {
        spawn_heart_ui_sprite(
            &animations,
            &textures,
            &mut texture_atlases,
            &mut commands,
            heart_count.0,
        );

        heart_count.0 += 1;
    }

    for (entity, heart, atlas, mut transform) in q_hearts.iter_mut() {
        if heart.index >= health.max {
            commands.entity(entity).despawn();
            continue;
        }

        if atlas.index == 0 && heart.index >= health.value {
            animation_state_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: HealthUIAnimationState::Unavailable,
            })
        } else if atlas.index == 1 && heart.index < health.value {
            animation_state_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: HealthUIAnimationState::Available,
            })
        }

        transform.translation = Vec3 {
            x: -window.width() / 2. + 40. + 40. * (health.max - 1 - heart.index) as f32,
            y: window.height() / 2. - 30.,
            z: 5.,
        }
    }
}

fn spawn_heart_ui_sprite(
    animations: &Res<HealthUIAnimations>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
    index: u32,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_heart_ui.clone(),
        Vec2 { x: 16., y: 16. },
        2,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(HealthUISprite { index })
        .insert(animation_bundle(
            HealthUIAnimationState::Available,
            animations,
            texture_atlas_handle,
            default(),
        ));
}
