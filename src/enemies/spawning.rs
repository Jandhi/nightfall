use bevy::prelude::*;

use crate::{movement::pause::ActionPauseState, animation::AnimationStateStorage, loading::TextureAssets, constants::SortingLayers};

use super::{imp::{spawn_imp, ImpAnimation}, beholder::{spawn_beholder, BeholderAnimation}};

#[derive(Resource)]
pub struct SpawnInfo{
    pub timer : Timer,
    pub count : u32,
}

pub fn spawn_loop(
    mut spawn_info : ResMut<SpawnInfo>,
    time : Res<Time>,
    pause : Res<ActionPauseState>,
    imp_animations: Res<AnimationStateStorage<ImpAnimation>>,
    beholder_animations: Res<AnimationStateStorage<BeholderAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,

) {
    if pause.is_paused {
        return;
    }

    spawn_info.timer.tick(time.delta());

    if spawn_info.timer.just_finished() {
        spawn_info.timer.reset();

        let position = Vec3 { x: 0., y: 100., z: SortingLayers::Action.into() };

        spawn_imp(position, &imp_animations, &textures, &mut texture_atlases, &mut commands);
        spawn_beholder(position, &beholder_animations, &textures, &mut texture_atlases, &mut commands);
        spawn_info.count += 1;
    }
}