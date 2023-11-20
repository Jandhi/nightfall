use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::{
        controller::AnimationController,
        info::{AnimationInfoBuilder, AnimationStateInfo},
        make_animation_bundle, Animation, AnimationStateChangeEvent, AnimationStateStorage,
    },
    collision::collider::{Collider, IsCollidingEvent},
    combat::{
        health::TookDamageEvent,
        projectile::{DamageTarget, PiercingMode, Projectile},
        teams::Team,
    },
    constants::{SortingLayers, SCALING_VEC3},
    enemies::{self, enemy::Enemy},
    loading::TextureAssets,
    movement::pause::ActionPauseState,
};

use super::Player;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ThornsAnimation {
    Spawning,
    Present,
    Despawning,
}

impl Animation<ThornsAnimation> for ThornsAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ThornsAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(
                ThornsAnimation::Spawning,
                4,
                Duration::from_secs_f32(1. / 4.),
            )
            .add_frames(
                ThornsAnimation::Present,
                2,
                Duration::from_secs_f32(1. / 4.),
            )
            .add_frames(
                ThornsAnimation::Despawning,
                4,
                Duration::from_secs_f32(1. / 4.),
            )
            .build()
    }
}

#[derive(Resource)]
pub struct ThornsTimer(pub Timer);

pub const THORNS_DURATION: f32 = 3.;
pub const THORNS_COOLDOWN: f32 = 15.;

pub fn thorns_update(
    q_player: Query<(Entity, &Player, &Transform)>,
    mut q_thorns: Query<
        (
            Entity,
            &mut Transform,
            &AnimationController<ThornsAnimation>,
        ),
        (Without<Player>, Without<Enemy>),
    >,
    mut animation_ev: EventWriter<AnimationStateChangeEvent<ThornsAnimation>>,
    mut hit_ev: EventReader<TookDamageEvent>,
    mut timer: ResMut<ThornsTimer>,
    animations: Res<AnimationStateStorage<ThornsAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    pause: Res<ActionPauseState>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let (player_entity, player, player_pos) = q_player.single();

    if !player.abilities.contains(&super::ability::Ability::Thorns) {
        return;
    }

    if pause.is_paused {
        return;
    }

    timer.0.tick(time.delta());
    let thorns = q_thorns.get_single_mut();

    // No thorns
    if thorns.is_err() {
        if timer.0.just_finished() {
            timer.0.set_duration(Duration::from_secs_f32(1.));
            timer.0.reset();

            let texture_atlas = TextureAtlas::from_grid(
                textures.thorns.clone(),
                Vec2 { x: 64., y: 64. },
                10,
                1,
                None,
                None,
            );
            let texture_atlas_handle = texture_atlases.add(texture_atlas);

            commands
                .spawn(make_animation_bundle(
                    ThornsAnimation::Spawning,
                    &animations,
                    texture_atlas_handle,
                    Vec3 {
                        x: 0.,
                        y: 0.,
                        z: SortingLayers::Front.into(),
                    },
                    1.,
                ))
                .insert(Collider::new_circle(55.))
                .insert(Projectile {
                    damage_target: DamageTarget::Team(Team::Enemy),
                    dmg: 50,
                    piercing_mode: PiercingMode::All,
                    entities_hit: vec![],
                    is_alive: true,
                });
        }
    } else {
        let (thorns_entity, mut thorns_transform, thorns_controller) = thorns.unwrap();

        thorns_transform.translation = Vec3 {
            x: player_pos.translation.x,
            y: player_pos.translation.y,
            z: thorns_transform.translation.z,
        };

        match thorns_controller.get_state() {
            ThornsAnimation::Spawning => {
                if timer.0.just_finished() {
                    animation_ev.send(AnimationStateChangeEvent {
                        id: thorns_entity,
                        state_id: ThornsAnimation::Present,
                    });

                    timer
                        .0
                        .set_duration(Duration::from_secs_f32(THORNS_DURATION));
                    timer.0.reset();
                }
            }
            ThornsAnimation::Present => {
                if timer.0.just_finished() {
                    animation_ev.send(AnimationStateChangeEvent {
                        id: thorns_entity,
                        state_id: ThornsAnimation::Despawning,
                    });

                    timer.0.set_duration(Duration::from_secs_f32(1.));
                    timer.0.reset();
                }
            }
            ThornsAnimation::Despawning => {
                if timer.0.just_finished() {
                    commands.entity(thorns_entity).despawn_recursive();

                    timer
                        .0
                        .set_duration(Duration::from_secs_f32(THORNS_COOLDOWN));
                    timer.0.reset();
                }
            }
        }
    }
}
