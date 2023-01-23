use bevy::prelude::*;

use crate::materials::font::FontMaterials;
use crate::scenes::SceneState;
use crate::resources::dictionary::Dictionary;

const BUTTON_HEIGHT: f32 = 150.0;
const BUTTON_WIDTH: f32 = 70.0;
const BUTTON_FONT_SIZE: f32 = 32.0;

#[derive( Clone )]
enum MainButtonComponent{
    Return,
    Start,
}

enum IncreaseDecreseButtonComponent{

}

pub struct CreateCharSceneData{
    pub user_interface_root: Entity,
}

pub struct CreateCharScenePlugin;

impl Plugin for CreateCharScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::MainMenuScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::MainMenuScene ).with_system( button_handle_system ));
        app.add_system_set( SystemSet::on_exit( SceneState::MainMenuScene ).with_system( cleanup ));
    }
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    dictionary: Res<Dictionary>,
    font: Res<FontMaterials>,
){
    let user_interface_root = commands
        .spawn_bundle( NodeBundle{
            style: Style{ 
                position_type: PositionType::Absolute,
                size: Size::new( Val::Percent(100.0), Val::Percent( 100.0 )),
                ..Default::default()
            },
            //image: UiImage( material_manager.main_menu_scene_material.background_image.clone() ),
            ..Default::default()
        })
        .with_children(|parent|{
            create_buttons( parent, &font, dictionary );
        })
        .id();
    
    commands.insert_resource( CreateCharSceneData{ 
        user_interface_root: user_interface_root,
    });

}

fn create_buttons( 
    root: &mut ChildBuilder, 
    font: &Res<FontMaterials>, 
    dictionary: Res<Dictionary>,
){
    let glossary = dictionary.get_glossary();

    let position: Rect<Val> = Rect {
        left: Val::Auto,
        right: Val::Px( BUTTON_HEIGHT + BUTTON_HEIGHT ), // 2 buttons together
        top: Val::Auto,
        bottom: Val::Px( BUTTON_WIDTH )
    };

    let size: Size<Val> = Size { 
        width: Val::Px( BUTTON_WIDTH ),
        height: Val::Px( BUTTON_HEIGHT ),
    };

    root.spawn_bundle( ButtonBundle {
        style: Style{
            size,
            justify_content: JustifyContent::Center,
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            align_self: AlignSelf::FlexEnd,
            position,
            ..Default::default()
        },
        color: UiColor( Color::NONE ),
        ..Default::default()
    })
    .with_children(|parent|{
        let text: &str = glossary.create_char_scene.back.as_str();

        parent.spawn_bundle( TextBundle{
            text: Text::with_section(
                text, 
                TextStyle {
                    font: font.get_font( dictionary.get_current_language() ),
                    font_size: BUTTON_FONT_SIZE,
                    color: Color::GRAY,
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

fn button_handle_system(){}

fn cleanup(
    mut commands: Commands,
    scene_data: Res<CreateCharSceneData>,
){
    commands.entity( scene_data.user_interface_root ).despawn_recursive();
}