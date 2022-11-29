use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::resources::dictionary::Dictionary;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

const OPTIONS_SCENE_RETURN_BUTTON_WIDTH: f32 = 150.0;
const OPTIONS_SCENE_RETURN_BUTTON_HEIGHT: f32 = 40.0;

const OPTIONS_SCENE_SIMPLY_BUTTON_WIDTH: f32 = 80.0;
const OPTIONS_SCENE_SIMPLY_BUTTON_HEIGHT: f32 = 40.0;

const OPTIONS_SCENE_SIMPLY_BUTTON_SELECTED: Color = Color::Rgba{ red:( 200.0 / 255.0 ), green:( 100.0 / 255.0 ) , blue:( 70.0 / 255.0 ) , alpha: 1.0 };
const OPTIONS_SCENE_SIMPLY_BUTTON_NORMAL: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };

const TEXT_TOP_POSITION: f32 = 300.0;
const TEXT_LEFT_POSITION: f32 = 200.0;
const TEXT_SKIP_HEIGHT: f32 = 10.0;

const TEXT_OPTIONS_FONT_SIZE: f32 = 52.0;
const TEXT_FONT_SIZE: f32 = 32.0;

#[derive(Component, Copy, Clone)]
enum ButtonComponent{
    EnableMusic,
    EnableSound,
}

impl ButtonComponent{
    pub fn iterator() -> Iter<'static, ButtonComponent>{
        [
            ButtonComponent::EnableMusic,
            ButtonComponent::EnableSound,
        ].iter()
    }
}

#[derive(Component, Copy, Clone)]
enum LanguageButtonComponent{
    LanguageRU,
    LanguageEN
}

impl LanguageButtonComponent{
    pub fn iterator() -> Iter<'static, LanguageButtonComponent>{
        [
            LanguageButtonComponent::LanguageRU,
            LanguageButtonComponent::LanguageEN,
        ].iter()
    }
}

#[derive(Component, Clone)]
enum TextComponent{
    Options,
    EnableMusic,
    EnableSound,
    Language
}

impl TextComponent{
    pub fn iterator() -> Iter<'static, TextComponent>{
        [
            TextComponent::Options,
            TextComponent::EnableMusic,
            TextComponent::EnableSound,
            TextComponent::Language,
        ].iter()
    }
}
pub struct OptionsScenePlugin;

struct OptionsSceneData{
    user_interface_root: Entity,
}

impl Plugin for OptionsScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set( SystemSet::on_enter( SceneState::OptionsScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::OptionsScene ).with_system( update ));
        app.add_system_set( SystemSet::on_exit( SceneState::OptionsScene ).with_system( cleanup ));
    }
}

fn setup( 
    mut commands: Commands, 
    font: Res<FontMaterials>,
    material_manager: Res<MaterialManager>, 
    setting: Res<Setting>, 
    dictionary: Res<Dictionary> 
    ){
        let user_interface_root = commands
            .spawn_bundle( NodeBundle{
                style: Style{
                    size: Size::new( Val::Percent( 100.0), Val::Percent( 100.0 )),
                    ..Default::default()
                },
                image: UiImage( material_manager.options_scene_material.background_image.clone() ),
                ..Default::default()
            })
            .with_children( |parent|{
                texts( parent, &font, &dictionary );
                buttons( parent, &setting, &material_manager, &font, &dictionary );
                language_buttons( parent, &setting, &material_manager, &font, &dictionary );
                return_button( parent, &font, &dictionary );
            })
            .id();

        commands.insert_resource( OptionsSceneData {
            user_interface_root: user_interface_root,
        });
}

fn cleanup( 
    mut commands: Commands,
    setting: Res<Setting>,
    option_scene_data: Res<OptionsSceneData> 
    ){
        setting.save_setting();
        commands.entity( option_scene_data.user_interface_root ).despawn_recursive();
}

