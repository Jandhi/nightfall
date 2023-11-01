use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    animation::{
        controller::AnimationController,
        info::{AnimationInfoBuilder, AnimationStateInfo},
        make_animation_bundle, Animation, AnimationStateChangeEvent, AnimationStateStorage,
    },
    constants::SortingLayers,
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
    Right,
}

const BUBBLE_FRAME_DURATION: f32 = 0.1;
impl Animation<XPBarAnimation> for XPBarAnimation {
    fn get_states() -> Vec<AnimationStateInfo<XPBarAnimation>> {
        let mut builder = AnimationInfoBuilder::new();

        for pos in [
            XPBarPosition::Left,
            XPBarPosition::Center,
            XPBarPosition::Right,
        ] {
            builder
                .add_single(XPBarAnimation::Empty(pos))
                .add_frames(
                    XPBarAnimation::Half(pos),
                    4,
                    Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
                )
                .add_frames(
                    XPBarAnimation::Filled(pos),
                    4,
                    Duration::from_secs_f32(BUBBLE_FRAME_DURATION),
                );
        }

        builder.build()
    }
}

pub type XPBarAnimations = AnimationStateStorage<XPBarAnimation>;

pub fn manage_xp_bar_sprites(
    q_player: Query<&Experience, Without<XPBarSprite>>,
    mut q_bullets: Query<
        (
            Entity,
            &AnimationController<XPBarAnimation>,
            &XPBarSprite,
            &mut Transform,
        ),
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
            _ if (experience.curr_experience as f32 / experience.threshold as f32) * 10.0
                > (1 + xp_bar_slice.index) as f32 =>
            {
                XPBarAnimation::Filled(position)
            }
            _ if (experience.curr_experience as f32 / experience.threshold as f32) * 10.0
                > xp_bar_slice.index as f32 =>
            {
                XPBarAnimation::Half(position)
            }
            _ => XPBarAnimation::Empty(position),
        };

        if controller.get_state() != desired_state {
            animation_state_change.send(AnimationStateChangeEvent {
                id: entity,
                state_id: desired_state,
            });
        }

        transform.translation = Vec3 {
            x: 0. + 32. * xp_bar_slice.index as f32,
            y: window.height() / 2. - 30.,
            z: SortingLayers::UI.into(),
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
        textures.xp_bar.clone(),
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
            .insert(make_animation_bundle(
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
