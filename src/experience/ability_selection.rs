use std::slice::Windows;

use bevy::{prelude::*, window::PrimaryWindow, text::{Text2dBounds, TextLayoutInfo}, sprite::Anchor};
use rand::seq::IteratorRandom;

use crate::{
    animation::{
        controller::AnimationController,
        info::{AnimationInfoBuilder, AnimationStateInfo},
        make_animation_bundle, Animation, AnimationStateChangeEvent, AnimationStateStorage,
    },
    collision::collider::Collider,
    combat::health::Health,
    constants::SortingLayers,
    loading::{AbilityTextures, FontAssets},
    movement::pause::ActionPauseState,
    palette::Palette,
    player::{ability::Ability, Player},
    ui::{
        grid::{Grid, GridElement},
        selection_group::{
            HoverEvent, SelectionElement, SelectionEvent, SelectionGroup, UnhoverEvent,
        },
    },
    util::rng::{GlobalSeed, RNG},
};

use super::experience::LevelUpEvent;

#[derive(Component)]
pub struct AbilitySelectionMenuItem;

#[derive(Resource)]
pub struct AbilityRNG(pub RNG);

pub fn create_ability_selection_rng(seed: Res<GlobalSeed>, mut commands: Commands) {
    commands.insert_resource(AbilityRNG(RNG::new(seed.0.as_str(), "ability_rng")))
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AbilityFrameAnimation {
    Hovered,
    NonHovered,
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
    q_frames: Query<(Entity, &AnimationController<AbilityFrameAnimation>)>,
    mut animation_update: EventWriter<AnimationStateChangeEvent<AbilityFrameAnimation>>,
    mut hover_events: EventReader<HoverEvent>,
    mut unhover_events: EventReader<UnhoverEvent>,
) {
    for hover_ev in hover_events.iter() {
        if let Ok((entity, _)) = q_frames.get(hover_ev.hovered) {
            animation_update.send(AnimationStateChangeEvent {
                id: entity,
                state_id: AbilityFrameAnimation::Hovered,
            });
        }
    }

    for unhover_ev in unhover_events.iter() {
        if let Ok((entity, _)) = q_frames.get(unhover_ev.unhovered) {
            animation_update.send(AnimationStateChangeEvent {
                id: entity,
                state_id: AbilityFrameAnimation::NonHovered,
            });
        }
    }
}

#[derive(Component)]
pub struct AbilitySelection {
    abilities: Vec<Ability>,
}

pub fn on_select_ability(
    q_menu: Query<(Entity, &AbilitySelection)>,
    mut q_player: Query<(&mut Player, &mut Health), Without<AbilitySelection>>,
    mut q_selection_items : Query<(Entity), (With<AbilitySelectionMenuItem>, Without<AbilitySelection>, Without<Player>)>,
    mut selection_events: EventReader<SelectionEvent>,
    mut commands: Commands,
    mut pause: ResMut<ActionPauseState>,
) {
    let (mut player, mut health) = q_player.single_mut();

    for selection_ev in selection_events.iter() {
        if let Ok((entity, selection)) = q_menu.get(selection_ev.parent) {
            player
                .abilities
                .push(selection.abilities[selection_ev.selected_index]);

            if selection.abilities[selection_ev.selected_index] == Ability::MaxHp {
                health.max += 1;
                health.value += 1;
            }

            if selection.abilities[selection_ev.selected_index] == Ability::Potion {
                health.value += 2;
            }

            commands.entity(entity).despawn_recursive();
            pause.is_paused = false;

            for e in q_selection_items.iter() {
                commands.entity(e).despawn();
            }
        }
    }
}

pub fn start_ability_selection(
    mut q_player: Query<&mut Player>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut level_up_ev: EventReader<LevelUpEvent>,
    textures: Res<AbilityTextures>,
    frame_animations: Res<AnimationStateStorage<AbilityFrameAnimation>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut rng: ResMut<AbilityRNG>,
    font_assets: Res<FontAssets>,
    palette: Res<Palette>,
    mut pause: ResMut<ActionPauseState>,
    mut commands: Commands,
) {
    if level_up_ev.iter().len() == 0 {
        return;
    }

    if pause.is_paused {
        return;
    }

    pause.is_paused = true;

    let window = q_windows.single();
    let player = q_player.single_mut();

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
    let chosen_abilities = all_abilities
        .iter()
        .filter(|ability| ability.is_available(&player.abilities))
        .choose_multiple(&mut rng.0 .0, 3);

    commands
        .spawn(Grid {
            size: Vec2 {
                x: window.width(),
                y: 0.,
            },
            grid_size: IVec2 { x: 3, y: 1 },
        })
        .insert(AbilitySelection {
            abilities: chosen_abilities.iter().map(|a| **a).collect(),
        })
        .insert(SelectionGroup {
            is_focused: true,
            hovered_index: 0,
            is_horizontal: true,
        })
        .insert(SpriteBundle {
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 0.,
                z: SortingLayers::UI.into(),
            }),
            ..Default::default()
        })
        .with_children(|parent| {
            for i in 0..3 {
                parent
                    .spawn(make_animation_bundle(
                        match i {
                            0 => AbilityFrameAnimation::Hovered,
                            _ => AbilityFrameAnimation::NonHovered,
                        },
                        &frame_animations,
                        texture_atlas_handle.clone(),
                        Vec3::ZERO,
                        1.,
                    ))
                    .insert(GridElement {
                        index: IVec2 { x: i, y: 0 },
                    })
                    .with_children(|parent| {
                        let ability = chosen_abilities[i as usize];

                        parent.spawn(SpriteBundle {
                            texture: ability.get_texture(&textures),
                            transform: Transform::from_translation(Vec3 {
                                x: 0.,
                                y: 0.,
                                z: SortingLayers::UI.into(),
                            }),
                            ..Default::default()
                        });

                        parent.spawn(Text2dBundle {
                            text : Text::from_section(ability.get_name(), TextStyle 
                            { 
                                font: font_assets.gothic_pxl.clone(), font_size: 48., color: palette.orange,
                            }).with_alignment(TextAlignment::Center),
                            transform: Transform { 
                                translation: Vec3 { x: 0., y: -40., z: SortingLayers::UI.into() },
                                rotation: default(), 
                                scale: Vec3 { x: 0.5, y: 0.5, z: 1. },
                            },
                            text_2d_bounds : Text2dBounds {
                                size: Vec2 { x: 300., y: 50. },
                            },
                            ..Default::default()
                        });

                        parent.spawn(Text2dBundle {
                            text : Text::from_section(ability.get_description(), TextStyle 
                            { 
                                font: font_assets.garamond.clone(), font_size: 24., color: palette.white,
                            }).with_alignment(TextAlignment::Center),
                            transform: Transform { 
                                translation: Vec3 { x: 0., y: -55., z: SortingLayers::UI.into() }, 
                                rotation: default(), 
                                scale: Vec3 { x: 0.5, y: 0.5, z: 1. }, 
                            },
                            text_2d_bounds : Text2dBounds {
                                size: Vec2 { x: 200., y: 500. },
                            },
                            text_anchor: Anchor::TopCenter,
                            ..Default::default()
                        });
                    })
                    .insert(SelectionElement { index: i as usize })
                    .insert(Collider::new_rect(Vec2 { x: 64., y: 64. }, Vec2::ZERO));
            }
        });

}
