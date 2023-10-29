use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "textures/Flame.png")]
    pub texture_flame: Handle<Image>,

    #[asset(path = "textures/tower.png")]
    pub texture_tower: Handle<Image>,

    #[asset(path = "textures/turret.png")]
    pub texture_turret: Handle<Image>,

    #[asset(path = "textures/bullet_small.png")]
    pub texture_bullet_small: Handle<Image>,

    #[asset(path = "textures/bullet_medium.png")]
    pub texture_bullet_medium: Handle<Image>,

    #[asset(path = "textures/bullet_large.png")]
    pub texture_bullet_large: Handle<Image>,

    #[asset(path = "textures/imp.png")]
    pub texture_imp: Handle<Image>,

    #[asset(path = "textures/healthbar.png")]
    pub texture_healthbar: Handle<Image>,

    #[asset(path = "textures/hatman_spritesheet.png")]
    pub texture_hatman: Handle<Image>,

    #[asset(path = "textures/ui/bullet_ui.png")]
    pub texture_bullet_ui: Handle<Image>,

    #[asset(path = "textures/ui/xp_bar.png")]
    pub texture_xp_bar: Handle<Image>,

    #[asset(path = "textures/ui/reload.png")]
    pub texture_reload_ui: Handle<Image>,

    #[asset(path = "textures/ui/health.png")]
    pub texture_heart_ui: Handle<Image>,

    #[asset(path = "textures/crystal.png")]
    pub texture_crystal: Handle<Image>,
}
