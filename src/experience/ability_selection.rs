use std::iter;

use bevy::{prelude::*, window::PrimaryWindow};
use rand::seq::IteratorRandom;

use crate::{util::rng::{RNG, GlobalSeed}, player::{Player, ability::Ability}, movement::pause::ActionPauseState, ui::{grid::{Grid, GridElement}, selection_group::{SelectionGroup, HoverEvent, UnhoverEvent}}, loading::{TextureAssets, AbilityTextures}, constants::{SortingLayers, SCALING_VEC3}, animation::{make_animation_bundle, Animation, info::{AnimationStateInfo, AnimationInfoBuilder}, AnimationStateStorage, controller::AnimationController, AnimationStateChangeEvent}};

use super::experience::LevelUpEvent;

#[derive(Resource)]
pub struct AbilityRNG(pub RNG);

pub fn create_ability_selection_rng(
    seed : Res<GlobalSeed>,
    mut commands : Commands,
) {
    commands.insert_resource(AbilityRNG(RNG::new(
        seed.0.as_str(), 
        "ability_rng",
    )))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AbilityFrameAnimation {
    Hovered,
    NonHovered
}

impl Animation<AbilityFrameAnimation> for AbilityFrameAnimation {
    fn get_states() -> Vec<AnimationStateInfo<AbilityFrameAnimation>> {
        AnimationInfoBuilder::new()
            .add_single(AbilityFrameAnimation::NonHovered)
            .add_single(AbilityFrameAnimation::Hovered)
            .build()
    }
}

pub fn ability_frame_update(
    mut q_frames : Query<(Entity, &AnimationController<AbilityFrameAnimation>)>,
    mut animation_update : EventWriter<AnimationStateChangeEvent<AbilityFrameAnimation>>,
    mut hover_events : EventReader<HoverEvent>,
    mut unhover_events : EventReader<UnhoverEvent>,
) {
    for hover_ev in hover_events.iter() {
        if let Ok((entity, _)) = q_frames.get(hover_ev.hovered) {
            animation_update.send(AnimationStateChangeEvent { id: entity, state_id: AbilityFrameAnimation::Hovered });
        }
    }

    for unhover_ev in unhover_events.iter() {
        if let Ok((entity, _)) = q_frames.get(unhover_ev.unhovered) {
            animation_update.send(AnimationStateChangeEvent { id: entity, state_id: AbilityFrameAnimation::NonHovered });
        }
    }
}

pub fn start_ability_selection(
    mut q_player : Query<&mut Player>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut level_up_ev : EventReader<LevelUpEvent>,
    textures : Res<AbilityTextures>,
    frame_animations: Res<AnimationStateStorage<AbilityFrameAnimation>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rng : ResMut<AbilityRNG>,
    mut pause : ResMut<ActionPauseState>,
    mut commands : Commands,
    
) {
    if level_up_ev.iter().len() == 0 {
        return;
    }

    pause.is_paused = true;

    let window = q_windows.single();
    let mut player = q_player.single_mut();

    let texture_atlas = TextureAtlas::from_grid(
        textures.frame.clone(),
        Vec2 { x: 32., y: 32. },
        2,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);


    let all_abilities = Ability::all();
    let chosen_abilities = all_abilities.iter()
        .filter(|ability| !player.abilities.contains(*ability))
        .choose_multiple(&mut rng.0.0, 3);
    let ability1 = chosen_abilities[0];
    let ability2 = chosen_abilities[1];
    let ability3 = chosen_abilities[2];

    commands.spawn(
        Grid {
            size: Vec2 { x: window.width(), y: 0. },
            grid_size: IVec2 { x: 3, y: 1 },
        }
    ).insert(
        SelectionGroup{
            is_focused: true,
            hovered_index: 0,
            is_horizontal: true,
        }
    ).insert(
        SpriteBundle::default()
    ).with_children(|parent| {
        for i in 0..3 {
            parent.spawn(make_animation_bundle(
                match i {
                    0 => AbilityFrameAnimation::Hovered,
                    _ => AbilityFrameAnimation::NonHovered,
                }, 
                &frame_animations, 
                texture_atlas_handle.clone(), 
                Vec3::ZERO)
            ).insert(
                GridElement{ index: IVec2{ x: i, y: 0 } }
            ).with_children(|parent|{
                parent.spawn(SpriteBundle{
                    texture: match i {
                        0 => ability1,
                        1 => ability2,
                        _ => ability3
                    }.get_texture(&textures),
                    ..Default::default()
                });
            });
        }
    });
}