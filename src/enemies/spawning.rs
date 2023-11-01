use bevy::prelude::*;

use crate::movement::pause::ActionPauseState;

#[derive(Resource)]
pub struct SpawnInfo {
    pub timer: Timer,
    pub count: u32,
}

pub fn spawn_loop(
    mut spawn_info: ResMut<SpawnInfo>,
    time: Res<Time>,
    pause: Res<ActionPauseState>,
) {
    if pause.is_paused {
        return;
    }

    spawn_info.timer.tick(time.delta());

    if spawn_info.timer.just_finished() {
        spawn_info.timer.reset();
        spawn_info.count += 1;
    }
}
