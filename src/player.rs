use std::time::Duration;

use crate::actions::Actions;
use crate::animation::{AnimationState, animate, AnimationController, AnimationTimer};
use crate::collision::collider::Collider;
use crate::constants::SCALING_VEC3;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::{prelude::*, animation};


#[derive(Resource)]
pub struct PlayerAnimationStates {
    pub idle : AnimationState,
    pub running : AnimationState,
}

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_systems(Update, animate.run_if(in_state(GameState::Playing)))
            .insert_resource(PlayerAnimationStates{
                idle: AnimationState { start_index: 0, frames: 1},
                running: AnimationState { start_index: 1, frames: 4},
            });
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>, mut texture_atlases: ResMut<Assets<TextureAtlas>>, animation_states : Res<PlayerAnimationStates>) {
    let texture_atlas = 
        TextureAtlas::from_grid(textures.texture_hatman.clone(), Vec2 { x: 32., y: 32. }, 5, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
    .spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite::new(animation_states.idle.start_index),
        transform: Transform { translation: Vec3::new(0., 0., 1.), rotation: Quat::IDENTITY, scale: SCALING_VEC3 },
        ..Default::default()
    })
    .insert(Player)
    .insert(Collider::new_circle(50., Vec2 { x: 0., y: 0.}))
    .insert(AnimationTimer(Timer::from_seconds(1. / 10., TimerMode::Repeating)))
    .insert(AnimationController {
        state: animation_states.idle,
        is_facing_right: true,
    });
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    animation_states : Res<PlayerAnimationStates>,
    mut player_query: Query<(&mut Transform, &mut AnimationController, &mut TextureAtlasSprite)>,
) {
    
    let (mut player_transform, mut animation_controller, mut atlas) = player_query.single_mut();
    

    if actions.player_movement.is_none() {
        animation_controller.state = animation_states.idle;
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );

    player_transform.translation += movement;

    if animation_controller.state != animation_states.running {
        animation_controller.state = animation_states.running;
    }

    if movement.x > 0. && !animation_controller.is_facing_right {
        animation_controller.is_facing_right = true;
    } else if movement.x < 0. && animation_controller.is_facing_right {
        animation_controller.is_facing_right = false;
    }
}
