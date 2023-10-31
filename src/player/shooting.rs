use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_debug_text_overlay::screen_print;
use bevy_kira_audio::AudioControl;

use crate::{
    collision::collider::Collider,
    combat::{
        projectile::{DamageTarget, PiercingMode, Projectile},
        teams::Team, health::HealthType, knockback::Knockback,
    },
    constants::{SortingLayers, SCALING_VEC3},
    loading::{TextureAssets, AudioAssets},
    util::radians::Radian, movement::{velocity::Velocity, pause::ActionPauseState}, audio::{FXChannel, self},
};

use super::{reload_ui::ReloadTimer, Player, ability::Ability};

#[derive(Resource)]
pub struct ShootingCooldown(pub Timer);

pub fn shoot(
    buttons: Res<Input<MouseButton>>,
    mut q_player: Query<(&mut Player, &Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut shooting_cooldown: ResMut<ShootingCooldown>,
    mut reload_timer: ResMut<ReloadTimer>,
    textures: Res<TextureAssets>,
    audio_assets : Res<AudioAssets>,
    fx_channel : Res<FXChannel>,
    time: Res<Time>,
    pause : Res<ActionPauseState>,
    mut commands: Commands,
) {
    if pause.is_paused {
        return;
    }

    shooting_cooldown.0.tick(time.delta());
    reload_timer.0.tick(time.delta());

    let window = q_windows.single();
    let (mut player, transform) = q_player.single_mut();

    if reload_timer.0.just_finished() {
        player.curr_bullets = player.max_bullets;
        player.is_reloading = false;
    }

    if shooting_cooldown.0.finished() && !player.is_reloading && buttons.pressed(MouseButton::Left)
    {
        if let Some(cursor_position) = window.cursor_position() {
            // Reset cooldown
            shooting_cooldown.0.set_duration(Duration::from_secs_f32(player.shoot_time()));
            shooting_cooldown.0.reset();

            let target = Vec2::new(
                cursor_position.x - window.width() / 2.,
                window.height() / 2. - cursor_position.y,
            );

            let direction = target - transform.translation.truncate();
            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
            let direction_vec = angle_to_target.unit_vector();
            let bullet_translation = transform.translation
                + Vec3 {
                    x: direction_vec.x,
                    y: direction_vec.y,
                    z: SortingLayers::Action.into(),
                } * 10.
                + Vec3::Z * 5.;

            player.curr_bullets -= 1;
            if player.curr_bullets == 0 {
                player.is_reloading = true;
                reload_timer
                    .0
                    .set_duration(Duration::from_secs_f32(player.reload_time()));
                reload_timer.0.reset();

                fx_channel.play(audio_assets.reload.clone());
            }

            // Play audio
            fx_channel.play(audio_assets.gunshot.clone());

            let dmg = player.damage();
            let knockback = player.knockback();
            let velocity : f32 = 600.;

            if player.abilities.contains(&Ability::MegaShotgun) {
                let offset_angle = Radian::from_degrees(7.);
                let bullets = 7;

                for i in 0..bullets {
                    spawn_bullet(&player, &mut commands, bullet_translation, &textures, (angle_to_target + offset_angle * ((bullets - 1) as f32 / -2. + i as f32)).normalize().unit_vector(), velocity, dmg, knockback);
                }

            } else if player.abilities.contains(&Ability::Shotgun) {
                let offset_angle = Radian::from_degrees(7.);
                let bullets = 5;

                for i in 0..bullets {
                    spawn_bullet(&player, &mut commands, bullet_translation, &textures, (angle_to_target + offset_angle * ((bullets - 1) as f32 / -2. + i as f32)).normalize().unit_vector(), velocity, dmg, knockback);
                }

            } else if player.abilities.contains(&Ability::TripleBarrel) {
                let offset_angle = Radian::from_degrees(7.);
                
                spawn_bullet(&player, &mut commands, bullet_translation, &textures, (angle_to_target + offset_angle).normalize().unit_vector(), velocity, dmg, knockback);                
                spawn_bullet(&player, &mut commands, bullet_translation, &textures, (angle_to_target).unit_vector(), velocity, dmg, knockback);                
                spawn_bullet(&player, &mut commands, bullet_translation, &textures, (angle_to_target - offset_angle).normalize().unit_vector(), velocity, dmg, knockback);                

            } else if player.abilities.contains(&Ability::DoubleBarrel) {
                let perp_vec = Vec3{
                    x: direction_vec.perp().x,
                    y: direction_vec.perp().y,
                    z: 0.
                };

                spawn_bullet(&player, &mut commands, bullet_translation + (perp_vec * 5.), &textures, direction_vec, velocity, dmg, knockback);
                spawn_bullet(&player, &mut commands, bullet_translation - (perp_vec * 5.), &textures, direction_vec, velocity, dmg, knockback);
            } else {
                spawn_bullet(&player, &mut commands, bullet_translation, &textures, direction_vec, velocity, dmg, knockback);
            }
            
            // Shoot!
            
        }
    }
}

fn spawn_bullet(
    player : &Player,
    commands : &mut Commands,
    translation : Vec3,
    textures: &Res<TextureAssets>,
    direction_vec : Vec2,
    velocity : f32,
    damage : HealthType,
    knockback : f32,
) {
    commands
        .spawn(SpriteBundle {
            texture: match player.abilities.contains(&Ability::BigBullets) {
                true => textures.bullet_medium.clone(),
                false => textures.bullet_small.clone(),
            },
            transform: Transform {
                translation: translation,
                scale: SCALING_VEC3,
                rotation: Quat::IDENTITY,
            },
            ..Default::default()
        })
        .insert(Projectile {
            dmg: damage,
            damage_target: DamageTarget::Team(Team::Enemy),
            piercing_mode: PiercingMode::None,
            entities_hit: 0,
            is_alive: true,
        })
        .insert(Velocity {
            vec: direction_vec * velocity,
        })
        .insert(Collider::new_circle(5., translation.truncate()))
        .insert(Knockback{ force: knockback });
}