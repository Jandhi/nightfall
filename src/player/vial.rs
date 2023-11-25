use bevy::prelude::*;

use crate::{enemies::enemy::EnemyDeathEvent, ui::{alignment::{AnchorBundle, Alignment}, element::{UIElement, SizeVec2, SizeConstraint}, offset::Offset}, loading::TextureAssets, GameState};

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
}

fn vial_update(
    mut q_vial : Query<(&mut Vial, &mut TextureAtlasSprite)>,
    q_player : Query<&Player, Without<Vial>>,
    mut death_ev : EventReader<EnemyDeathEvent>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands : Commands,
) {
    let player = q_player.single();

    if !player.abilities.contains(&Ability::BloodthirstyVial) {
        return;
    }

    let vial_result = q_vial.get_single();

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

        commands.spawn(AnchorBundle::new(Alignment::BottomRight))
        .insert(Offset{
            amount: SizeVec2 {
                x: SizeConstraint::Const(100.),
                y: SizeConstraint::Const(100.),
            },
        }).insert(SpriteSheetBundle{
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_translation(Vec3 { x: 1000., y: 1000., z: 0. }),
            ..Default::default()
        });
        return;
    }

    let vial = vial_result.unwrap();

    let mut is_healing = false;

    for death in death_ev.iter() {
        
    }
}