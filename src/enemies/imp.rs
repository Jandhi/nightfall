use std::time::Duration;

use bevy::prelude::*;

use crate::{
    animation::{
        info::AnimationStateInfo, make_animation_bundle, Animation, AnimationStateStorage,
    },
    collision::collider::Collider,
    combat::{
        health::Health,
        healthbar::NeedsHealthBar,
        teams::{Team, TeamMember}, z_sort::ZSort,
    },
    loading::TextureAssets,
    movement::velocity::Velocity, constants::SortingLayers,
};

use super::{
    ai::FollowPlayerAI,
    enemy::{Enemy, EnemyType, EnemyBundle},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImpAnimation {
    Flying,
}

impl Animation<ImpAnimation> for ImpAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ImpAnimation>> {
        vec![AnimationStateInfo {
            id: ImpAnimation::Flying,
            start_index: 0,
            frame_count: 4,
            frame_duration: Duration::from_secs_f32(1. / 8.),
        }]
    }
}

pub fn spawn_imp(
    position: Vec3,
    imp_animations: &Res<AnimationStateStorage<ImpAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.imp.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(EnemyBundle{
            enemy: Enemy {
                xp: 5,
                enemy_type: EnemyType::Imp,
            },
            z_sort: ZSort{ layer: SortingLayers::Action.into() },
            velocity: Velocity::ZERO,
            health: Health::new(15),
            collider: Collider::new_circle(10., position.truncate()),
            team: TeamMember { team: Team::Enemy },
            needs_health_bar: NeedsHealthBar::default(),
        }
            
            
        )
        .insert(FollowPlayerAI {
            speed: 15.,
            corrective_force: 1.0,
        })
        .insert(FollowPlayerAI {
            speed: 15.,
            corrective_force: 1.0,
        })
        .insert(make_animation_bundle(
            ImpAnimation::Flying,
            imp_animations,
            texture_atlas_handle.clone(),
            position,
            1.,
        ));;
}

#[derive(Component)]
pub struct ImpQueen;

pub fn spawn_imp_queen(
    position: Vec3,
    imp_animations: &Res<AnimationStateStorage<ImpAnimation>>,
    textures: &Res<TextureAssets>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.imp_queen.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Enemy {
            xp: 50,
            enemy_type: EnemyType::Imp,
        })
        .insert(ImpQueen)
        .insert(FollowPlayerAI {
            speed: 12.,
            corrective_force: 3.0,
        })
        .insert(Velocity::ZERO)
        .insert(Health::new(150))
        .insert(Collider::new_circle(15., position.truncate()))
        .insert(make_animation_bundle(
            ImpAnimation::Flying,
            &imp_animations,
            texture_atlas_handle.clone(),
            position,
            1.,
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
}
