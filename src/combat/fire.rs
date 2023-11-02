use std::time::Duration;

use bevy::{prelude::*, transform::commands};

use crate::{
    animation::{
        info::AnimationInfoBuilder, make_animation_bundle, Animation, AnimationStateStorage,
    },
    enemies::enemy::Enemy,
    loading::TextureAssets,
    player::Player,
};

use super::{
    health::{Health, TookDamageEvent},
    projectile::ProjectileHitEvent,
};

#[derive(Component)]
pub struct Fire {
    pub parent: Entity,
    pub timer: Timer,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum FireAnimation {
    Fire,
}

impl Animation<FireAnimation> for FireAnimation {
    fn get_states() -> Vec<crate::animation::info::AnimationStateInfo<FireAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(FireAnimation::Fire, 5, Duration::from_secs_f32(1. / 8.))
            .build()
    }
}

pub fn fire_update(
    mut q_fire: Query<(&mut Fire, &Parent), Without<Health>>,
    mut q_health: Query<(Entity, &mut Health, &Children), With<Enemy>>,
    mut q_player: Query<&Player, (Without<Enemy>, Without<Fire>)>,
    mut took_damage_ev: EventWriter<TookDamageEvent>,
    mut projectile_hit: EventReader<ProjectileHitEvent>,
    animations: Res<AnimationStateStorage<FireAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut fire, parent) in q_fire.iter_mut() {
        fire.timer.tick(time.delta());

        if !fire.timer.just_finished() {
            continue;
        }

        if let Ok((_, mut health, _)) = q_health.get_mut(parent.get()) {
            health.take_damage(parent.get(), &mut took_damage_ev, 2)
        }
    }

    let texture_atlas = TextureAtlas::from_grid(
        textures.fire.clone(),
        Vec2 { x: 32., y: 32. },
        5,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = q_player.single();

    if !player
        .abilities
        .contains(&crate::player::ability::Ability::FlamingBullets)
    {
        return;
    }

    for proj_hit in projectile_hit.iter() {
        if let Ok((entity, _, children)) = q_health.get(proj_hit.victim) {
            if children.iter().any(|child| q_fire.contains(*child)) {
                continue;
            }

            commands
                .spawn(make_animation_bundle(
                    FireAnimation::Fire,
                    &animations,
                    texture_atlas_handle.clone(),
                    Vec3::ZERO,
                    0.5,
                ))
                .insert(Fire {
                    parent: entity,
                    timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                })
                .add(|id, world: &mut World| {
                    if let Some(fire) = world.entity(id).get::<Fire>() {
                        world.entity_mut(fire.parent).add_child(id);
                    }
                });
        }
    }
}
