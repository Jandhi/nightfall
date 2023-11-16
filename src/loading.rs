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

    #[asset(path = "fonts/GothicPixels.ttf")]
    pub gothic: Handle<Font>,

    #[asset(path = "fonts/gothic-pixel-font.ttf")]
    pub gothic_pxl: Handle<Font>,

    #[asset(path = "fonts/EBGaramond-MediumItalic.TTF")]
    pub garamond: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,

    #[asset(path = "audio/levelup.ogg")]
    pub levelup: Handle<AudioSource>,

    #[asset(path = "audio/big_crystal.ogg")]
    pub big_crystal: Handle<AudioSource>,

    #[asset(path = "audio/coin.ogg")]
    pub coin: Handle<AudioSource>,

    #[asset(path = "audio/gunshot.ogg")]
    pub gunshot: Handle<AudioSource>,

    #[asset(path = "audio/gunshot_2.ogg")]
    pub gunshot2: Handle<AudioSource>,

    #[asset(path = "audio/grunt.ogg")]
    pub grunt: Handle<AudioSource>,

    #[asset(path = "audio/reaper_death.ogg")]
    pub reaper_death: Handle<AudioSource>,

    #[asset(path = "audio/blade.ogg")]
    pub blade: Handle<AudioSource>,

    #[asset(path = "audio/reload.ogg")]
    pub reload: Handle<AudioSource>,

    #[asset(path = "audio/reload_done.ogg")]
    pub reload_done: Handle<AudioSource>,

    #[asset(path = "audio/beholder_death.ogg")]
    pub beholder_death: Handle<AudioSource>,

    #[asset(path = "audio/beholder_prince_death.ogg")]
    pub beholder_prince_death: Handle<AudioSource>,

    #[asset(path = "audio/fireball.ogg")]
    pub fireball: Handle<AudioSource>,

    #[asset(path = "audio/imp_death.ogg")]
    pub imp_death: Handle<AudioSource>,

    #[asset(path = "audio/imp_death2.ogg")]
    pub imp_death2: Handle<AudioSource>,

    #[asset(path = "audio/imp_death3.ogg")]
    pub imp_death3: Handle<AudioSource>,

    #[asset(path = "audio/imp_death4.ogg")]
    pub imp_death4: Handle<AudioSource>,

    #[asset(path = "audio/theme.ogg")]
    pub theme: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "textures/Flame.png")]
    pub flame: Handle<Image>,

    #[asset(path = "textures/thorns.png")]
    pub thorns: Handle<Image>,

    #[asset(path = "textures/fire.png")]
    pub fire: Handle<Image>,

    #[asset(path = "textures/hit.png")]
    pub hit: Handle<Image>,

    #[asset(path = "textures/bullet_small.png")]
    pub bullet_small: Handle<Image>,

    #[asset(path = "textures/bullet_medium.png")]
    pub bullet_medium: Handle<Image>,

    #[asset(path = "textures/bullet_large.png")]
    pub bullet_large: Handle<Image>,

    #[asset(path = "textures/enemies/imp.png")]
    pub imp: Handle<Image>,

    #[asset(path = "textures/enemies/imp_mother.png")]
    pub imp_queen: Handle<Image>,

    #[asset(path = "textures/enemies/beholder.png")]
    pub beholder: Handle<Image>,

    #[asset(path = "textures/enemies/beholder_prince.png")]
    pub beholder_prince: Handle<Image>,

    #[asset(path = "textures/enemies/beholder_projectile.png")]
    pub beholder_projectile: Handle<Image>,

    #[asset(path = "textures/enemies/reaper.png")]
    pub reaper: Handle<Image>,

    #[asset(path = "textures/enemies/reaper_blade.png")]
    pub reaper_blade: Handle<Image>,

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

    #[asset(path = "textures/big_crystal.png")]
    pub big_crystal: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct AbilityTextures {
    #[asset(path = "textures/abilities/frame.png")]
    pub frame: Handle<Image>,

    #[asset(path = "textures/abilities/big_bullets.png")]
    pub big_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/biggest_bullets.png")]
    pub biggest_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/bullets_galore.png")]
    pub bullets_galore: Handle<Image>,

    #[asset(path = "textures/abilities/crossbow.png")]
    pub crossbow: Handle<Image>,

    #[asset(path = "textures/abilities/double_barrel.png")]
    pub double_barrel: Handle<Image>,

    #[asset(path = "textures/abilities/potion.png")]
    pub potion: Handle<Image>,
    
    #[asset(path = "textures/abilities/piercing.png")]
    pub piercing: Handle<Image>,

    #[asset(path = "textures/abilities/max_hp.png")]
    pub max_hp: Handle<Image>,

    #[asset(path = "textures/abilities/faster.png")]
    pub faster: Handle<Image>,

    #[asset(path = "textures/abilities/triple_barrel.png")]
    pub triple_barrel: Handle<Image>,

    #[asset(path = "textures/abilities/flaming_bullets.png")]
    pub flaming_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/hotter_fire.png")]
    pub hotter_fire: Handle<Image>,

    #[asset(path = "textures/abilities/medium_bullets.png")]
    pub medium_bullets: Handle<Image>,

    #[asset(path = "textures/abilities/reload.png")]
    pub reload: Handle<Image>,

    #[asset(path = "textures/abilities/shooting_speed.png")]
    pub shooting_speed: Handle<Image>,

    #[asset(path = "textures/abilities/sixfold.png")]
    pub sixfold: Handle<Image>,

    #[asset(path = "textures/abilities/thorns.png")]
    pub thorns: Handle<Image>,

    #[asset(path = "textures/abilities/shells.png")]
    pub shells: Handle<Image>,

    #[asset(path = "textures/abilities/sniper.png")]
    pub sniper: Handle<Image>,

    #[asset(path = "textures/abilities/shotgun.png")]
    pub shotgun: Handle<Image>,

    #[asset(path = "textures/abilities/mega_shotgun.png")]
    pub mega_shotgun: Handle<Image>,
}
