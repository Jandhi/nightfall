use bevy::prelude::*;

use crate::{
    combat::health::DeathEvent,
    experience::experience::{Experience, LevelUpEvent},
    player::Player,
    GameState,
};

pub struct CheatsPlugin;

impl Plugin for CheatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (cheats).run_if(in_state(GameState::Playing)));
    }
}

fn cheats(
    q_player: Query<(Entity, &Player, &Experience)>,
    keyboard_input: Res<Input<KeyCode>>,
    mut levelup: EventWriter<LevelUpEvent>,
    mut death_ev: EventWriter<DeathEvent>,
) {
    let (player_entity, _player, xp) = q_player.single();

    if keyboard_input.just_pressed(KeyCode::L) {
        levelup.send(LevelUpEvent {
            new_level: xp.level + 1,
        });
    }

    if keyboard_input.just_pressed(KeyCode::K) {
        death_ev.send(DeathEvent {
            entity: player_entity,
        });
    }
}
