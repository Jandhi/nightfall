use std::fmt::Display;
use std::time::Duration;

use crate::actions::Actions;
use crate::animation::{AnimationStateInfo, update_animation_frames, AnimationController, AnimationTimer, AnimationStateStorage, AnimationStateChangeEvent, AppAnimationSetup, make_animation_bundle};
use crate::collision::collider::Collider;
use crate::constants::SCALING_VEC3;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::utils::HashMap;
use bevy::{prelude::*, animation};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum PlayerAnimationState {
    IDLE,
    RUNNING
}

impl Display for PlayerAnimationState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            PlayerAnimationState::IDLE => "idle",
            PlayerAnimationState::RUNNING => "running",
        })
    }
}

type PlayerAnimations = AnimationStateStorage<PlayerAnimationState>;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(GameState::Playing)))
            .add_animation(vec![
                AnimationStateInfo { 
                    id: PlayerAnimationState::IDLE, 
                    start_index: 0, 
                    frames: 2,
                    frame_duration: Duration::from_secs_f32(1. / 2.),
                },
                AnimationStateInfo { 
                    id: PlayerAnimationState::RUNNING, 
                    start_index: 2, 
                    frames: 4,
                    frame_duration: Duration::from_secs_f32(1. / 10.),
                }

            ]);
    }
}

fn spawn_player(
    player_animations : Res<PlayerAnimations>,
    textures: Res<TextureAssets>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>>, 
    mut commands: Commands, 
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_hatman.clone(),
         Vec2 { x: 32., y: 32. },
          6,
           1,
            None,
             None
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
    .spawn(Player)
    .insert(Collider::new_circle(50., Vec2 { x: 0., y: 0.}))
    .insert(make_animation_bundle(
        PlayerAnimationState::IDLE, 
        player_animations, 
        texture_atlas_handle
    ));
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    animation_states : Res<PlayerAnimations>,
    mut animation_change : EventWriter<AnimationStateChangeEvent<PlayerAnimationState>>,
    mut player_query: Query<(Entity, &mut Transform, &mut AnimationController<PlayerAnimationState>, &mut TextureAtlasSprite)>,
) {
    
    let (entity, mut player_transform, mut animation_controller, mut atlas) = player_query.single_mut();
    

    if actions.player_movement.is_none() {
        if animation_controller.get_state() != PlayerAnimationState::IDLE {
            animation_change.send(AnimationStateChangeEvent{
                id : entity,
                state_id : PlayerAnimationState::IDLE
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

    if animation_controller.get_state() != PlayerAnimationState::RUNNING {
        animation_change.send(AnimationStateChangeEvent{
            id : entity,
            state_id : PlayerAnimationState::RUNNING
        });
    }

    if movement.x > 0. && !animation_controller.is_facing_right() {
        animation_controller.set_facing_right(true);
    } else if movement.x < 0. && animation_controller.is_facing_right() {
        animation_controller.set_facing_right(false);
    }
}
