use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;
use bevy_kira_audio::AudioControl;

use crate::{audio::FXChannel, loading::AudioAssets};

#[derive(Component)]
pub struct Experience {
    pub curr_experience: u32,
    pub level: u32,
    pub threshold: u32,
    pub pick_distance: f32,
}

#[derive(Event)]
pub struct LevelUpEvent {
    pub new_level: u32,
}

pub fn experience_update(
    mut q_xp: Query<&mut Experience>,
    mut level_up_ev: EventWriter<LevelUpEvent>,
    fx_channel: Res<FXChannel>,
    audio: Res<AudioAssets>,
) {
    let mut xp = q_xp.single_mut();

    if xp.curr_experience >= xp.threshold {
        xp.curr_experience -= xp.threshold;
        xp.level += 1;
        xp.threshold = (xp.threshold as f32 * 1.5) as u32;

        level_up_ev.send(LevelUpEvent {
            new_level: xp.level,
        });

        fx_channel.play(audio.levelup.clone());
    }
}
