use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::resources::dictionary::Dictionary;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;
use crate::resources::language::Language;

const OPTIONS_SCENE_RETURN_BUTTON_WIDTH: f32 = 150.0;
const OPTIONS_SCENE_RETURN_BUTTON_HEIGHT: f32 = 40.0;

const OPTIONS_SCENE_ON_OFF_BUTTON_WIDTH: f32 = 80.0;
const OPTIONS_SCENE_ON_OFF_BUTTON_HEIGHT: f32 = 40.0;

const OPTIONS_SCENE_ON_OFF_BUTTON_SELECTED: Color = Color::Rgba{ red:( 200.0 / 255.0 ), green:( 100.0 / 255.0 ) , blue:( 70.0 / 255.0 ) , alpha: 1.0 };
const OPTIONS_SCENE_ON_OFF_BUTTON_NORMAL: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };

const OPTIONS_SCENE_LANGUAGE_BUTTON_NORMAL_COLOR: Color = Color::Rgba{ red:( 175.0 / 255.0 ), green:( 0.0 ), blue:( 0.0 ), alpha: 0.5 };
const OPTIONS_SCENE_LANGUAGE_BUTTON_HOVER_COLOR: Color = Color::Rgba{ red:( 0.0 ), green:( 75.0 / 255.0 ), blue:( 0.0 ), alpha: 0.5 };
const OPTIONS_SCENE_LANGUAGE_BUTTON_SELECTED_COLOR: Color = Color::Rgba{ red:( 0.0 ), green:( 175.0/ 255.0 ), blue:( 0.0 ), alpha: 0.5 };

const TEXT_OPTIONS_TOP_POSITION: f32 = 300.0;
const TEXT_OPTIONS_LEFT_POSITION: f32 = 300.0;

const TEXT_ENABLESOUND_TOP_POSITION: f32 = 400.0;
const TEXT_ENABLESOUND_LEFT_POSITION: f32 = 300.0;

const TEXT_ENABLEMUSIC_TOP_POSITION: f32 = 450.0;
const TEXT_ENABLEMUSIC_LEFT_POSITION: f32 = 300.0;

