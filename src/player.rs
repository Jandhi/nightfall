use crate::actions::Actions;
use crate::animation::controller::AnimationController;
use crate::animation::{make_animation_bundle, AnimationStateChangeEvent, AppAnimationSetup};
use crate::audio::FXChannel;
use crate::collision::collider::{Collider, IsCollidingEvent};
use crate::combat::health::{Health, HealthType, TookDamageEvent};
use crate::combat::projectile::projectile_collision_check;
use crate::combat::teams::{Team, TeamMember};
use crate::constants::SortingLayers;
use crate::enemies::enemy::Enemy;
use crate::experience::experience::Experience;
use crate::loading::{AudioAssets, TextureAssets};
use crate::movement::edge_teleport::EdgeTeleports;
use crate::movement::pause::ActionPauseState;
use crate::util::pitch_rng::PitchRNG;
use crate::GameState;
use bevy::prelude::*;

use bevy_kira_audio::AudioControl;
use rand::Rng;

use self::ability::Ability;
use self::animations::{PlayerAnimationState, PlayerAnimations};
use self::bullets_ui::{manage_bullet_ui_sprites, BulletUIAnimation, BulletUICount};
use self::health_ui::{manage_health_ui_sprites, HealthUIAnimationState, HealthUICount};
use self::hit::{spawn_hit_sprite, update_hit_sprite};
use self::reload_ui::{spawn_reload_ui, update_reload_ui, ReloadTimer};
use self::shooting::{shoot, ShootingCooldown};

pub mod ability;
mod animations;
mod bullets_ui;
mod health_ui;
mod hit;
mod reload_ui;
mod shooting;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    curr_bullets: u32,
    max_bullets: u32,
    is_reloading: bool,
    pub abilities: Vec<Ability>,
}

impl Player {
    pub fn damage(&self) -> HealthType {
        self.abilities
            .iter()
            .fold(5., |dmg, ability| dmg * ability.damage_mult()) as u32
    }

    pub fn shoot_time(&self) -> f32 {
        self.abilities
            .iter()
            .fold(0.5, |dmg, ability| dmg / ability.shoot_speed_mult())
    }

    pub fn reload_time(&self) -> f32 {
        self.abilities
            .iter()
            .fold(1.0, |dmg, ability| dmg / ability.reload_mult())
    }

    pub fn knockback(&self) -> f32 {
        self.abilities
            .iter()
            .fold(20., |dmg, ability| dmg * ability.knockback_mult())
    }
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing),
            (spawn_player, spawn_reload_ui, spawn_hit_sprite),
        )
        .add_systems(
            Update,
            (
                move_player,
                shoot,
                manage_bullet_ui_sprites,
                manage_health_ui_sprites,
                update_reload_ui,
                enemy_collision,
                update_hit_sprite,
                hit_immunity
                    .after(projectile_collision_check)
                    .after(enemy_collision),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .insert_resource(ReloadTimer(Timer::from_seconds(0., TimerMode::Once)))
        .insert_resource(BulletUICount(0))
        .insert_resource(HealthUICount(0))
        .insert_resource(InvincibilityTimer(Timer::from_seconds(
            3.0,
            TimerMode::Once,
        )))
        .insert_resource(ShootingCooldown(Timer::from_seconds(1.0, TimerMode::Once)))
        .add_animation::<PlayerAnimationState>()
        .add_animation::<BulletUIAnimation>()
        .add_animation::<HealthUIAnimationState>();
    }
}

pub fn spawn_player(
    player_animations: Res<PlayerAnimations>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.hatman.clone(),
        Vec2 { x: 32., y: 32. },
        6,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn(Player {
            max_bullets: 6,
            curr_bullets: 6,
            is_reloading: false,
            abilities: vec![],
        })
        .insert(Collider::new_circle(10., Vec2 { x: 0., y: 0. }))
        .insert(make_animation_bundle(
            PlayerAnimationState::Idle,
            &player_animations,
            texture_atlas_handle,
            Vec3 {
                x: 0.,
                y: 0.,
                z: SortingLayers::Player.into(),
            },
            1.,
        ))
        .insert(Experience {
            curr_experience: 0,
            level: 0,
            threshold: 20,
            pick_distance: 10.0,
        })
        .insert(EdgeTeleports)
        .insert(Health::new(3))
        .insert(TeamMember { team: Team::Player });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut animation_change: EventWriter<AnimationStateChangeEvent<PlayerAnimationState>>,
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut AnimationController<PlayerAnimationState>,
        &mut TextureAtlasSprite,
    )>,
    pause: Res<ActionPauseState>,
) {
    if pause.is_paused {
        return;
    }

    let (_entity, _player_transform, _animation_controller, _) = player_query.single_mut();
    let (entity, mut player_transform, mut animation_controller, _) = player_query.single_mut();

    if actions.player_movement.is_none() {
        if animation_controller.get_state() != PlayerAnimationState::Idle {
            animation_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: PlayerAnimationState::Idle,
            });
        }

        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );

    player_transform.translation += movement;

    if animation_controller.get_state() != PlayerAnimationState::Running {
        animation_change.send(AnimationStateChangeEvent {
            id: entity,
            state_id: PlayerAnimationState::Running,
        });
    }

    if movement.x > 0. && !animation_controller.is_facing_right() {
        animation_controller.set_facing_right(true);
    } else if movement.x < 0. && animation_controller.is_facing_right() {
        animation_controller.set_facing_right(false);
    }
}

#[derive(Resource)]
pub struct InvincibilityTimer(Timer);

pub fn hit_immunity(
    mut q_player: Query<(Entity, &mut Health), With<Player>>,
    mut timer: ResMut<InvincibilityTimer>,
    mut ev_dmg: EventReader<TookDamageEvent>,
    time: Res<Time>,
    audio_assets: Res<AudioAssets>,
    fx_channel: Res<FXChannel>,
    mut pitch_rng: ResMut<PitchRNG>,
) {
    timer.0.tick(time.delta());
    let (player_entity, mut player_health) = q_player.single_mut();

    if timer.0.just_finished() {
        player_health.is_invincible = false;
    }

    for took_dmg in ev_dmg.iter() {
        if took_dmg.entity == player_entity {
            player_health.is_invincible = true;
            timer.0.reset();

            fx_channel
                .play(audio_assets.grunt.clone())
                .with_playback_rate(pitch_rng.0 .0.gen_range(0.9..1.1));
        }
    }
}

pub fn enemy_collision(
    mut q_player: Query<(Entity, &mut Health), With<Player>>,
    q_enemies: Query<Entity, With<Enemy>>,
    mut collisions: EventReader<IsCollidingEvent>,
    mut ev_dmg: EventWriter<TookDamageEvent>,
) {
    let (player, mut health) = q_player.single_mut();
    let mut is_hit = false;

    for ev in collisions.iter() {
        if let Ok(_) = q_enemies.get(ev.collision.entity_a) {
            if player == ev.collision.entity_b {
                is_hit = true;
                break;
            }
        }
        if let Ok(_) = q_enemies.get(ev.collision.entity_b) {
            if player == ev.collision.entity_a {
                is_hit = true;
                break;
            }
        }
    }

    if is_hit {
        health.take_damage(player, &mut ev_dmg, 1);
    }
}