fn texts( parent: &mut ChildBuilder, font_material: &FontMaterials, dictionary: &Dictionary ){
    let font = font_material.get_font( dictionary.get_current_language() );
    let glossary = dictionary.get_glossary();

    for( index, prevalue ) in TextComponent::iterator().enumerate(){
        let value: String = match index{
            0 => glossary.options_text.options.clone(),
            1 => glossary.options_text.enable_music.clone(),
            2 => glossary.options_text.enable_sound.clone(),
            3 => glossary.options_text.language.clone(),
            _ => panic!( "Error in options_scene.rs TextComponent not available" ),
        };

        let component_name = match index {
            0 => "OptionsText",
            1 => "EnableMusicText",
            2 => "EnableSoundText",
            3 => "LanguageText",
            _ => "Unknown text",
        };

        let font_size: f32 = match index {
            0 => TEXT_OPTIONS_FONT_SIZE,
            _ => TEXT_FONT_SIZE,
        };

        let top_position: f32 = TEXT_TOP_POSITION + index as f32 * ( TEXT_SKIP_HEIGHT + font_size );
        let left_position: f32 = match index {
            0 => TEXT_LEFT_POSITION + 100.0,
            _ => TEXT_LEFT_POSITION,
        };

        parent.spawn_bundle( TextBundle{
            style: Style{
                position_type: PositionType::Absolute,
                position: Rect{
                    left: Val::Px( left_position ),
                    top: Val::Px( top_position ),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                value, 
                TextStyle {
                    font: font.clone(),
                    font_size,
                    color: Color::BLACK,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            ..Default::default()
        })
        .insert(Name::new( component_name ))
        .insert(prevalue.clone() );
    };
}

fn buttons( parent: &mut ChildBuilder, setting: &Setting, scenes_material: &MaterialManager, font_material: &FontMaterials, dictionary: &Dictionary ){
    for( index, button_component ) in ButtonComponent::iterator().enumerate(){
        let size = Size{
            width: Val::Px( OPTIONS_SCENE_SIMPLY_BUTTON_WIDTH ),
            height: Val::Px( OPTIONS_SCENE_SIMPLY_BUTTON_HEIGHT ),
        };

        let component_name_button_on = match button_component{
            ButtonComponent::EnableMusic => "EnableMusicOn",
            ButtonComponent::EnableSound => "EnableSoundOn",
        };

        let component_name_button_off: &str = match button_component{
            ButtonComponent::EnableMusic => "EnableMusicOff",
            ButtonComponent::EnableSound => "EnableSoundOff",
        };

        let first_top_position = TEXT_TOP_POSITION + TEXT_OPTIONS_FONT_SIZE + TEXT_SKIP_HEIGHT;
        let left_button_position: f32 = 600.0;
        let font = font_material.get_font( dictionary.get_current_language() );
        let text_button_on = dictionary.get_glossary().options_text.on.clone();
        let text_button_off = dictionary.get_glossary().options_text.off.clone();

        let position_button_on = Rect {
            left: Val::Px( left_button_position ),
            top: Val::Px( first_top_position + index as f32 * OPTIONS_SCENE_SIMPLY_BUTTON_HEIGHT ),
            right: Val::Auto,
            bottom: Val::Auto,
        };
        let position_button_off: Rect<Val> = Rect { 
            left: Val::Px( left_button_position + OPTIONS_SCENE_SIMPLY_BUTTON_WIDTH ),
            right: Val::Px( first_top_position + index as f32 * OPTIONS_SCENE_SIMPLY_BUTTON_HEIGHT ), 
            top: Val::Auto, 
            bottom: Val::Auto, 
        };

        parent.spawn_bundle( ButtonBundle{
            style: Style {
                position: position_button_on,
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor( OPTIONS_SCENE_SIMPLY_BUTTON_SELECTED ),
            ..Default::default()
        })
        .with_children( |root| {
            root.spawn_bundle( TextBundle{
                text: Text::with_section(
                    text_button_on, 
                    TextStyle{
                        font: font,
                        font_size: TEXT_OPTIONS_FONT_SIZE,
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
        .insert( button_component.clone())
        .insert( Name::new( component_name_button_on ));

        parent.spawn_bundle( ButtonBundle{
            style: Style{
                position: position_button_off,
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor( OPTIONS_SCENE_SIMPLY_BUTTON_NORMAL ),
            ..Default::default()
        })
        .with_children( |root|{
            root.spawn_bundle( TextBundle{
                text: Text::with_section(
                    text_button_off, 
                    TextStyle{
                        font: font,
                        font_size: TEXT_OPTIONS_FONT_SIZE,
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
        .insert( button_component.clone() )
        .insert( Name::new( component_name_button_off ));
    }
}

fn language_buttons( parent: &mut ChildBuilder, setting: &Setting, material_manager: &MaterialManager, font_material: &FontMaterials ){

}

fn return_button( parent: &mut ChildBuilder, font_material: &FontMaterials ){

}