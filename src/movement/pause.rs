use bevy::{math::bool, prelude::*, sprite::Anchor};
use bevy_kira_audio::AudioControl;

use crate::{
    audio::{FXChannel, Music, MusicChannel, Volume, FX},
    collision::collider::Collider,
    constants::SortingLayers,
    loading::{FontAssets, TextureAssets},
    palette::Palette,
    ui::bar::{Bar, BarButtonInfo, BarUpdatedEvent},
};

#[derive(Resource)]
pub struct ActionPauseState {
    pub is_paused: bool,
}

#[derive(Component)]
pub struct PauseMenuComponent;

#[derive(Event)]
pub struct TogglePauseMenu;

#[derive(Resource)]
pub struct PauseMenuState(pub bool);

pub fn pause_keypress(
    mut enter_ev: EventWriter<TogglePauseMenu>,
    keyboard_input: Res<Input<KeyCode>>,
    pause_menu_state: Res<PauseMenuState>,
    pause: Res<ActionPauseState>,
) {
    if !pause_menu_state.0 && pause.is_paused {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        enter_ev.send(TogglePauseMenu);
    }
}

pub fn update_volume_bars(
    q_music_bar: Query<Entity, With<MusicBar>>,
    q_fx_bar: Query<Entity, With<FXBar>>,
    mut music_volume: ResMut<Volume<Music>>,
    mut fx_volume: ResMut<Volume<FX>>,
    mut music_channel: ResMut<MusicChannel>,
    mut fx_channel: ResMut<FXChannel>,
    mut bar_update: EventReader<BarUpdatedEvent>,
) {
    for ev in bar_update.iter() {
        if q_music_bar.get(ev.entity).is_ok() {
            music_volume.set_volume(ev.new_val as f32 / 10., &mut music_channel);
        }

        if q_fx_bar.get(ev.entity).is_ok() {
            fx_volume.set_volume(ev.new_val as f32 / 10., &mut fx_channel);
        }
    }
}

#[derive(Component)]
pub struct FXBar;

#[derive(Component)]
pub struct MusicBar;

pub fn update_pause_menu(
    menu_items: Query<Entity, With<PauseMenuComponent>>,
    mut enter_ev: EventReader<TogglePauseMenu>,
    mut pause_menu_state: ResMut<PauseMenuState>,
    mut pause: ResMut<ActionPauseState>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    font_assets: Res<FontAssets>,
    palette: Res<Palette>,
    music_volume: Res<Volume<Music>>,
    fx_volume: Res<Volume<FX>>,
    mut commands: Commands,
) {
    if enter_ev.iter().len() == 0 {
        return;
    }

    for _ev in enter_ev.iter() {
        // let ev be marked as read
    }

    if pause_menu_state.0 {
        pause.is_paused = false;
        pause_menu_state.0 = false;

        for e in menu_items.iter() {
            commands.entity(e).despawn_recursive();
        }

        return;
    }

    pause.is_paused = true;
    pause_menu_state.0 = true;

    commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(250.0),
                height: Val::Px(50.0),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: palette.dark.into(),
            ..Default::default()
        })
        .insert(PauseMenuComponent)
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    "Unpause",
                    TextStyle {
                        font: font_assets.gothic.clone(),
                        font_size: 40.0,
                        color: palette.white,
                    },
                ))
                .insert(PauseMenuComponent);
        });

    let bar_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        textures.bar.clone(),
        Vec2 { x: 64., y: 64. },
        11,
        1,
        None,
        None,
    ));

    let button_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        textures.next_button.clone(),
        Vec2 { x: 64., y: 64. },
        3,
        1,
        None,
        None,
    ));

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "Music Volume",
                TextStyle {
                    font: font_assets.gothic_pxl.clone(),
                    font_size: 30.,
                    color: palette.orange,
                },
            ),
            text_anchor: Anchor::Center,
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 180.,
                z: SortingLayers::UI.into(),
            }),
            ..Default::default()
        })
        .insert(PauseMenuComponent);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "FX Volume",
                TextStyle {
                    font: font_assets.gothic_pxl.clone(),
                    font_size: 30.,
                    color: palette.orange,
                },
            ),
            text_anchor: Anchor::Center,
            transform: Transform::from_translation(Vec3 {
                x: 0.,
                y: 110.,
                z: SortingLayers::UI.into(),
            }),
            ..Default::default()
        })
        .insert(PauseMenuComponent);

    Bar::spawn(
        (music_volume.volume() * 10.) as u32,
        10,
        SpriteSheetBundle {
            texture_atlas: bar_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 150.,
                    z: SortingLayers::UI.into(),
                },
                rotation: default(),
                scale: Vec3 {
                    x: 3.,
                    y: 3.,
                    z: 1.,
                },
            },
            ..Default::default()
        },
        Some(BarButtonInfo {
            sprite: SpriteSheetBundle {
                texture_atlas: button_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: -40.,
                        y: 0.,
                        z: SortingLayers::UI.into(),
                    },
                    rotation: default(),
                    scale: Vec3 {
                        x: -1.,
                        y: 1.,
                        z: 1.,
                    },
                },
                ..Default::default()
            },
            has_pressed_state: true,
            collider: Collider::new_rect(Vec2 { x: 20., y: 30. }),
        }),
        Some(BarButtonInfo {
            sprite: SpriteSheetBundle {
                texture_atlas: button_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3 {
                    x: 40.,
                    y: 0.,
                    z: SortingLayers::UI.into(),
                }),
                ..Default::default()
            },
            has_pressed_state: true,
            collider: Collider::new_rect(Vec2 { x: 20., y: 30. }),
        }),
        Some((PauseMenuComponent, MusicBar)),
        &mut commands,
    );

    Bar::spawn(
        (fx_volume.volume() * 10.) as u32,
        10,
        SpriteSheetBundle {
            texture_atlas: bar_atlas_handle.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 80.,
                    z: SortingLayers::UI.into(),
                },
                rotation: default(),
                scale: Vec3 {
                    x: 3.,
                    y: 3.,
                    z: 1.,
                },
            },
            ..Default::default()
        },
        Some(BarButtonInfo {
            sprite: SpriteSheetBundle {
                texture_atlas: button_atlas_handle.clone(),
                transform: Transform {
                    translation: Vec3 {
                        x: -40.,
                        y: 0.,
                        z: SortingLayers::UI.into(),
                    },
                    rotation: default(),
                    scale: Vec3 {
                        x: -1.,
                        y: 1.,
                        z: 1.,
                    },
                },
                ..Default::default()
            },
            has_pressed_state: true,
            collider: Collider::new_rect(Vec2 { x: 20., y: 30. }),
        }),
        Some(BarButtonInfo {
            sprite: SpriteSheetBundle {
                texture_atlas: button_atlas_handle.clone(),
                transform: Transform::from_translation(Vec3 {
                    x: 40.,
                    y: 0.,
                    z: SortingLayers::UI.into(),
                }),
                ..Default::default()
            },
            has_pressed_state: true,
            collider: Collider::new_rect(Vec2 { x: 20., y: 30. }),
        }),
        Some((PauseMenuComponent, FXBar)),
        &mut commands,
    );
}

pub fn click_unpause(
    palette: Res<Palette>,
    mut ev_toggle: EventWriter<TogglePauseMenu>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            (With<Button>, With<PauseMenuComponent>),
        ),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                ev_toggle.send(TogglePauseMenu);
            }
            Interaction::Hovered => {
                *color = palette.orange.into();
            }
            Interaction::None => {
                *color = palette.red.into();
            }
        }
    }
}
