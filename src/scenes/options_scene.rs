use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::resources::dictionary::Dictionary;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

const OPTIONS_SCENE_BUTTON_WIDTH: f32 = 150.0;
const OPTIONS_SCENE_BUTTON_HEIGHT: f32 = 40.0;

const TEXT_TOP_POSITION: f32 = 300.0;
const TEXT_LEFT_POSITION: f32 = 200.0;
const TEXT_SKIP_HEIGHT: f32 = 10.0;

#[derive(Component, Copy, Clone)]
enum ButtonComponent{
    EnableSound,
    EnableMusic,
    Return
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
        app.add_system_set(SystemSet::on_enter( SceneState::OptionsScene ).with_system( setup ));
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
                buttons( parent, &setting, &material_manager );
                pair_buttons();
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

fn texts( parent: &mut ChildBuilder, font_material: &Res<FontMaterials>, dictionary: &Res<Dictionary> ){
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

        let top_position: f32 = TEXT_TOP_POSITION + index as f32 * TEXT_SKIP_HEIGHT;
        let left_position: f32 = match index {
            0 => TEXT_LEFT_POSITION + 100.0,
            _ => TEXT_LEFT_POSITION,
        };

        let font_size: f32 = match index {
            0 => 52.0,
            _ => 32.0,
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
        .insert(Name::new(component_name))
        .insert(prevalue.clone());
    };
}

fn buttons( parent: &mut ChildBuilder, setting: &Setting, scenes_material: &MaterialManager ){
    for( index, button_component ) in ButtonComponent.iterator().enumerate(){
        parent.spawn_bundle( ButtonBundle{
            
        } )
    }
}