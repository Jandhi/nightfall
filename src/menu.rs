use crate::collision::collider_debug::ColliderDebugSpriteState;
use crate::loading::FontAssets;
use crate::movement::pause::PauseMenuComponent;
use crate::palette::Palette;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup_menu)
            .add_systems(Update, click_play_button.run_if(in_state(GameState::Menu)))
            .add_systems(OnExit(GameState::Menu), cleanup_menu);
    }
}

fn setup_menu(mut commands: Commands, font_assets: Res<FontAssets>, palette: Res<Palette>) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Percent(100.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                grid_auto_flow: GridAutoFlow::Row,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(
                TextBundle::from_section(
                    "Nightfall",
                    TextStyle {
                        font: font_assets.gothic_pxl.clone(),
                        font_size: 150.0,
                        color: palette.orange,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }),
            );

            parent.spawn(
                TextBundle::from_section(
                    "By Jan Dohring",
                    TextStyle {
                        font: font_assets.gothic.clone(),
                        font_size: 40.0,
                        color: palette.white,
                    },
                )
                .with_style(Style {
                    margin: UiRect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                }),
            );

            parent
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
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: font_assets.gothic.clone(),
                            font_size: 40.0,
                            color: palette.white,
                        },
                    ));
                });
        });
}

fn click_play_button(
    palette: Res<Palette>,
    mut state: ResMut<NextState<GameState>>,
    _cd_state: ResMut<NextState<ColliderDebugSpriteState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (
            Changed<Interaction>,
            With<Button>,
            Without<PauseMenuComponent>,
        ),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                state.set(GameState::Playing);
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

fn cleanup_menu(mut commands: Commands, ui: Query<Entity, With<Node>>) {
    for entity in ui.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
