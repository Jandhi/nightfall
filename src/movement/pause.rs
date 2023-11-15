use bevy::{prelude::*, math::bool, input::keyboard::KeyboardInput, a11y::accesskit::Action};
use bevy_debug_text_overlay::screen_print;

use crate::{loading::FontAssets, palette::Palette, menu};

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
    mut enter_ev : EventWriter<TogglePauseMenu>,
    keyboard_input : Res<Input<KeyCode>>,
    pause_menu_state : Res<PauseMenuState>,
    pause : Res<ActionPauseState>,
) {
    if !pause_menu_state.0 && pause.is_paused {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        enter_ev.send(TogglePauseMenu);
    }
}

pub fn update_pause_menu(
    menu_items : Query<Entity, With<PauseMenuComponent>>,
    mut enter_ev : EventReader<TogglePauseMenu>,
    mut pause_menu_state : ResMut<PauseMenuState>,
    mut pause : ResMut<ActionPauseState>,
    font_assets: Res<FontAssets>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    if enter_ev.iter().len() == 0 {
        return;
    }

    for ev in enter_ev.iter() {
        // let ev be marked as read
    }

    if pause_menu_state.0 {
        pause.is_paused = false;
        pause_menu_state.0 = false;

        for e in menu_items.iter() {
            commands.entity(e).despawn();
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
            parent.spawn(TextBundle::from_section(
                "Unpause",
                TextStyle {
                    font: font_assets.gothic.clone(),
                    font_size: 40.0,
                    color: palette.white,
                },
            )).insert(PauseMenuComponent);
        });

}

pub fn click_unpause(
    palette: Res<Palette>,
    mut ev_toggle : EventWriter<TogglePauseMenu>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, (With<Button>, With<PauseMenuComponent>)),
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