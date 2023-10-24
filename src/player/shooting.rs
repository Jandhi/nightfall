use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{util::radians::Radian, constants::SCALING_VEC3, loading::TextureAssets, combat::{projectile::{Projectile, DamageTarget, PiercingMode, StraightMovement}, teams::Team}, collision::collider::Collider};

use super::Player;

#[derive(Resource)]
pub struct ShootingCooldown(pub Timer);

pub fn shoot(
    buttons : Res<Input<MouseButton>>,
    q_player : Query<(&Player, &Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut shooting_cooldown : ResMut<ShootingCooldown>,
    textures: Res<TextureAssets>,
    time : Res<Time>,
    mut commands : Commands
) {
    shooting_cooldown.0.tick(time.delta());

    if shooting_cooldown.0.finished() && buttons.just_pressed(MouseButton::Left) {
        let window = q_windows.single();
        let (_, transform) = q_player.single();

        if let Some(cursor_position) = window.cursor_position() {

            // Reset cooldown
            shooting_cooldown.0.reset();
            
            
            let target = Vec2::new(
                cursor_position.x - window.width() / 2.,
                window.height() / 2. - cursor_position.y,
            );

            
            let direction = target - transform.translation.truncate();
            // obtain angle to target with respect to x-axis.
            let angle_to_target = Radian::from(direction.y.atan2(direction.x) - PI / 2.);
            let direction_vec = Vec2{
                x: -angle_to_target.angle.sin(),
                y: angle_to_target.angle.cos(),
            };
            let bullet_translation = transform.translation + Vec3{
                x: direction_vec.x,
                y: direction_vec.y,
                z: 0.
            } * 10. + Vec3::Z * 5.;
            
            // Shoot!
            commands.spawn(SpriteBundle{
                texture: textures.texture_bullet_small.clone(),
                transform: Transform {
                    translation: bullet_translation,
                    scale: SCALING_VEC3,
                    rotation: Quat::IDENTITY,
                },
                ..Default::default()
            })
                .insert(Projectile{
                    dmg: 1,
                    damage_target: DamageTarget::Team(Team::Enemy),
                    piercing_mode: PiercingMode::None,
                    entities_hit: 0,
                    is_alive: true,
                })
                .insert(StraightMovement{
                    angle: direction_vec,
                    velocity: 600.,
                })
                .insert(Collider::new_circle(
                    5., 
                    bullet_translation.truncate()
                ));
        }
        
    }
}