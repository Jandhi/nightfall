use std::{time::Duration, f32::consts::PI};

use bevy::prelude::*;

use crate::{animation::{AnimationStateStorage, make_animation_bundle, info::{AnimationStateInfo, AnimationInfoBuilder}, Animation, AnimationStateChangeEvent, controller::AnimationController}, loading::TextureAssets, movement::velocity::Velocity, combat::{health::Health, teams::{TeamMember, Team}, healthbar::NeedsHealthBar, projectile::{Projectile, DamageTarget, PiercingMode}}, collision::collider::Collider, player::Player, util::radians::Radian};

use super::{enemy::{Enemy, EnemyType}, ai::{MoveAndShootAI, ShootEvent, ChargeShootEvent}};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BeholderAnimation {
    Flying,
    Shoot
}

impl Animation<BeholderAnimation> for BeholderAnimation {
    fn get_states() -> Vec<AnimationStateInfo<BeholderAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(BeholderAnimation::Flying, 16, Duration::from_secs_f32(1. / 8.))
            .add_frames(BeholderAnimation::Shoot, 6, Duration::from_secs_f32(1. / 8.))
            .build()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum BeholderProjectileAnimation {
    Flying,
}

impl Animation<BeholderProjectileAnimation> for BeholderProjectileAnimation {
    fn get_states() -> Vec<AnimationStateInfo<BeholderProjectileAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(BeholderProjectileAnimation::Flying, 4, Duration::from_secs_f32(1. / 4.))
            .build()
    }
}

pub fn beholder_update(
    q_beholders :  Query<(Entity, &Transform, &AnimationController<BeholderAnimation>), Without<Player>>,
    q_player : Query<(Entity, &Transform), With<Player>>,
    mut shoot_ev : EventReader<ShootEvent>,
    mut charge_ev : EventReader<ChargeShootEvent>,
    mut animate : EventWriter<AnimationStateChangeEvent<BeholderAnimation>>,
    beholder_projetile_animations : Res<AnimationStateStorage<BeholderProjectileAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands : Commands
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.beholder_projectile.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for charge in charge_ev.iter() {
        if let Ok((entity, _, _)) = q_beholders.get(charge.entity) {
            animate.send(AnimationStateChangeEvent { id: entity, state_id: BeholderAnimation::Shoot })
        } 
    }

    for shoot in shoot_ev.iter() {
        if let Ok((entity, transform, _)) = q_beholders.get(shoot.entity) {
            animate.send(AnimationStateChangeEvent { id: entity, state_id: BeholderAnimation::Flying });
            let (_player_entity, player_transform) = q_player.single();

            let direction = player_transform.translation.truncate() - transform.translation.truncate();
            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
            let direction_vec = angle_to_target.unit_vector();

            commands
                .spawn(make_animation_bundle(BeholderProjectileAnimation::Flying, &beholder_projetile_animations, texture_atlas_handle.clone(), transform.translation))
                .insert(Projectile {
                    dmg: 1,
                    damage_target: DamageTarget::Team(Team::Player),
                    piercing_mode: PiercingMode::None,
                    entities_hit: 0,
                    is_alive: true,
                })
                .insert(Velocity {
                    vec: direction_vec * 20.,
                })
                .insert(Collider::new_circle(15., transform.translation.truncate()));
        }
    }
}

pub fn spawn_beholder(
    position : Vec3,
    animations: &Res<AnimationStateStorage<BeholderAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.beholder.clone(),
        Vec2 { x: 32., y: 32. },
        22,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 10,
            enemy_type: EnemyType::Beholder,
        })
        .insert(MoveAndShootAI::new(
            20., 
            3., 
            200., 
            6./8., 
            2.
        ))
        .insert(Velocity::ZERO)
        .insert(Health::new(15))
        .insert(Collider::new_circle(12., Vec2 { x: 70., y: 70. }))
        .insert(make_animation_bundle(
            BeholderAnimation::Flying,
            animations,
            texture_atlas_handle.clone(),
            position,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}