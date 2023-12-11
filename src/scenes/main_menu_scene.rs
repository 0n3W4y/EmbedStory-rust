use bevy::app::AppExit;
use bevy::prelude::*;
use std::slice::Iter;

use crate::scenes::SceneState;
use crate::resources::dictionary::Dictionary;
use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::config::{ MAIN_MENU_BUTTON_FONT_SIZE, MONITOR_HEIGHT };

const MAIN_MENU_BUTTON_WIDTH: f32 = 150.0;
const MAIN_MENU_BUTTON_HEIGHT: f32 = MAIN_MENU_BUTTON_FONT_SIZE + 10.0;

#[derive(Component, Copy, Clone)]
enum ButtonComponent{
    Play,
    Load,
    Options,
    Quit
}

impl ButtonComponent{
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Play,
            ButtonComponent::Load,
            ButtonComponent::Options,
            ButtonComponent::Quit,
        ].iter()
    }
}

struct MainMenuSceneData{
    user_interface_root: Entity,
}

pub struct MainMenuScenePlugin;

impl Plugin for MainMenuScenePlugin{
    fn build( &self, app: &mut App ){
        app.add_system_set(SystemSet::on_enter( SceneState::MainMenuScene ).with_system( setup ));
        app.add_system_set( SystemSet::on_update( SceneState::MainMenuScene ).with_system( button_handle_system ));
        app.add_system_set( SystemSet::on_exit( SceneState::MainMenuScene ).with_system( cleanup ));
    }
}

fn setup( mut commands: Commands, dictionary: Res<Dictionary>, font: Res<FontMaterials>, material_manager: Res<MaterialManager>){
    let user_interface_root = commands
        .spawn_bundle( NodeBundle{
            style: Style{ 
                position_type: PositionType::Absolute,
                size: Size::new( Val::Percent(100.0), Val::Percent( 100.0 )),
                ..Default::default()
            },
            image: UiImage( material_manager.main_menu_scene.background_image.clone() ),
            ..Default::default()
        })
        .with_children(|parent|{
            create_buttons( parent, &font, dictionary );
        })
        .id();
    commands.insert_resource( MainMenuSceneData{ 
         user_interface_root: user_interface_root, 
    });
}

fn cleanup( mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData> ){
    commands.entity( main_menu_scene_data.user_interface_root ).despawn_recursive();
}

fn create_buttons(root: &mut ChildBuilder, font: &Res<FontMaterials>, dictionary: Res<Dictionary> ){
    let glossary = dictionary.get_glossary();

    for( index, button ) in ButtonComponent::iterator().enumerate(){
        let position: UiRect<Val> = UiRect { 
            left: Val::Px( 100.0 ), 
            right: Val::Auto, 
            top: Val::Px( MONITOR_HEIGHT / 2.0 + MAIN_MENU_BUTTON_HEIGHT * ( index as f32 + 1.0 )), 
            bottom: Val::Auto,
        };

        let size: Size<Val> = Size { 
            width: Val::Px( MAIN_MENU_BUTTON_WIDTH ),
            height: Val::Px( MAIN_MENU_BUTTON_HEIGHT ),
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
        .with_children( |parent|{
            let text: &str = match button{
                ButtonComponent::Play => glossary.main_menu_text.play.as_str(),
                ButtonComponent::Load => glossary.main_menu_text.load.as_str(),
                ButtonComponent::Options => glossary.main_menu_text.options.as_str(),
                ButtonComponent::Quit => glossary.main_menu_text.quit.as_str(),
            };

            parent.spawn_bundle( TextBundle{
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: font.get_font( dictionary.get_current_language() ),
                        font_size: MAIN_MENU_BUTTON_FONT_SIZE,
                        color: Color::GRAY,
                    }
                ),
                ..Default::default()
            }
            .with_text_alignment(TextAlignment {
                vertical: VerticalAlign::Center,
                horizontal: HorizontalAlign::Center,
                })
            );
        })
        .insert( button.clone() );
    }
}

fn button_handle_system( 
    mut button_query: Query<( &Interaction, &ButtonComponent, &mut UiColor ), ( Changed<Interaction>, With<Button> )>,
    mut state: ResMut<State<SceneState>>,
    mut exit: EventWriter<AppExit>
){
    for( interaction, button, mut color ) in button_query.iter_mut(){
        match *interaction{
            Interaction::None => *color = UiColor( Color::NONE ),
            Interaction::Hovered => *color = UiColor( Color::rgb(0.25, 0.25, 0.25 )),
            Interaction::Clicked => {
                *color = UiColor( Color::rgb(0.25, 0.75, 0.25));
                match button{
                    //ButtonComponent::Play => state.set( SceneState::CreateCharScene).expect( "Could not load CreateCharacterScene"),
                    ButtonComponent::Play => state.set( SceneState::CreateCharScene).expect( "Could not load GameGroundScene"),
                    ButtonComponent::Load => state.set( SceneState::LoadPreviousGameScene ).expect( "Could not load LoadPreviousGameScene"),
                    ButtonComponent::Options => state.set( SceneState::OptionsScene ).expect( "Could not load OptionsScene" ),
                    ButtonComponent::Quit => exit.send(AppExit),
                }
            }
        }
    }
}