use bevy::{prelude::*, window::PrimaryWindow};

use crate::{loading::TextureAssets, collision::collider::Collider, constants::SCALING_VEC3, animation::{make_animation_bundle, AnimationStateStorage}};

pub type Health = u32;

#[derive(Component, Clone)]
pub struct Enemy {
    pub track_progress : f32,
    pub health : Health,
}

#[derive(Event)]
pub struct EnemyDeathEvent {
    pub entity:  Entity,
    pub enemy: Enemy
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ImpAnimationState {
    FLYING,
}

impl Enemy {
    pub fn take_damage(&mut self, dmg : Health) {
        if dmg > self.health {
            self.health = 0;
        } else {
            self.health -= dmg;
        }
    }

    pub fn estimate_position(&self, transform : &Transform, time : f32) -> Vec2 {
        transform.translation.truncate()
    }
}

// Get it? Like the game?
pub fn death_loop(
    mut death_event : EventWriter<EnemyDeathEvent>,
    mut q_enemies : Query<(Entity, &Enemy)>,
    mut commands : Commands
) {
    for (entity, enemy) in q_enemies.iter_mut() {
        // Skip healthy entities
        if enemy.health > 0 {
            continue;
        }

        commands.entity(entity).despawn();
        death_event.send(EnemyDeathEvent { entity: entity, enemy: enemy.clone() });
    }
}

pub fn spawn_enemy(
    imp_animations : Res<AnimationStateStorage<ImpAnimationState>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
    mut commands: Commands, 
    textures: Res<TextureAssets>
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_imp.clone(),
         Vec2 { x: 32., y: 32. },
          4,
           1,
            None,
             None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Enemy{ track_progress: 0., health: 100 })
        .insert(Collider::new_circle(10., Vec2 { x: 70., y: 70. }))
        .insert(make_animation_bundle(ImpAnimationState::FLYING, imp_animations, texture_atlas_handle));
}

pub fn follow_mouse(
    mut q_enemies : Query<(Entity, &mut Transform), With<Enemy>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = q_windows.single();
    if let Some(cursor_position) = window.cursor_position() {
        let target = Vec2::new(
            cursor_position.x - window.width() / 2.,
            window.height() / 2. - cursor_position.y,
        );

        for (_, mut transform) in q_enemies.iter_mut() {
            transform.translation = Vec3{
                x: target.x,
                y: target.y,
                z: transform.translation.z
            };
        }
    }
}