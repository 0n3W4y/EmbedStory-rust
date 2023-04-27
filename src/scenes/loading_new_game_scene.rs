use bevy::prelude::*;

use super::game_scenes::tilemap;
use crate::config::{RESOLUTION, TILE_SIZE, WINDOW_HEIGHT};
use crate::materials::{font::FontMaterials, material_manager::MaterialManager};
use crate::resources::charactor_manager::CharactorManager;
use crate::resources::deploy::Deploy;
use crate::resources::deploy::game_scene_biome_deploy::BiomeType;
use crate::resources::dictionary::Dictionary;
use crate::resources::stuff_manager::StuffManager;
use crate::resources::thing_manager::ThingManager;
use crate::resources::scene_manager::{SceneManager, SceneType};
use crate::scenes::SceneState;

use super::game_scenes::game_scene::GameScene;

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

pub struct LoadingNewGameSceneData {
    user_interface_root: Entity,
}

pub struct LoadingNewGameScenePlugin;

impl Plugin for LoadingNewGameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(SceneState::LoadingNewGameScene)
                .with_system(setup)
                .with_system(create_starting_scenes),
        );
        app.add_system_set(
            SystemSet::on_update(SceneState::LoadingNewGameScene).with_system(update),
        );
        app.add_system_set(
            SystemSet::on_exit(SceneState::LoadingNewGameScene).with_system(cleanup),
        );
    }
}

fn setup(
    mut commands: Commands,
    material_manager: Res<MaterialManager>,
    font: Res<FontMaterials>,
    dictionary: Res<Dictionary>,
) {
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(
                material_manager
                    .loading_new_game_scene
                    .background_image
                    .clone(),
            ),
            ..Default::default()
        })
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
    root.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(
                Val::Px(LOADING_BORDER_WIDTH),
                Val::Px(LOADING_BORDER_HEIGHT),
            ),
            //position in center
            position: Rect {
                top: Val::Px((WINDOW_HEIGHT / 2.0) - (LOADING_BORDER_HEIGHT / 2.0)),
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION) / 2.0 - (LOADING_BORDER_WIDTH / 2.0)),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        color: UiColor(Color::DARK_GRAY),
        ..Default::default()
    })
    .with_children(|parent| {
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    size: Size::new(
                        Val::Px(0.0),
                        Val::Px(LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2),
                    ),
                    position: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                color: UiColor(INNER_LOADER_COLOR),
                ..Default::default()
            })
            .with_children(|parent| {
                let font_str = font.get_font(dictionary.get_current_language());

                parent.spawn_bundle(TextBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: font_str,
                            font_size: TEXT_FONT_SIZE,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            vertical: VerticalAlign::Center,
                            horizontal: HorizontalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(LoadingNewGameSceneComponent {
                max_width: LOADING_BORDER_WIDTH - 10.0,
                current_width: 0.0,
            });
    });
}

fn loading_text(root: &mut ChildBuilder, font: &Res<FontMaterials>, dictionary: &Res<Dictionary>) {
    root.spawn_bundle(NodeBundle {
        style: Style {
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            size: Size::new(Val::Px(LOADING_BORDER_WIDTH), Val::Px(35.0)),
            position: Rect {
                left: Val::Px((WINDOW_HEIGHT * RESOLUTION - LOADING_BORDER_WIDTH) / 2.0),
                top: Val::Px((WINDOW_HEIGHT - LOADING_BORDER_HEIGHT) / 2.0 - 37.0),
                bottom: Val::Auto,
                right: Val::Auto,
            },
            ..Default::default()
        },
        color: UiColor(Color::NONE),
        ..Default::default()
    })
    .with_children(|parent| {
        let glossary = dictionary.get_glossary();
        let font_str = font.get_font(dictionary.get_current_language());

        parent.spawn_bundle(TextBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text::with_section(
                glossary.loading_scene_text.loading,
                TextStyle {
                    font: font_str,
                    font_size: LOADING_TEXT_FONT_SIZE,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        });
    });
}

fn update(
    mut query: Query<(&mut LoadingNewGameSceneComponent, &mut Style, &Children)>,
    mut state: ResMut<State<SceneState>>,
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
                .set(SceneState::GameScene)
                .expect("Couldn't switch state to Game Ground Scene");
        }
    }
}

fn cleanup(mut commands: Commands, scene_data: Res<LoadingNewGameSceneData>) {
    commands
        .entity(scene_data.user_interface_root)
        .despawn_recursive();
}

fn create_starting_scenes(mut commands: Commands, deploy: Res<Deploy>) {
    //get scene settings fro deploy;
    let scene_setting = deploy.game_scene.get_scene_setting(BiomeType::Plain);

    // Create new scene_manager;
    let mut scene_manager: SceneManager = Default::default();

    //create new object manager;
    let mut object_manager: ThingManager = Default::default();

    //create new charactor manager;
    
    let mut charactor_manager: CharactorManager = Default::default();

    //create new stuff manager;

    let mut stuff_manager: StuffManager = Default::default();

    //Create starting scene;
    let mut starting_scene: GameScene = scene_manager.create_game_scene(&SceneType::GroundScene);
    let id = starting_scene.scene_id;

    //config scen etilemap with deploy ;
    starting_scene
        .tilemap
        .set(TILE_SIZE, scene_setting.width, scene_setting.height);

    //generate tilemap with template from biome type;
    tilemap::generate::generate_tilemap(
        &mut starting_scene.tilemap,
        &deploy,
        &scene_setting.biome_type,
    );

    //prepare things for scene;
    let biome_setting = deploy
        .game_scene_biome
        .get_biome_setting(&scene_setting.biome_type);
    //let things_for_scene = scene_setting.objects.things;
    object_manager.generate_things_for_scene(
        &mut starting_scene,
        &deploy,
        &biome_setting.objects.things,
    );
    //object_manager.generate_pattern_things_for_scene( &mut starting_scene );

    //store scene into scene_manager;
    scene_manager.store_game_scene(starting_scene);

    //set next scene to load - new scene;
    scene_manager.set_current_game_scene(id);

    commands.insert_resource(scene_manager);
    commands.insert_resource(object_manager);
    commands.insert_resource(stuff_manager);
    commands.insert_resource(charactor_manager);
}
