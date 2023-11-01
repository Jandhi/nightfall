use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

use crate::animation::info::AnimationStateInfo;
use crate::animation::{make_animation_bundle, AnimationStateStorage, Animation};
use crate::collision::collider::{Collider, IsCollidingEvent};

use crate::combat::{
    health::{DeathEvent, Health},
    healthbar::NeedsHealthBar,
    teams::{Team, TeamMember},
};
use crate::constants::{SortingLayers, DISTANCE_SCALING};
use crate::loading::TextureAssets;
use crate::movement::friction::Friction;
use crate::movement::pause::ActionPauseState;
use crate::movement::velocity::Velocity;
use crate::player::Player;
use crate::util::radians::Radian;

use super::ai::FollowPlayerAI;
use super::beholder::{spawn_beholder, BeholderAnimation};
use super::imp::ImpAnimation;

#[derive(Copy, Clone)]
pub enum EnemyType {
    Imp,
    Beholder,
    Zombie
}


#[derive(Component, Clone)]
pub struct Enemy {
    pub enemy_type : EnemyType,
    pub xp: u32,
}



#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub enemy: Enemy,
    pub location : Vec3, 
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
            screen_print!("Despawn");
            commands.entity(entity).despawn_recursive();
            ememy_death_event.send(EnemyDeathEvent {
                entity,
                enemy: enemy.clone(),
                location: transform.translation,
            });
        }
    }
}

pub fn spread_enemies(
    mut collisions : EventReader<IsCollidingEvent>,
    mut q_enemies : Query<&mut Transform, With<Enemy>>
) {
    for collision_event in collisions.iter() {
        if let Ok(mut entities) = q_enemies.get_many_mut([collision_event.collision.entity_a, collision_event.collision.entity_b]) {
            let force = 1.0;
            
            let (slice_a, slice_b) = &mut entities.split_at_mut(1);
            let a_transform = &mut slice_a[0];
            let b_transform = &mut slice_b[0];
            let diff = (a_transform.translation - b_transform.translation).normalize();
            a_transform.translation += diff * force;
            b_transform.translation -= diff * force;
        }
    }
}

pub fn initial_spawn(
    imp_animations: Res<AnimationStateStorage<ImpAnimation>>,
    beholder_animations: Res<AnimationStateStorage<BeholderAnimation>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
) {
    spawn_beholder(Vec3 { x: 30., y: 30., z: SortingLayers::Action.into() }, &beholder_animations, &textures, &mut texture_atlases, &mut commands);
}


