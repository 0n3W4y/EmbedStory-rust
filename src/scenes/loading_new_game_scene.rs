use bevy::prelude::*;

use crate::config::{RESOLUTION, WINDOW_HEIGHT};
use crate::materials::{font::FontMaterials, material_manager::MaterialManager};
use crate::resources::dictionary::Dictionary;
use crate::resources::profile::Profile;
use crate::resources::scene_manager::SceneManager;
use crate::scenes::AppState;

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;
const TEXT_FONT_SIZE: f32 = 32.0;
const LOADING_TEXT_FONT_SIZE: f32 = 40.0;
const INNER_LOADER_COLOR: Color = Color::rgb(247.0 / 255.0, 104.0 / 255.0, 12.0 / 255.0);

#[derive(Component)]
pub struct LoadingNewGameSceneComponent {
    max_width: f32,
    current_width: f32,
}

#[derive(Resource)]
pub struct LoadingNewGameSceneData {
    user_interface_root: Entity,
}

pub struct LoadingNewGameScenePlugin;

impl Plugin for LoadingNewGameScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (
                    setup,
                    prepare_next_scene
                )
                .in_schedule(OnEnter(AppState::LoadingNewGameScene))
            )
            .add_system(update.in_set(OnUpdate(AppState::LoadingNewGameScene)))
            .add_system(cleanup.in_schedule(OnExit(AppState::LoadingNewGameScene)));
    }
}

fn setup(
    mut commands: Commands,
    material_manager: Res<MaterialManager>,
    font: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let user_interface_root = commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        UiImage::new(material_manager.loading_new_game_scene.background_image.clone()),
        ))
        .with_children(|parent| {
            loading_text(parent, &font, &dictionary);
            loader_bundle(parent, &font, &dictionary);
        })
        .id();

    commands.insert_resource(LoadingNewGameSceneData {
        user_interface_root,
    });
}

fn loader_bundle(root: &mut ChildBuilder, font: &Res<FontMaterials>, dictionary: &Res<Dictionary>) {
    root.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(LOADING_BORDER_WIDTH),
                Val::Px(LOADING_BORDER_HEIGHT),
            ),
            //position in center
            position: UiRect {
                top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0)),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        background_color: BackgroundColor(Color::DARK_GRAY),
        ..Default::default()
    })
    .with_children(|parent| {
        parent
            .spawn((NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    size: Size::new(
                        Val::Px(0.0),
                        Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    ),
                    position: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                background_color: BackgroundColor(INNER_LOADER_COLOR),
                ..Default::default()
            },
            LoadingNewGameSceneComponent {
                max_width: LOADING_BORDER_WIDTH - 10.0,
                current_width: 0.0,
            }
            ))
            .with_children(|parent| {
                let font_str = font.get_font(dictionary.get_current_language());

                parent.spawn(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::from_section(
                        "",
                        TextStyle {
                            font: font_str,
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        }
                    ),
                    ..Default::default()
                }
                .with_text_alignment(TextAlignment::Center)
                );
            });
    });
}

fn loading_text(root: &mut ChildBuilder, font: &Res<FontMaterials>, dictionary: &Res<Dictionary>) {
    root.spawn(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(LOADING_BORDER_WIDTH), Val::Px(35.0)),
            position: UiRect {
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
                top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        background_color: BackgroundColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossary = dictionary.get_glossary();
        let font_str = font.get_font(dictionary.get_current_language());

        parent.spawn(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text::from_section(
                glossary.loading_scene_text.loading,
                TextStyle {
                    font: font_str,
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                }
            ),
            ..Default::default()
        }
        .with_text_alignment(TextAlignment::Center)
        );
    });
}

fn update(
    mut query: Query<(&mut LoadingNewGameSceneComponent, &mut Style, &Children)>,
    mut state: ResMut<NextState<AppState>>,
    mut text_query: Query<&mut Text>,
) {
    for (mut loading_component, mut style, children) in query.iter_mut() {
        if loading_component.max_width > loading_component.current_width {
            loading_component.current_width += 2.5;
            style.size.width = Val::Px(loading_component.current_width);

            let value =
                (loading_component.current_width / loading_component.max_width * 100.0) as usize;
            if value >= 5 {
                let mut text = text_query.get_mut(children[0]).unwrap();
                text.sections[0].value = value.to_string() + "%";
            }
        } else {
            state
                .set(AppState::GameScene);
        }
    }
}

fn cleanup(mut commands: Commands, scene_data: Res<LoadingNewGameSceneData>) {
    commands
        .entity(scene_data.user_interface_root)
        .despawn_recursive();
}

fn prepare_next_scene(
    profile: Res<Profile>,
    mut scene_manager: ResMut<SceneManager>,
) {
    let next_scene_id = scene_manager.get_next_scene().scene_id; 
    scene_manager.set_current_game_scene(next_scene_id);
    scene_manager.next_game_scene = None;
    let current_scene = scene_manager.get_current_game_scene_mut();

    match &profile.charactor {
        Some(v) => current_scene.charactors.store(v.clone()),
        None => panic!("Player not created"),
    };

    match &profile.companion {
        Some(v) => current_scene.charactors.store(v.clone()),
        None => {}
    };
}
