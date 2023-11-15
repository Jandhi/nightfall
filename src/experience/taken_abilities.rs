use bevy::{prelude::*, window::{WindowResized, PrimaryWindow}, text::Text2dBounds, sprite::Anchor};

use crate::{player::ability::Ability, loading::{AbilityTextures, FontAssets}, constants::SortingLayers, collision::collider::{Collider}, palette::Palette};

#[derive(Component)]
pub struct TakenAbility {
    pub index : usize,
    pub ability : Ability
}

pub fn spawn_taken(
    ability : Ability,
    index : usize,
    window : &Window,
    textures : &Res<AbilityTextures>,
    commands : &mut Commands,
) {
    commands.spawn(
        SpriteBundle{
            transform: Transform::from_translation(Vec3 { 
                x: window.width() / -2. + 24. + 32. * index as f32,
                    y: window.height() / -2. + 24.,
                z: SortingLayers::UI.into() }),
            texture: ability.get_texture(textures),
            ..Default::default()
        }
    ).insert(TakenAbility{
        index,
        ability,
    }).insert(Collider::new_rect(Vec2 { x: 30., y: 30. }, Vec2 { 
        x: window.width() / -2. + 24. + 32. * index as f32,
        y: window.height() / -2. + 24.,
    }));
}

#[derive(Component)]
pub struct TakenTitle;

#[derive(Component)]
pub struct TakenDescription; 


pub fn update_description(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_takens : Query<(&Transform, &TakenAbility, &Collider), Without<Window>>,
    mut q_taken_title : Query<(Entity, &mut Text), (With<TakenTitle>, Without<Window>, Without<TakenAbility>)>,
    mut q_taken_description : Query<(Entity, &mut Text), (With<TakenDescription>, Without<Window>, Without<TakenAbility>, Without<TakenTitle>)>,
    font_assets: Res<FontAssets>,
    palette: Res<Palette>,
    mut commands : Commands
) {
    let window = q_windows.single();

    if let Some(cursor_position) = window.cursor_position() {
        let cursor_point = Vec2::new(
            cursor_position.x - window.width() / 2.,
            window.height() / 2. - cursor_position.y,
        );

        for (transform, taken, collider) in q_takens.iter() {            
            if !collider.contains_point(transform.translation.truncate(), cursor_point) {
                continue;
            }

            if let (Ok((_, mut title)), Ok((_, mut desc))) = (q_taken_title.get_single_mut(), q_taken_description.get_single_mut()) {
                title.sections[0].value = taken.ability.get_name();
                desc.sections[0].value = taken.ability.get_description();
            } else {
                let ability = taken.ability;
                commands.spawn(Text2dBundle {
                    text : Text::from_section(ability.get_name(), TextStyle 
                    { 
                        font: font_assets.gothic_pxl.clone(), font_size: 96., color: palette.orange,
                    }).with_alignment(TextAlignment::Center),
                    transform: Transform { 
                        translation: Vec3 { x: 0., y: 200., z: SortingLayers::UI.into() },
                        rotation: default(), 
                        scale: Vec3 { x: 0.5, y: 0.5, z: 1. },
                    },
                    text_2d_bounds : Text2dBounds {
                        size: Vec2 { x: 300., y: 50. },
                    },
                    ..Default::default()
                }).insert(TakenTitle);

                commands.spawn(Text2dBundle {
                    text : Text::from_section(ability.get_description(), TextStyle 
                    { 
                        font: font_assets.garamond.clone(), font_size: 48., color: palette.white,
                    }).with_alignment(TextAlignment::Center),
                    transform: Transform { 
                        translation: Vec3 { x: 0., y: 170., z: SortingLayers::UI.into() }, 
                        rotation: default(), 
                        scale: Vec3 { x: 0.5, y: 0.5, z: 1. }, 
                    },
                    text_2d_bounds : Text2dBounds {
                        size: Vec2 { x: 200., y: 500. },
                    },
                    text_anchor: Anchor::TopCenter,
                    ..Default::default()
                }).insert(TakenDescription);
            }

            return;
        }

        if let (Ok((title, _)), Ok((desc, _))) = (q_taken_title.get_single(), q_taken_description.get_single()) {
            commands.entity(title).despawn();
            commands.entity(desc).despawn();
        } 
    }
}

pub fn update_taken_positions(
    mut ev_resize : EventReader<WindowResized>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    mut q_takens : Query<(&mut Transform, &TakenAbility)>,
) {
    for resize in ev_resize.iter() {
        if let Ok(window) = q_window.get(resize.window) {
            for (mut transform, taken) in q_takens.iter_mut() {
                transform.translation = Vec3{
                    x: window.width() / -2. + 24. + 32. * taken.index as f32,
                    y: window.height() / -2. + 24.,
                    z: SortingLayers::UI.into(),
                };

            }
        }
    }
}