use std::time::Duration;

use bevy::prelude::*;

use crate::{enemies::enemy::EnemyDeathEvent, animation::{Animation, info::{AnimationStateInfo, AnimationInfoBuilder}, AppAnimationSetup, make_animation_bundle, AnimationStateStorage}, player::{Player, ability::Ability}, loading::TextureAssets, movement::pause::ActionPauseState, GameState, collision::collider::Collider};

use super::{projectile::{Projectile, DamageTarget, PiercingMode}, teams::Team};

pub struct DeathrattlePlugin;

impl Plugin for DeathrattlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_animation::<ExplosionAnimation>()
            .add_systems(Update, deathrattle_update.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct ExplosionAnimation;

impl Animation<ExplosionAnimation> for ExplosionAnimation {
    fn get_states() -> Vec<AnimationStateInfo<ExplosionAnimation>> {
        AnimationInfoBuilder::new()
            .add_frames(ExplosionAnimation, 5, Duration::from_secs_f32(1. / 16.))
            .build()
    }
}

#[derive(Component)]
struct Explosion(pub Timer);

fn deathrattle_update(
    q_player : Query<&Player>,
    mut q_explosion : Query<(Entity, &mut Collider, &mut Explosion), Without<Player>>,
    mut death_ev : EventReader<EnemyDeathEvent>,
    animations : Res<AnimationStateStorage<ExplosionAnimation>>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    pause : Res<ActionPauseState>,
    time : Res<Time>,
    mut commands : Commands,
) {
    let player = q_player.single();

    if !player.abilities.contains(&Ability::Deathrattle) {
        return;
    }

    if pause.is_paused {
        return;
    }

    let texture_atlas = TextureAtlas::from_grid(
        textures.explosion.clone(),
        Vec2 { x: 64., y: 64. },
        5,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for death in death_ev.iter() {
        commands.spawn(Projectile{
            damage_target: DamageTarget::Team(Team::Enemy),
            dmg: player.damage(),
            piercing_mode: PiercingMode::All,
            entities_hit: vec![],
            is_alive: true,
        }).insert(make_animation_bundle(
            ExplosionAnimation, 
            &animations, 
            texture_atlas_handle.clone(), 
            death.location, 
            1.0
        ))
        .insert(Collider::new_circle(50.))
        .insert(Explosion(Timer::from_seconds(5. * 1. / 16., TimerMode::Once)));
    }

    for (entity, mut collider, mut timer) in q_explosion.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}