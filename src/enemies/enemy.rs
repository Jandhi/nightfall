use bevy::prelude::*;

use bevy_kira_audio::AudioControl;
use rand::Rng;


use crate::audio::FXChannel;
use crate::collision::collider::{Collider, IsCollidingEvent};

use crate::combat::health::{DeathEvent, Health};
use crate::combat::teams::TeamMember;
use crate::combat::z_sort::ZSort;

use crate::loading::{AudioAssets, TextureAssets};

use crate::movement::velocity::Velocity;
use crate::util::pitch_rng::PitchRNG;



#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum EnemyType {
    Imp,
    ImpQueen,
    Beholder,
    BeholderPrince,
    Reaper,
}

impl EnemyType {
    pub fn all() -> Vec<EnemyType> {
        vec![
            EnemyType::Imp,
            EnemyType::ImpQueen,
            EnemyType::Beholder,
            EnemyType::BeholderPrince,
            EnemyType::Reaper,
        ]
    }

    pub fn difficulty(&self) -> f32 {
        match self {
            EnemyType::Imp => 5.0,
            EnemyType::ImpQueen => 50.0,
            EnemyType::Beholder => 10.0,
            EnemyType::BeholderPrince => 100.0,
            EnemyType::Reaper => 120.0,
        }
    }

    pub fn sprite_size(&self) -> Vec2 {
        match self {
            EnemyType::Reaper => Vec2 { x: 64.0, y: 64.0 },
            _ => Vec2 { x: 32.0, y: 32.0 },
        }
    }

    pub fn get_texture(&self, textures : &Res<TextureAssets>) -> Handle<Image> {
        match self {
            EnemyType::Imp => textures.imp.clone(),
            EnemyType::ImpQueen => textures.imp_queen.clone(),
            EnemyType::Beholder => textures.beholder.clone(),
            EnemyType::BeholderPrince => textures.beholder_prince.clone(),
            EnemyType::Reaper => textures.reaper.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            EnemyType::Imp => "Imp",
            EnemyType::ImpQueen => "Imp Queen",
            EnemyType::Beholder => "Beholder",
            EnemyType::BeholderPrince => "BeholderPrince",
            EnemyType::Reaper => "Reaper",
        }.to_string()
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub enemy: Enemy,
    pub z_sort: ZSort,
    pub health: Health,
    pub velocity: Velocity,
    pub collider: Collider,
    pub team: TeamMember,
}

#[derive(Component, Clone)]
pub struct Enemy {
    pub enemy_type: EnemyType,
    pub xp: u32,
}

#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity: Entity,
    pub enemy: Enemy,
    pub location: Vec3,
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
    fx_channel: Res<FXChannel>,
    audio: Res<AudioAssets>,
    mut pitch_rng: ResMut<PitchRNG>,
    mut commands: Commands,
) {
    for death_ev in death_event.iter() {
        if let Ok((entity, enemy, transform)) = q_enemies.get_mut(death_ev.entity) {
            fx_channel.play(match enemy.enemy_type {
                EnemyType::Imp | EnemyType::ImpQueen => match pitch_rng.0 .0.gen_range(0..4) {
                    0 => audio.imp_death.clone(),
                    1 => audio.imp_death2.clone(),
                    2 => audio.imp_death3.clone(),
                    _ => audio.imp_death4.clone(),
                },
                EnemyType::Beholder => audio.beholder_death.clone(),
                EnemyType::BeholderPrince => audio.beholder_prince_death.clone(),
                EnemyType::Reaper => audio.reaper_death.clone(),
            });

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
    mut collisions: EventReader<IsCollidingEvent>,
    mut q_enemies: Query<&mut Transform, With<Enemy>>,
) {
    for collision_event in collisions.iter() {
        if let Ok(mut entities) = q_enemies.get_many_mut([
            collision_event.collision.entity_a,
            collision_event.collision.entity_b,
        ]) {
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
