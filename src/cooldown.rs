use std::{default, time::Duration};
use crate::GameState; 
use bevy::prelude::*;
use bevy::log::LogPlugin;


#[derive(Component)]
pub struct Cooldown {
    pub time_remaining : Duration
}

#[derive(Event)]
pub struct CooldownFinishedEvent(Entity);


fn tick_cooldown(
    mut ev_cooldown_finished : EventWriter<CooldownFinishedEvent>,
    mut cooldowns : Query<(Entity, &mut Cooldown)>,
    time : Res<Time>
) {
    for (entity, mut cooldown) in cooldowns.iter_mut() {
        if cooldown.time_remaining == Duration::ZERO {
            // We are done here
            return;
        } else if time.delta() > cooldown.time_remaining {
            // Cooldown finished
            cooldown.time_remaining = Duration::ZERO;
            ev_cooldown_finished.send(CooldownFinishedEvent(entity));
        }
        else {
            cooldown.time_remaining -= time.delta();
        }
    }
}

pub struct CooldownPlugin;

impl Plugin for CooldownPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, tick_cooldown.run_if(in_state(GameState::Playing)))
            .add_event::<CooldownFinishedEvent>();
    }
}

impl Cooldown {
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.time_remaining.is_zero()
    }
}

impl Default for Cooldown {
    fn default() -> Self {
        Self { time_remaining: Duration::ZERO }
    }
}