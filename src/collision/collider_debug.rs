use bevy::{prelude::*, ecs::system::Command};

use crate::{loading::DebugTextureAssets, constants::SortingLayers};

use super::collider::{Collider, ColliderShape, IsCollidingEvent, Collision};

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ColliderDebugSpriteState {
    #[default]
    Off,
    On,
}

#[derive(Component)]
pub struct ColliderDebugSprite;

#[derive(Component)]
pub struct HasColliderDebugSprite;


pub fn spawn_colliders_sprites(
    q_colliders : Query<(Entity, &Collider)>,
    mut commands : Commands
) {
    for (entity, collider) in q_colliders.iter() {
        commands.add(AddDebugCollider{ parent: entity, collider: collider.clone() });
    }
}

pub fn despawn_colliders_sprites(
    q_sprites : Query<Entity, With<ColliderDebugSprite>>,
    q_parents : Query<Entity, (With<HasColliderDebugSprite>, Without<ColliderDebugSprite>)>,
    mut commands : Commands,
) {
    for sprite in q_sprites.iter() {
        commands.entity(sprite).despawn_recursive();
    }

    for parent in q_parents.iter() {
        commands.entity(parent).remove::<HasColliderDebugSprite>();
    }
}

pub fn update_collider_sprites(
    mut q_sprites : Query<(&mut Transform, &Parent, &mut Sprite), With<ColliderDebugSprite>>,
    mut q_errant_sprites : Query<Entity, (With<ColliderDebugSprite>, Without<Parent>)>,
    q_colliders : Query<(&Collider, &GlobalTransform), (With<HasColliderDebugSprite>, Without<ColliderDebugSprite>)>,
    q_spriteless_colliders : Query<(Entity, &Collider), (Without<HasColliderDebugSprite>, Without<ColliderDebugSprite>)>,
    mut ev_is_colliding : EventReader<IsCollidingEvent>,
    textures : Res<DebugTextureAssets>,
    mut commands : Commands
) {
    let current_collisions  = ev_is_colliding.iter().map(|ev| ev.collision.clone()).collect::<Vec<Collision>>();

    for (mut transform, parent, mut sprite) in q_sprites.iter_mut() {
        if let Ok((collider, global_transform)) = q_colliders.get(parent.get()) {
            let scale = global_transform.to_scale_rotation_translation().0;
            transform.scale = match collider.shape() {
                ColliderShape::Rect(size) => {
                    Vec3 {
                        x: size.x / 128. / scale.x,
                        y: size.y / 128. / scale.y,
                        z: 1.,
                    }
                },
                ColliderShape::Circle(radius) => {
                    Vec3 {
                        x: 2. * radius / 128. / scale.x,
                        y: 2. * radius / 128. / scale.y,
                        z: 1.,
                    }
                },
            };
        }

        let is_colliding = current_collisions.iter().any(|c| c.contains(parent.get()));
        
        if is_colliding {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::WHITE;
        }
    }

    for sprite in q_errant_sprites.iter() {
        commands.entity(sprite).despawn_recursive();
    }

    for (entity, collider) in q_spriteless_colliders.iter() {
        commands.add(AddDebugCollider{ parent: entity, collider: collider.clone() });
    }
}

#[derive(Debug)]
pub struct AddDebugCollider {
    parent: Entity,
    collider : Collider,
}

impl Command for AddDebugCollider {
    fn apply(self, world: &mut World) {
        let textures = world.remove_resource::<DebugTextureAssets>().expect("We should have access to debug textures");

        match world.get_entity_mut(self.parent) {
            Some(mut entity) => {
                let scale = entity.get::<GlobalTransform>().expect("Parent should have transform").clone().to_scale_rotation_translation().0;

                entity.with_children(|parent| {
                    parent.spawn(SpriteBundle{
                        texture: match self.collider.shape() {
                            ColliderShape::Rect(_) => textures.rect.clone(),
                            ColliderShape::Circle(_) => textures.circle.clone(),
                        },
                        transform: Transform{
                            translation: Vec3 { x: 0., y: 0., z: SortingLayers::DebugFront.into() },
                            rotation: default(),
                            scale: match self.collider.shape() {
                                ColliderShape::Rect(size) => {
                                    Vec3 {
                                        x: size.x / 128. / scale.x,
                                        y: size.y / 128. / scale.y,
                                        z: 1.,
                                    }
                                },
                                ColliderShape::Circle(radius) => {
                                    Vec3 {
                                        x: radius / 128. / scale.x,
                                        y: radius / 128. / scale.y,
                                        z: 1.,
                                    }
                                },
                            },
                        },
                        ..default()
                    }).insert(ColliderDebugSprite);
                });

                entity.insert(HasColliderDebugSprite);                
            },
            None => (),
        };

        world.insert_resource(textures);
    }
}