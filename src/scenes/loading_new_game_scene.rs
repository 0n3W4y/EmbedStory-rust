use bevy::prelude::*;

use crate::scenes::SceneState;
use crate::materials::{material_manager::MaterialManager, font::FontMaterials};
use crate::resources::dictionary::Dictionary;
use crate::config::{ RESOLUTION, WINDOW_HEIGHT };

const LOADING_BORDER_WIDTH: f32 = 600.0;
const LOADING_BORDER_HEIGHT: f32 = 60.0;
const TEXT_FONT_SIZE: f32 = 32.0;
const LOADING_TEXT_FONT_SIZE: f32 = 40.0;
const INNER_LOADER_COLOR: Color = Color::rgb(247.0 / 255.0, 104.0 / 255.0, 12.0 / 255.0);


#[derive( Component )]
pub struct LoadingNewGameSceneComponent{
    max_width: f32,
    current_width: f32,
}

pub struct LoadingNewGameSceneData{
    user_interface_root: Entity,
}

pub struct LoadingNewGameScenePlugin;

impl Plugin for LoadingNewGameScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::LoadingNewGameScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::LoadingNewGameScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::LoadingNewGameScene ).with_system( cleanup ));
    }
}

fn setup(
    mut commands: Commands,
    material_manager: Res<MaterialManager>,
    font: Res<FontMaterials>,
    dictionary: Res<Dictionary>
){
    let user_interface_root = commands
        .spawn_bundle( NodeBundle{
            style: Style{
                size: Size::new( Val::Percent( 100.0 ), Val::Percent( 100.0)),
                ..Default::default()
            },
            image: UiImage( material_manager.loading_new_game_scene_material.background_image.clone() ),
            ..Default::default()
        })
        .with_children(|parent|{
            loading_text( parent, &font, &dictionary );
            loader_bundle( parent, &font, &dictionary );
        })
        .id();

    commands.insert_resource( LoadingNewGameSceneData{ user_interface_root });
}

fn loader_bundle( 
    root: &mut ChildBuilder, 
    font: &Res<FontMaterials>, 
    dictionary: &Res<Dictionary>,
){
    root.spawn_bundle( 
        NodeBundle{
            style: Style{
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::new(
                    Val::Px( LOADING_BORDER_WIDTH ),
                    Val::Px( LOADING_BORDER_HEIGHT ),
                ),
                //position in center
                position: Rect {
                    top: Val::Px(( WINDOW_HEIGHT / 2.0 ) - ( LOADING_BORDER_HEIGHT / 2.0 )),
                    left: Val::Px(( WINDOW_HEIGHT * RESOLUTION ) / 2.0 - ( LOADING_BORDER_WIDTH / 2.0 )),
                    bottom: Val::Auto,
                    right: Val::Auto,
                },
                ..Default::default()
            },
            color: UiColor( Color::DARK_GRAY ),
            ..Default::default()
        },
    )
    .with_children(|parent|{
        parent.spawn_bundle( NodeBundle{
            style: Style {
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::new(
                    Val::Px( 0.0 ),
                    Val::Px( LOADING_BORDER_HEIGHT - LOADING_BORDER_HEIGHT * 0.2 ),
                ),
                position: Rect::all(Val::Px(5.0)),
                ..Default::default()
            },
            color: UiColor( INNER_LOADER_COLOR ),
                ..Default::default()
        })
        .with_children( |parent|{
            let font_str = font.get_font( dictionary.get_current_language() );

            parent.spawn_bundle( TextBundle{
                style: Style{
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::Center,
                    ..Default::default()
                },
                text: Text::with_section(
                    "",
                    TextStyle {
                        font:  font_str,
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
        .insert( LoadingNewGameSceneComponent{
            max_width: LOADING_BORDER_WIDTH - 10.0,
            current_width: 0.0,
        });
    });
}

fn loading_text(
    root: &mut ChildBuilder,
    font: &Res<FontMaterials>,
    dictionary: &Res<Dictionary>,
){
    root.spawn_bundle( NodeBundle{ 
        style: Style{
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
    .with_children( |parent| {
        let glossary = dictionary.get_glossary();
        let font_str = font.get_font( dictionary.get_current_language() );

        parent.spawn_bundle( TextBundle{
            style: Style{
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center,
                ..Default::default()
            },
            text: Text::with_section(
                glossary.loading_scene_text.loading,
                TextStyle{
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
    mut query: Query<( &mut LoadingNewGameSceneComponent, &mut Style, &Children )>,
    mut state: ResMut<State<SceneState>>,
    mut text_query: Query< &mut Text>,
){}

fn cleanup(
    mut commands: Commands,
    scene_data: Res<LoadingNewGameSceneData>,
){
    commands.entity( scene_data.user_interface_root ).despawn_recursive();
}