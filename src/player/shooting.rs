use std::{f32::consts::PI, time::Duration};

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    collision::collider::Collider,
    combat::{
        projectile::{DamageTarget, PiercingMode, Projectile},
        teams::Team,
    },
    constants::{SortingLayers, SCALING_VEC3},
    loading::TextureAssets,
    util::radians::Radian, movement::{velocity::Velocity},
};

use super::{reload_ui::ReloadTimer, Player};

#[derive(Resource)]
pub struct ShootingCooldown(pub Timer);

pub fn shoot(
    buttons: Res<Input<MouseButton>>,
    mut q_player: Query<(&mut Player, &Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut shooting_cooldown: ResMut<ShootingCooldown>,
    mut reload_timer: ResMut<ReloadTimer>,
    textures: Res<TextureAssets>,
    time: Res<Time>,
    mut commands: Commands,
) {
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
            shooting_cooldown.0.set_duration(Duration::from_secs_f32(player.shoot_time));
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
                    .set_duration(Duration::from_secs_f32(player.reload_time));
                reload_timer.0.reset()
            }

            // Shoot!
            commands
                .spawn(SpriteBundle {
                    texture: textures.texture_bullet_small.clone(),
                    transform: Transform {
                        translation: bullet_translation,
                        scale: SCALING_VEC3,
                        rotation: Quat::IDENTITY,
                    },
                    ..Default::default()
                })
                .insert(Projectile {
                    dmg: 5,
                    damage_target: DamageTarget::Team(Team::Enemy),
                    piercing_mode: PiercingMode::None,
                    entities_hit: 0,
                    is_alive: true,
                })
                .insert(Velocity {
                    vec: direction_vec * 600.,
                })
                .insert(Collider::new_circle(5., bullet_translation.truncate()));
        }
    }
}
