use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    constants::{SortingLayers, SCALING_VEC3},
    loading::TextureAssets,
};

use super::Player;

#[derive(Component)]
pub struct ReloadUI;

#[derive(Resource)]
pub struct ReloadTimer(pub Timer);

pub fn spawn_reload_ui(
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_reload_ui.clone(),
        Vec2 { x: 16., y: 16. },
        10,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(9),
            transform: Transform {
                translation: default(),
                rotation: Quat::IDENTITY,
                scale: SCALING_VEC3,
            },
            ..Default::default()
        })
        .insert(ReloadUI);
}

pub fn update_reload_ui(
    mut q_reload_ui: Query<(&mut Transform, &mut TextureAtlasSprite), With<ReloadUI>>,
    q_windows: Query<&Window, Without<ReloadUI>>,
    q_player: Query<(&Player), (Without<ReloadUI>, Without<Window>)>,
    mut timer: Res<ReloadTimer>,
) {
    let (mut reload_transform, mut reload_atlas) = q_reload_ui.single_mut();
    let window = q_windows.single();
    let player = q_player.single();

    reload_transform.translation = Vec3 {
        x: window.width() / 2. - 40.,
        y: window.height() / 2. - 30. - 20. * player.max_bullets as f32,
        z: SortingLayers::UI.into(),
    };

    if timer.0.remaining() > Duration::ZERO {
        let index = 9.0 * (1.0 - timer.0.remaining().as_secs_f32() / player.reload_time);
        reload_atlas.index = index as usize;
    } else {
        reload_atlas.index = 9;
    }
}
