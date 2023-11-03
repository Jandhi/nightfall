#![allow(clippy::type_complexity)]

mod actions;
mod animation;
mod audio;
mod cheats;
mod collision;
mod combat;
mod constants;
mod enemies;
mod experience;
mod loading;
mod menu;
mod movement;
mod palette;
mod player;
mod ui;
mod util;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::enemies::EnemiesPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::palette::PalettePlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use cheats::CheatsPlugin;
use collision::CollisionPlugin;
use combat::CombatPlugin;
use experience::ExperiencePlugin;
use movement::MovementPlugin;
use ui::UIPlugin;
use util::UtilPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_plugins((
            PalettePlugin,
            LoadingPlugin,
            MenuPlugin,
            ActionsPlugin,
            InternalAudioPlugin,
            PlayerPlugin,
            CombatPlugin,
            EnemiesPlugin,
            CollisionPlugin,
            ExperiencePlugin,
            UtilPlugin,
            MovementPlugin,
            UIPlugin,
            CheatsPlugin,
        ));

        #[cfg(debug_assertions)]
        {
            app.add_plugins((FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin::default()));
        }
    }
}
