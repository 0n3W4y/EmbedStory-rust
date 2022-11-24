use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::resources::dictionary::Dictionary;
use crate::resources::setting::Setting;
use crate::scenes::SceneState;

const OPTIONS_SCENE_BUTTON_WIDTH: f32 = 150.0;
const OPTIONS_SCENE_BUTTON_HEIGHT: f32 = 40.0;

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
    scenes_material: Res<MaterialManager>, 
    setting: Res<Setting>, 
    dictionary: Res<Dictionary> 
    ){
        let user_interface_root = commands
            .spawn_bundle( NodeBundle{
                style: Style{
                    size: Size::new( Val::Percent( 100.0), Val::Percent( 100.0 )),
                    ..Default::default()
                },
                image: UiImage( scenes_material.options_scene_material.background_image.clone() ),
                ..Default::default()
            })
            .with_children( |parent|{
                texts( parent, &font, &dictionary );
                buttons();
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

    parent.spawn_bundle( NodeBundle{
        ..Default::default()
    })
}