const TEXT_LANGUAGE_TOP_POSITION: f32 = 500.0;
const TEXT_LANGUAGE_LEFT_POSITION: f32 = 300.0;

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
                buttons( parent, &setting, &font, &dictionary );
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

        let position = match index{
            0 => Rect{
                left: Val::Px( TEXT_OPTIONS_LEFT_POSITION ),
                top: Val::Px( TEXT_OPTIONS_TOP_POSITION ),
                ..Default::default()
            },
            1 => Rect{
                left: Val::Px( TEXT_ENABLEMUSIC_LEFT_POSITION ),
                top: Val::Px( TEXT_ENABLEMUSIC_TOP_POSITION ),
                ..Default::default()
            },
            2 => Rect{
                left: Val::Px( TEXT_ENABLESOUND_LEFT_POSITION ),
                top: Val::Px( TEXT_ENABLESOUND_TOP_POSITION ),
                ..Default::default()
            },
            3 => Rect{
                left: Val::Px( TEXT_LANGUAGE_LEFT_POSITION ),
                top: Val::Px( TEXT_LANGUAGE_TOP_POSITION ),
                ..Default::default()
            },
            _ => panic!( "unknown position for index 4", ),
        };

        parent.spawn_bundle( TextBundle{
            style: Style{
                position_type: PositionType::Absolute,
                position: position,
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

fn buttons( parent: &mut ChildBuilder, setting: &Setting, font_material: &FontMaterials, dictionary: &Dictionary ){
    for( index, button_component ) in ButtonComponent::iterator().enumerate(){
        let size = Size{
            width: Val::Px( OPTIONS_SCENE_ON_OFF_BUTTON_WIDTH ),
            height: Val::Px( OPTIONS_SCENE_ON_OFF_BUTTON_HEIGHT ),
        };

        let component_name_button_on = match button_component{
            ButtonComponent::EnableMusic => "EnableMusicOn",
            ButtonComponent::EnableSound => "EnableSoundOn",
        };

        let component_name_button_off: &str = match button_component{
            ButtonComponent::EnableMusic => "EnableMusicOff",
            ButtonComponent::EnableSound => "EnableSoundOff",
        };

        let font = font_material.get_font( dictionary.get_current_language() );
        let text_button_on = dictionary.get_glossary().options_text.on.clone();
        let text_button_off = dictionary.get_glossary().options_text.off.clone();

        let position_button_on = match button_component{
            ButtonComponent::EnableMusic => Rect{
                left: Val::Px( TEXT_ENABLEMUSIC_LEFT_POSITION + 300.0 ),
                top: Val::Px( TEXT_ENABLEMUSIC_TOP_POSITION ),
                right: Val::Auto,
                bottom: Val::Auto,
            },
            ButtonComponent::EnableSound => Rect{
                left: Val::Px( TEXT_ENABLESOUND_LEFT_POSITION + 300.0 ),
                top: Val::Px( TEXT_ENABLESOUND_TOP_POSITION ),
                right: Val::Auto,
                bottom: Val::Auto,
            },
        };
        let position_button_off: Rect<Val> = match button_component{ 
            ButtonComponent::EnableMusic => Rect { 
                left: Val::Px( TEXT_ENABLEMUSIC_LEFT_POSITION + 300.0 ),
                right: Val::Px( TEXT_ENABLEMUSIC_TOP_POSITION ), 
                top: Val::Auto, 
                bottom: Val::Auto,
            },
            ButtonComponent::EnableSound => Rect {
                left: Val::Px( TEXT_ENABLESOUND_LEFT_POSITION + 300.0 ),
                right: Val::Px( TEXT_ENABLESOUND_TOP_POSITION ), 
                top: Val::Auto, 
                bottom: Val::Auto,
            },
        };

        parent.spawn_bundle( ButtonBundle{
            style: Style {
                position: position_button_on,
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor( OPTIONS_SCENE_ON_OFF_BUTTON_SELECTED ),
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
            color: UiColor( OPTIONS_SCENE_ON_OFF_BUTTON_NORMAL ),
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

fn language_buttons( parent: &mut ChildBuilder, setting: &Setting, material_manager: &MaterialManager, font_material: &FontMaterials, dictionary: &Dictionary ){
    for( index, button_component ) in LanguageButtonComponent::iterator().enumerate(){
        let position = Rect{ 
                left: Val::Px( TEXT_LANGUAGE_LEFT_POSITION + 300.0 + index as f32 * OPTIONS_SCENE_ON_OFF_BUTTON_HEIGHT ),
                top: Val::Px( TEXT_LANGUAGE_TOP_POSITION ),
                right: Val::Auto,
                bottom: Val::Auto,
        };

        let component_name = match button_component{
            LanguageButtonComponent::LanguageEN => "LanguageEN",
            LanguageButtonComponent::LanguageRU => "LanguageRU",
        };

        let handle_image: Handle<Image> = match button_component{
            LanguageButtonComponent::LanguageEN => material_manager.options_scene_material.language_en.clone(),
            LanguageButtonComponent::LanguageRU => material_manager.options_scene_material.language_ru.clone(),
        };

        let color: Color = match button_component{
            LanguageButtonComponent::LanguageEN => match dictionary.get_current_language(){
                Language::EN => OPTIONS_SCENE_LANGUAGE_BUTTON_SELECTED_COLOR,
                _ => OPTIONS_SCENE_LANGUAGE_BUTTON_NORMAL_COLOR,
            },
            LanguageButtonComponent::LanguageRU => match dictionary.get_current_language(){
                Language::RU => OPTIONS_SCENE_LANGUAGE_BUTTON_SELECTED_COLOR,
                _ => OPTIONS_SCENE_LANGUAGE_BUTTON_NORMAL_COLOR,
            }
        };

        parent.spawn_bundle( ButtonBundle{
            style:Style {
                position: position,
                size: Size { 
                    width: Val::Px( OPTIONS_SCENE_ON_OFF_BUTTON_WIDTH ), 
                    height: Val::Px( OPTIONS_SCENE_ON_OFF_BUTTON_HEIGHT ), 
                },
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: UiColor( color ),
            image: UiImage( handle_image ),
            ..Default::default()
        })
        .insert( Name::new( component_name ))
        .insert( button_component.clone() );

    }
}

fn return_button( parent: &mut ChildBuilder, font_material: &FontMaterials, dictionary: &Dictionary ){

}