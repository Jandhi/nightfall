use std::f32::consts::PI;

use bevy::prelude::*;

use crate::animation::{make_animation_bundle, AnimationStateStorage};
use crate::collision::collider::Collider;
use crate::combat::{
    health::{DeathEvent, Health},
    healthbar::NeedsHealthBar,
    teams::{Team, TeamMember},
};
use crate::constants::{SortingLayers, DISTANCE_SCALING};
use crate::loading::TextureAssets;
use crate::player::Player;
use crate::util::radians::Radian;

#[derive(Component, Clone)]
pub struct Enemy {
    pub track_progress: f32,
    pub speed: f32,
    pub xp: u32,
}

#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub enemy: Enemy,
    pub location : Vec3, 
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImpAnimationState {
    FLYING,
}

impl Enemy {
    pub fn estimate_position(&self, transform: &Transform, _time: f32) -> Vec2 {
        transform.translation.truncate()
    }
}

// Get it? Like the game?
pub fn death_loop(
    mut ememy_death_event: EventWriter<EnemyDeathEvent>,
    mut death_event: EventReader<DeathEvent>,
    mut q_enemies: Query<(Entity, &Enemy, &Transform)>,
    mut commands: Commands,
) {
    for death_ev in death_event.iter() {
        if let Ok((entity, enemy, transform)) = q_enemies.get_mut(death_ev.entity) {
            commands.entity(entity).despawn();
            ememy_death_event.send(EnemyDeathEvent {
                entity,
                enemy: enemy.clone(),
                location: transform.translation,
            });
        }
    }
}

pub fn spawn_enemy(
    imp_animations: Res<AnimationStateStorage<ImpAnimationState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_imp.clone(),
        Vec2 { x: 32., y: 32. },
        4,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for i in 0..5 {
        commands
        .spawn(Enemy {
            track_progress: 0.,
            speed: 0.3,
            xp: 5,
        })
        .insert(Health::new(15))
        .insert(Collider::new_circle(10., Vec2 { x: 70., y: 70. }))
        .insert(make_animation_bundle(
            ImpAnimationState::FLYING,
            &imp_animations,
            texture_atlas_handle.clone(),
            Vec3 {
                x: -30. + (i as f32) * 15.,
                y: 30.,
                z: SortingLayers::Action.into(),
            },
        ))
        .insert(TeamMember { team: Team::Enemy })
        .insert(NeedsHealthBar::default());
    }
}

pub fn follow_player(
    mut q_enemies: Query<(&mut Transform, &Enemy)>,
    q_player: Query<&Transform, (With<Player>, Without<Enemy>)>,
) {
    let player_transform = q_player.single();

    for (mut enemy_transform, enemy) in q_enemies.iter_mut() {
        let direction =
            player_transform.translation.truncate() - enemy_transform.translation.truncate();
        // obtain angle to target with respect to x-axis.
        let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
        let direction_vec = angle_to_target.unit_vector();

        if direction.length() < enemy.speed * DISTANCE_SCALING {
            enemy_transform.translation = Vec3 {
                x: player_transform.translation.x,
                y: player_transform.translation.y,
                z: enemy_transform.translation.z,
            }
        } else {
            enemy_transform.translation = Vec3 {
                x: enemy_transform.translation.x + direction_vec.x * enemy.speed * DISTANCE_SCALING,
                y: enemy_transform.translation.y + direction_vec.y * enemy.speed * DISTANCE_SCALING,
                z: enemy_transform.translation.z,
            }
        }
    }
}
