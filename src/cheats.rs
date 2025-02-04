use bevy::prelude::*;

use crate::{
    collision::collider_debug::ColliderDebugSpriteState,
    combat::health::DeathEvent,
    experience::experience::{Experience, LevelUpEvent},
    player::Player,
    GameState, enemies::spawn_menu::SpawnMenuState,
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
    state: Res<State<ColliderDebugSpriteState>>,
    mut next_state: ResMut<NextState<ColliderDebugSpriteState>>,
    spawn_menu_state : Res<State<SpawnMenuState>>,
    mut next_spawn_menu_state : ResMut<NextState<SpawnMenuState>>,
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

    if keyboard_input.just_pressed(KeyCode::M) {
        next_spawn_menu_state.set(match spawn_menu_state.get() {
            SpawnMenuState::Off => SpawnMenuState::On,
            SpawnMenuState::On => SpawnMenuState::Off,
        })
    }

    if keyboard_input.just_pressed(KeyCode::J) {
        match state.get() {
            ColliderDebugSpriteState::Off => next_state.set(ColliderDebugSpriteState::On),
            ColliderDebugSpriteState::On => next_state.set(ColliderDebugSpriteState::Off),
        };
    }
}
