use bevy::prelude::*;

use crate::{enemies::spawning::SpawnInfo, loading::FontAssets, palette::Palette};

#[derive(Component)]
pub struct GameTimer;

pub fn spawn_game_timer(
    font_assets: Res<FontAssets>,
    palette: Res<Palette>,
    mut commands: Commands,
) {
    commands
        .spawn(
            TextBundle::from_section(
                "0:00",
                TextStyle {
                    font: font_assets.gothic.clone(),
                    font_size: 40.0,
                    color: palette.white,
                },
            )
            .with_style(Style {
                left: Val::Px(200.0),
                top: Val::Px(10.0),
                ..default()
            }),
        )
        .insert(GameTimer);
}

pub fn update_game_timer(mut q_text: Query<&mut Text, With<GameTimer>>, spawning: Res<SpawnInfo>) {
    let mut text = q_text.single_mut();

    text.sections[0].value = format!(
        "{}:{}{}",
        (spawning.game.elapsed().as_secs() / 60) as u32,
        match spawning.game.elapsed().as_secs() % 60 < 10 {
            true => "0",
            false => "",
        },
        spawning.game.elapsed().as_secs() % 60
    )
}
