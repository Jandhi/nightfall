use bevy::prelude::*;

use crate::{animation::{AnimationStateStorage, make_animation_bundle}, loading::TextureAssets, collision::collider::Collider};

use super::Player;

#[derive(Component)]
pub struct BulletUISprite {
    index : usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BulletUIAnimationState {
    Available,
    Unavailable
}
pub type BulletUIAnimations = AnimationStateStorage<BulletUIAnimationState>;

pub fn manage_bullet_ui_sprites(
    q_player : Query<(&Player), Without<BulletUISprite>>,
    mut q_bullets : Query<(&BulletUISprite, &mut TextureAtlasSprite, &mut Transform), Without<Player>>
) {
    let player = q_player.single(); 

    for (bullet, sprite, mut transform) in q_bullets.iter_mut() {
        
    }
}

pub fn spawn_bullet_ui_sprite(
    animations : Res<BulletUIAnimations>,
    textures: Res<TextureAssets>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
    mut commands: Commands, 
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_bullet_ui.clone(),
         Vec2 { x: 32., y: 32. },
          2,
           1,
            None,
             None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
    .spawn(BulletUISprite{
        index: 0
    })
    .insert(make_animation_bundle(
        BulletUIAnimationState::Available, 
        animations, 
        texture_atlas_handle,
        Vec3 { x: 0., y: 0., z: 0. }
    ));
}