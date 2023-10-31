


use crate::actions::Actions;
use crate::animation::controller::AnimationController;
use crate::animation::{
    make_animation_bundle, AnimationStateChangeEvent, AppAnimationSetup,
};
use crate::collision::collider::Collider;
use crate::combat::health::{Health, HealthType};
use crate::combat::teams::{TeamMember, Team};
use crate::constants::SortingLayers;
use crate::experience::experience::Experience;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::movement::edge_teleport::EdgeTeleports;
use crate::movement::pause::ActionPauseState;
use bevy::prelude::*;
use bevy::utils::HashSet;

use self::ability::Ability;
use self::animations::{PlayerAnimationState, PlayerAnimations};
use self::bullets_ui::{manage_bullet_ui_sprites, BulletUIAnimation, BulletUICount};
use self::health_ui::{manage_health_ui_sprites, HealthUICount, HealthUIAnimationState};
use self::reload_ui::{spawn_reload_ui, update_reload_ui, ReloadTimer};
use self::shooting::{shoot, ShootingCooldown};

mod animations;
mod bullets_ui;
mod reload_ui;
mod shooting;
mod health_ui;
pub mod ability;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    curr_bullets: u32,
    max_bullets: u32,
    is_reloading: bool,
    pub abilities : Vec<Ability>,
}

impl Player {
    pub fn damage(&self) -> HealthType {
        self.abilities.iter().fold(5., |dmg, ability| dmg * ability.damage_mult()) as u32
    }
    
    pub fn shoot_time(&self) -> f32 {
        self.abilities.iter().fold(0.5, |dmg, ability| dmg * ability.shoot_speed_mult())
    }
     
    pub fn reload_time(&self) -> f32 {
        self.abilities.iter().fold(1.0, |dmg, ability| dmg * ability.reload_mult())
    }

    pub fn knockback(&self) -> f32 {
        self.abilities.iter().fold(20., |dmg, ability| dmg * ability.knockback_mult())
    }


}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), (
                spawn_player,
                spawn_reload_ui
            ))
            .add_systems(Update, (
                move_player,
                shoot,
                manage_bullet_ui_sprites,
                manage_health_ui_sprites,
                update_reload_ui,
            ).run_if(in_state(GameState::Playing)))
            .insert_resource(ReloadTimer(Timer::from_seconds(0., TimerMode::Once)))
            .insert_resource(BulletUICount(0))
            .insert_resource(HealthUICount(0))
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
        )).insert(Experience{
            curr_experience: 0,
            level: 0,
            threshold: 20,
            pick_distance: 10.0,
        }).insert(EdgeTeleports)
        .insert(Health{
            value: 3,
            max: 3,
        }).insert(TeamMember{team: Team::Player});
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
    pause : Res<ActionPauseState>,
) {
    if pause.is_paused {
        return;
    }

    let (entity, mut player_transform, mut animation_controller, _) =
        player_query.single_mut();

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
