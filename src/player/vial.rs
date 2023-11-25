use std::time::Duration;

use bevy::prelude::*;

use crate::{enemies::enemy::EnemyDeathEvent, ui::{alignment::{AnchorBundle, Alignment, AlignedBundle}, element::{UIElement, SizeVec2, SizeConstraint}, offset::Offset}, loading::TextureAssets, GameState, combat::health::Health, constants::SortingLayers};

use super::{Player, ability::Ability};

pub struct VialPlugin;

impl Plugin for VialPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, vial_update.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
struct Vial{
    pub count : i32,
    animation_timer : Timer,
    animation_count : i32,
}

fn vial_update(
    mut q_vial : Query<(&mut Vial, &mut TextureAtlasSprite)>,
    mut q_player : Query<(&Player, &mut Health), Without<Vial>>,
    mut death_ev : EventReader<EnemyDeathEvent>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time : Res<Time>,
    mut commands : Commands,
) {
    let (player, mut health) = q_player.single_mut();

    if !player.abilities.contains(&Ability::BloodthirstyVial) {
        return;
    }

    let vial_result = q_vial.get_single_mut();

    if let Err(_) = vial_result {
        let texture_atlas = TextureAtlas::from_grid(
            textures.vial.clone(),
            Vec2 { x: 64., y: 64. },
            24,
            1,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        commands.spawn(AlignedBundle::new(Alignment::BottomRight, default()))
        .insert(Offset{
            amount: SizeVec2 {
                x: SizeConstraint::Const(-50.),
                y: SizeConstraint::Const(50.),
            },
        }).insert(SpriteSheetBundle{
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3 { x: 1000., y: 1000., z: SortingLayers::UI.into() }),
            ..Default::default()
        }).insert(Vial{
            count: 0,
            animation_count: 0,
            animation_timer: Timer::from_seconds(1. / 8., TimerMode::Repeating)
        });
        return;
    }

    let (mut vial, mut atlas) = vial_result.unwrap();
    vial.animation_timer.tick(time.delta());

    let mut is_healing = false;

    for death in death_ev.iter() {
        vial.count += 1;
        if vial.count >= 100 {
            vial.count = 0;
            is_healing = true;
        }
    }

    if is_healing {
        if health.value < health.max {
            health.value += 1;
        }

        vial.animation_count = 1;
        atlas.index = 22;
        vial.animation_timer.reset();
    }

    match vial.animation_count {
        4 => {
            if vial.animation_timer.just_finished() {
                vial.animation_count = 0;
            }
        }
        3 => {
            if vial.animation_timer.just_finished() {
                atlas.index = 23;
                vial.animation_count += 1;
            }
        }
        2 => {
            if vial.animation_timer.just_finished() {
                atlas.index = 22;
                vial.animation_count += 1;
            }
        }
        1 => {
            if vial.animation_timer.just_finished() {
                atlas.index = 23;
                vial.animation_count += 1;
            }
        }
        _ => {
            if vial.count == 0 {
                atlas.index = 0;
            } else {
                atlas.index = (vial.count as f32 / 100. * 22. + 1.) as usize;
            }
        }
    }
}