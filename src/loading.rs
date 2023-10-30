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
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AbilityTextures>(GameState::Loading);
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

    #[asset(path = "audio/gunshot.ogg")]
    pub gunshot: Handle<AudioSource>,

    #[asset(path = "audio/reload.ogg")]
    pub reload: Handle<AudioSource>,

    #[asset(path = "audio/reload_done.ogg")]
    pub reload_done: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/Flame.png")]
    pub flame: Handle<Image>,

    #[asset(path = "textures/tower.png")]
    pub tower: Handle<Image>,

    #[asset(path = "textures/turret.png")]
    pub turret: Handle<Image>,

    #[asset(path = "textures/bullet_small.png")]
    pub bullet_small: Handle<Image>,

    #[asset(path = "textures/bullet_medium.png")]
    pub bullet_medium: Handle<Image>,

    #[asset(path = "textures/bullet_large.png")]
    pub bullet_large: Handle<Image>,

    #[asset(path = "textures/imp.png")]
    pub imp: Handle<Image>,

    #[asset(path = "textures/healthbar.png")]
    pub healthbar: Handle<Image>,

    #[asset(path = "textures/hatman_spritesheet.png")]
    pub hatman: Handle<Image>,

    #[asset(path = "textures/ui/bullet_ui.png")]
    pub bullet_ui: Handle<Image>,

    #[asset(path = "textures/ui/xp_bar.png")]
    pub xp_bar: Handle<Image>,

    #[asset(path = "textures/ui/reload.png")]
    pub reload_ui: Handle<Image>,

    #[asset(path = "textures/ui/health.png")]
    pub heart_ui: Handle<Image>,

    #[asset(path = "textures/crystal.png")]
    pub crystal: Handle<Image>,

    


}

#[derive(AssetCollection, Resource)]
pub struct AbilityTextures {
    #[asset(path = "textures/abilities/frame.png")]
    pub frame: Handle<Image>,

    #[asset(path = "textures/abilities/big_bullets.png")]
    pub big_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/crossbow.png")]
    pub crossbow: Handle<Image>,

    #[asset(path = "textures/abilities/double_barrel.png")]
    pub double_barrel: Handle<Image>,

    #[asset(path = "textures/abilities/flaming_bullets.png")]
    pub flaming_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/shells.png")]
    pub shells: Handle<Image>,

    #[asset(path = "textures/abilities/sniper.png")]
    pub sniper: Handle<Image>,
}