use bevy::prelude::*;

use bevy_kira_audio::AudioControl;
use rand::Rng;

use crate::{
    audio::FXChannel,
    constants::{SortingLayers, SCALING_VEC3},
    enemies::enemy::EnemyDeathEvent,
    loading::{AudioAssets, TextureAssets},
    movement::{
        edge_teleport::EdgeTeleports, fake_magnetic::FakeMagnetic, friction::Friction,
        velocity::Velocity,
    },
    player::Player,
    util::{
        radians::Radian,
        rng::{GlobalSeed, RNG},
    },
};

use super::experience::Experience;

#[derive(Resource)]
pub struct CrystalRNG(pub RNG);

#[derive(Component)]
pub struct XPCrystal {
    contained_xp: u32,
}

pub fn create_xp_crystal_rng(seed: Res<GlobalSeed>, mut commands: Commands) {
    commands.insert_resource(CrystalRNG(RNG::new(seed.0.as_str(), "crystal_rng")))
}

#[derive(Bundle)]
pub struct XPCrystalBundle {
    pub sprite_bundle: SpriteBundle,
    pub crystal: XPCrystal,
    pub velocity: Velocity,
    pub friction: Friction,
    pub magnetic: FakeMagnetic,
}

pub const BIG_XP_AMT: u32 = 40;
pub fn drop_crystals(
    mut enemy_death_event: EventReader<EnemyDeathEvent>,
    mut crystal_rng: ResMut<CrystalRNG>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
) {
    for death_ev in enemy_death_event.iter() {
        let mut xp_to_spawn = death_ev.enemy.xp;
        while xp_to_spawn > 0 {
            let rng = &mut crystal_rng.0 .0;
            let velocity: f32 = rng.gen_range(20.0..50.0);
            let direction: Radian = Radian {
                angle: rng.gen_range(Radian::ZERO.angle..Radian::FULL.angle),
            };

            let is_big = xp_to_spawn >= BIG_XP_AMT;

            commands
                .spawn(XPCrystalBundle {
                    sprite_bundle: SpriteBundle {
                        transform: Transform {
                            translation: Vec3 {
                                x: death_ev.location.x,
                                y: death_ev.location.y,
                                z: SortingLayers::BehindAction.into(),
                            },
                            rotation: default(),
                            scale: SCALING_VEC3,
                        },
                        texture: match is_big {
                            true => textures.big_crystal.clone(),
                            false => textures.crystal.clone(),
                        },
                        ..Default::default()
                    },
                    crystal: XPCrystal {
                        contained_xp: match is_big {
                            true => BIG_XP_AMT,
                            false => 1,
                        },
                    },
                    velocity: (direction.unit_vector() * velocity).into(),
                    friction: Friction { force: 50.0 },
                    magnetic: FakeMagnetic { force: 1_000_000.0 },
                })
                .insert(EdgeTeleports);

            xp_to_spawn -= match is_big {
                true => BIG_XP_AMT,
                false => 1,
            };
        }
    }
}

pub fn xp_crystal_update(
    q_crystals: Query<(Entity, &Transform, &XPCrystal), Without<Player>>,
    mut q_player: Query<(&Transform, &mut Experience), (With<Player>, Without<XPCrystal>)>,
    fx_channel: Res<FXChannel>,
    audio: Res<AudioAssets>,
    mut commands: Commands,
) {
    let (player_transform, mut experience) = q_player.single_mut();

    for (entity, crystal_transform, crystal) in q_crystals.iter() {
        let _distance = crystal_transform
            .translation
            .distance(player_transform.translation);
        let distance = crystal_transform
            .translation
            .distance(player_transform.translation);
        if distance < experience.pick_distance {
            commands.entity(entity).despawn_recursive();
            experience.curr_experience += crystal.contained_xp;
            fx_channel.play(match crystal.contained_xp {
                BIG_XP_AMT => audio.big_crystal.clone(),
                _ => audio.coin.clone(),
            });
        }
    }
}
