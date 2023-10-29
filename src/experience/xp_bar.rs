use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};


use crate::{
    animation::{
        animation_bundle, AnimationStateChangeEvent,
        AnimationStateStorage, Animation, controller::AnimationController, info::AnimationStateInfo,
    },
    loading::TextureAssets,
};

use super::experience::Experience;


#[derive(Component)]
pub struct XPBarSprite {
    index: u32,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum XPBarAnimation {
    Empty(XPBarPosition),
    Half(XPBarPosition),
    Filled(XPBarPosition),
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum XPBarPosition {
    Left,
    Center,
    Right
}

const BUBBLE_FRAME_DURATION : f32 = 0.1;
impl Animation<XPBarAnimation> for XPBarAnimation {
    fn get_states() -> Vec<AnimationStateInfo<XPBarAnimation>> {
        vec![
            AnimationStateInfo {
                id: XPBarAnimation::Empty(XPBarPosition::Left),
                start_index: 0,
                frames: 1,
                frame_duration: Duration::from_secs_f32(1.),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Half(XPBarPosition::Left),
                start_index: 1,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Filled(XPBarPosition::Left),
                start_index: 5,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Empty(XPBarPosition::Center),
                start_index: 9,
                frames: 1,
                frame_duration: Duration::from_secs_f32(1.),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Half(XPBarPosition::Center),
                start_index: 10,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Filled(XPBarPosition::Center),
                start_index: 14,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Empty(XPBarPosition::Right),
                start_index: 18,
                frames: 1,
                frame_duration: Duration::from_secs_f32(1.),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Half(XPBarPosition::Right),
                start_index: 19,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
            AnimationStateInfo {
                id: XPBarAnimation::Filled(XPBarPosition::Right),
                start_index: 23,
                frames: 4,
                frame_duration: Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
            },
        ]
    }
}

pub type XPBarAnimations = AnimationStateStorage<XPBarAnimation>;

pub fn manage_bullet_ui_sprites(
    q_player: Query<&Experience, Without<XPBarSprite>>,
    mut q_bullets: Query<
        (Entity, &AnimationController<XPBarAnimation>, &XPBarSprite, &mut Transform),
        Without<Experience>,
    >,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut animation_state_change: EventWriter<AnimationStateChangeEvent<XPBarAnimation>>,
) {
    let experience = q_player.single();
    let window = q_windows.single();

    for (entity, controller, xp_bar_slice, mut transform) in q_bullets.iter_mut() {
        let position = match xp_bar_slice.index {
            0 => XPBarPosition::Left,
            9 => XPBarPosition::Right,
            _ => XPBarPosition::Center,
        };
        let desired_state = match () {
            _ if experience.curr_experience as f32 / experience.xp_threshold as f32 > (1 + xp_bar_slice.index) as f32 => XPBarAnimation::Filled(position),
            _ if experience.curr_experience as f32 / experience.xp_threshold as f32 > xp_bar_slice.index as f32 => XPBarAnimation::Half(position),
            _ => XPBarAnimation::Empty(position),
        };

        if controller.get_state() != desired_state {
            animation_state_change.send(AnimationStateChangeEvent { 
                id: entity, 
                state_id: desired_state 
            });
        }

        transform.translation = Vec3 {
            x: 0. + 32. * xp_bar_slice.index as f32,
            y: window.height() / 2. - 30. ,
            z: 5.,
        }
    }
}

pub fn spawn_xp_bar(
    animations: Res<XPBarAnimations>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut commands: Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        textures.texture_xp_bar.clone(),
        Vec2 { x: 16., y: 16. },
        27,
        1,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    for i in 0..10 {
        commands
            .spawn(XPBarSprite { index: i })
            .insert(animation_bundle(
                match i {
                    0 => XPBarAnimation::Empty(XPBarPosition::Left),
                    9 => XPBarAnimation::Empty(XPBarPosition::Right),
                    _ => XPBarAnimation::Empty(XPBarPosition::Center),
                },
                &animations,
                texture_atlas_handle.clone(),
                default(),
            ));
    }
}
