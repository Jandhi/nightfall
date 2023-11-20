use std::marker::PhantomData;

use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::prelude::{*, Volume as KiraVolume};

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_systems(OnEnter(GameState::Menu), start_music)
            .add_systems(OnEnter(GameState::Playing), start_audio)
            .add_audio_channel::<FX>()
            .add_audio_channel::<Music>()
            .insert_resource(Volume::<Music>::new())
            .insert_resource(Volume::<FX>::new());
    }
}

#[derive(Resource)]
pub struct Volume<T : Send + Sync + 'static> {
    data : PhantomData<T>,
    volume : f32,
}

impl<T : Send + Sync + 'static> Volume<T> {
    pub fn new() -> Volume<T> {
        default()
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn set_volume(&mut self, value : f32, channel : &mut ResMut<AudioChannel<T>>) {
        self.volume = value;
        channel.set_volume(self);
    }
}

impl<T : Send + Sync + 'static> Into<KiraVolume> for &mut Volume<T> {
    fn into(self) -> KiraVolume {
        KiraVolume::Amplitude(self.volume.into())
    }
}

impl<T : Send + Sync + 'static> Default for Volume<T> {
    fn default() -> Self {
        Self { data: Default::default(), volume: 1. }
    }
}

#[derive(Resource)]
pub struct FX;
pub type FXChannel = AudioChannel<FX>;

#[derive(Resource)]
pub struct Music;

pub type MusicChannel = AudioChannel<Music>;

fn start_audio(_commands: Commands, _audio_assets: Res<AudioAssets>, _audio: Res<Audio>) {}

fn start_music(music: Res<MusicChannel>, audio_assets: Res<AudioAssets>) {
    music
        .play(audio_assets.theme.clone())
        .with_volume(0.9)
        .looped();
}