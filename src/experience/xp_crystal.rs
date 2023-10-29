

use bevy::{prelude::*};

use rand::Rng;

use crate::{enemies::enemy::{EnemyDeathEvent}, util::{rng::{GlobalSeed, RNG}, radians::Radian}, movement::{velocity::Velocity, friction::Friction, magnetic::Magnetic, edge_teleport::EdgeTeleports}, loading::TextureAssets, constants::{SCALING_VEC3}, player::Player};

use super::experience::Experience;

#[derive(Resource)]
pub struct CrystalRNG(pub RNG);

#[derive(Component)]
pub struct XPCrystal;

pub fn create_rng(
    seed : Res<GlobalSeed>,
    mut commands : Commands,
) {
    commands.insert_resource(CrystalRNG(RNG::new(
        seed.0.as_str(), 
        "crystal_rng",
    )))
}

#[derive(Bundle)]
pub struct XPCrystalBundle {
    pub spirte_bundle : SpriteBundle,
    pub crystal : XPCrystal,
    pub velocity : Velocity,
    pub friction: Friction,
    pub magnetic : Magnetic
}

pub fn drop_crystals(
    mut enemy_death_event : EventReader<EnemyDeathEvent>,
    mut crystal_rng : ResMut<CrystalRNG>,
    textures: Res<TextureAssets>,
    mut commands : Commands,
) {
    for death_ev in enemy_death_event.iter() {
        for _i in 0..death_ev.enemy.xp {

            let rng = &mut crystal_rng.0.0;

            let velocity  : f32 = rng.gen_range(20.0 .. 50.0);
            let direction : Radian = Radian { angle: rng.gen_range(Radian::ZERO.angle .. Radian::FULL.angle) };

            commands.spawn(XPCrystalBundle{
                spirte_bundle: SpriteBundle {
                        transform: Transform { 
                        translation: death_ev.location, 
                        rotation: default(), 
                        scale: SCALING_VEC3 
                    },
                        texture: textures.texture_crystal.clone(), 
                        ..Default::default()
                },
                crystal: XPCrystal,
                velocity: (direction.unit_vector() * velocity).into(),
                friction: Friction{
                    force: 50.0,
                },
                magnetic: Magnetic{
                    force: 1000000.0,
                }
            }).insert(EdgeTeleports);
        }
        
    }
}

pub fn xp_crystal_update(
    q_crystals : Query<(Entity, &Transform), (With<XPCrystal>, Without<Player>)>,
    mut q_player : Query<(&Transform, &mut Experience), (With<Player>, Without<XPCrystal>)>,
    mut commands : Commands
) {
    let (player_transform, mut  experience) = q_player.single_mut();

    for (entity, crystal_transform) in q_crystals.iter() {
        let distance = crystal_transform.translation.distance(player_transform.translation);
        if distance < experience.xp_pickup_distance {
            commands.entity(entity).despawn();
            experience.curr_experience += 1;
        }
    }
}