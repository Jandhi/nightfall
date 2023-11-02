use bevy::prelude::*;

use crate::{
    combat::health::{Health, TookDamageEvent},
    constants::{SortingLayers, SCALING_VEC3},
    loading::TextureAssets,
};

use super::Player;

#[derive(Component)]
pub struct HitSprite(Timer);

pub fn update_hit_sprite(
    mut q_hit: Query<(Entity, &mut HitSprite, &mut Visibility, &mut Transform), Without<Player>>,
    q_player: Query<(Entity, &Health, &Transform), With<Player>>,
    mut ev_dmgs: EventReader<TookDamageEvent>,
    time: Res<Time>,
) {
    let (player, p_health, p_transform) = q_player.single();
    let (entity, mut hit, mut visibility, mut transform) = q_hit.single_mut();

    if !p_health.is_invincible {
        *visibility = Visibility::Hidden;
        return;
    }

    // Follow player
    transform.translation.x = p_transform.translation.x;
    transform.translation.y = p_transform.translation.y;

    for ev_dmg in ev_dmgs.iter() {
        if ev_dmg.entity == player {
            hit.0.reset();
            *visibility = Visibility::Visible;
        }
    }

    hit.0.tick(time.delta());

    if hit.0.just_finished() {
        *visibility = match *visibility {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }
}

pub fn spawn_hit_sprite(textures: Res<TextureAssets>, mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            texture: textures.hit.clone(),
            visibility: Visibility::Hidden,
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: SortingLayers::UI.into(),
                },
                rotation: default(),
                scale: SCALING_VEC3,
            },
            ..Default::default()
        })
        .insert(HitSprite(Timer::from_seconds(0.25, TimerMode::Repeating)));
}
