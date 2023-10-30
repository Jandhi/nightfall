use crate::actions::{set_movement_actions, Actions};
use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_audio_channel::<FX>()
            .add_audio_channel::<Ambience>();
    }
}

#[derive(Resource)]
pub struct FX;
pub type FXChannel = AudioChannel<FX>;

#[derive(Resource)]
pub struct Ambience;
pub type AmbienceChannel = AudioChannel<Ambience>;


fn start_audio(mut commands: Commands, audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    
}
