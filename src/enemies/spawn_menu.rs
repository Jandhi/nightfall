use bevy::{prelude::*, sprite::Anchor};

use crate::{ui::{grid::{GridBundle, Grid}, button::{ButtonBundle, Button}, alignment::AlignedBundle, clickable::ClickedEvent}, loading::{TextureAssets, AbilityTextures, FontAssets}, collision::collider::Collider, palette::Palette, constants::SortingLayers, util::with_z::WithZ};

use super::{enemy::EnemyType, spawning::EnemySpawnEvent};


pub struct SpawnMenuPlugin;

impl Plugin for SpawnMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<SpawnMenuState>()
            .add_systems(OnEnter(SpawnMenuState::On), initialize_menu)
            .add_systems(OnExit(SpawnMenuState::On), destroy_menu)
            .add_systems(Update, spawn_button_update.run_if(in_state(SpawnMenuState::On)));
    }
}

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum SpawnMenuState {
    #[default]
    Off,
    On,
}



#[derive(Component)]
struct SpawnMenuComponent;

#[derive(Component)]
struct SpawnMenuButton(EnemyType);

fn initialize_menu(
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    textures : Res<TextureAssets>,
    fonts : Res<FontAssets>,
    palette : Res<Palette>,
    ability_textures : Res<AbilityTextures>,
    mut commands : Commands
) {
    let enemy_types = EnemyType::all();

    let frame_atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
        ability_textures.frame.clone(),
        Vec2 { x: 32., y: 32. },
        2,
        1,
        None,
        None,
    ));

    commands.spawn(
        GridBundle {
            grid: Grid {
                grid_size: IVec2 {
                    x: (enemy_types.len() + 1) as i32 / 2,
                    y: 2,
                },
            },
            ..Default::default()
        }
    ).with_children( |parent| {
        for enemy_type in enemy_types.iter() {
            parent.spawn(ButtonBundle{
                button: Button::new(false),
                sprite: SpriteSheetBundle { 
                    texture_atlas: frame_atlas_handle.clone(),
                    transform: Transform::from_scale(Vec3 { x: 2.0, y: 2.0, z: 1.0 }).with_z(SortingLayers::UI.into()),
                    ..Default::default()
                },
                collider: Collider::new_rect(Vec2 { x: 64.0, y: 64.0}),
                ..Default::default()
            }).insert(AlignedBundle::default()).with_children(|parent| {
                let atlas_handle = texture_atlases.add(TextureAtlas::from_grid(
                    enemy_type.get_texture(&textures).clone(),
                    enemy_type.sprite_size(),
                    1,
                    1,
                    None,
                    None,
                ));

                parent.spawn(SpriteSheetBundle{
                    transform: Transform::from_scale(Vec3 { x: 32.0 / enemy_type.sprite_size().x, y: 32.0 / enemy_type.sprite_size().y, z: SortingLayers::UI.into() }),
                    texture_atlas: atlas_handle,
                    ..Default::default()
                });

                parent.spawn(Text2dBundle{
                    text: Text::from_section(enemy_type.name(), TextStyle {
                         font: fonts.gothic.clone(), 
                         font_size: 10.0, 
                         color: palette.orange,
                    }),
                    text_anchor: Anchor::Center,
                    transform: Transform::from_translation(Vec3 { x: 0., y: -50., z: SortingLayers::UI.into() }),
                    ..Default::default()
                });
            }).insert(SpawnMenuButton(*enemy_type));
        }   
    }).insert(SpawnMenuComponent);
}

fn destroy_menu (
    q_components : Query<Entity, With<SpawnMenuComponent>>,
    mut commands : Commands,
) {
    for entity in q_components.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn spawn_button_update(
    q_buttons : Query<&SpawnMenuButton>,
    mut clicked_ev : EventReader<ClickedEvent>,
    mut spawn_ev : EventWriter<EnemySpawnEvent>,
    mut next_state : ResMut<NextState<SpawnMenuState>>,
) {
    for clicked in clicked_ev.iter() {
        if let Ok(button) = q_buttons.get(clicked.entity) {
            spawn_ev.send(EnemySpawnEvent { enemy_type: button.0, position: default() });
            next_state.set(SpawnMenuState::Off);
        }
    }
}