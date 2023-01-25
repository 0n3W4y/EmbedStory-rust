use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::scenes::SceneState;
use crate::resources::dictionary::Dictionary;
use crate::materials::material_manager::MaterialManager;
use crate::resources::profile::Profile;

const BUTTON_HEIGHT: f32 = 150.0;
const BUTTON_WIDTH: f32 = 70.0;
const BUTTON_FONT_SIZE: f32 = 32.0;
const BUTTON_NORMAL_COLOR: Color = Color::Rgba{ red:( 100.0 / 255.0 ), green:( 50.0 / 255.0 ) , blue:( 20.0 / 255.0 ) , alpha: 1.0 };
const BUTTON_HOVER_COLOR: Color = Color::Rgba{ red:( 150.0 / 255.0 ), green:( 75.0 / 255.0 ), blue:( 45.0 / 255.0 ), alpha: 1.0 };
const BUTTON_SELECT_COLOR: Color = Color::Rgba{ red:( 10.0 / 255.0 ), green:( 200.0 / 255.0 ) , blue:( 70.0 / 255.0 ) , alpha: 1.0 };

#[derive( Component, Clone )]
enum MainButtonComponent{
    Back,
    Start,
}

impl MainButtonComponent{
    pub fn iterator() -> Iter<'static, MainButtonComponent>{
        [
            MainButtonComponent::Back,
            MainButtonComponent::Start,
        ].iter()
    }
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
    dictionary: Res<Dictionary>,
    font: Res<FontMaterials>,
    material_manager: Res<MaterialManager>,
){
    let user_interface_root = commands
        .spawn_bundle( NodeBundle{
            style: Style{ 
                position_type: PositionType::Absolute,
                size: Size::new( Val::Percent(100.0), Val::Percent( 100.0 )),
                ..Default::default()
            },
            image: UiImage( material_manager.create_char_scene.background_image.clone() ),
            ..Default::default()
        })
        .with_children(|parent|{
            create_buttons( parent, &font, dictionary );
        })
        .id();
    
    commands.insert_resource( CreateCharSceneData{ 
        user_interface_root: user_interface_root,
    });

    commands.insert_resource( Profile::new() );
}

fn create_buttons( 
    root: &mut ChildBuilder, 
    font: &Res<FontMaterials>, 
    dictionary: Res<Dictionary>,
){
    let glossary = dictionary.get_glossary();

    for ( index, button_component ) in MainButtonComponent::iterator().enumerate(){
        let position: Rect<Val> = Rect {
            left: Val::Auto,
            right: Val::Px( BUTTON_HEIGHT + index as f32 * BUTTON_HEIGHT ), // 2 buttons together
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
            color: UiColor( BUTTON_NORMAL_COLOR ),
            ..Default::default()
        })
        .with_children(|parent|{
            let text: &str = match button_component{
                MainButtonComponent::Back =>  glossary.create_char_scene.back.as_str(),
                MainButtonComponent::Start =>  glossary.create_char_scene.start.as_str(),
            };
    
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
        })
        .insert( button_component.clone() );
    } 
}

fn button_handle_system(
    mut button_query: Query<( &Interaction, &MainButtonComponent, &mut UiColor ),( Changed<Interaction>, With<Button> )>,
    mut state: ResMut<State<SceneState>>,
    mut profile: ResMut<Profile>,
){
    for( interaction, button_component, mut color ) in button_query.iter_mut(){
        match *button_component {
            MainButtonComponent::Back => match *interaction{
                Interaction::None => { *color = UiColor( BUTTON_NORMAL_COLOR ); },
                Interaction::Hovered => { *color = UiColor( BUTTON_HOVER_COLOR ); },
                Interaction::Clicked =>{
                    *color = UiColor( BUTTON_SELECT_COLOR );
                    state.set( SceneState::MainMenuScene )
                        .expect( "Couldn't switch state to Main Menu Scene" );
                }
            },
            MainButtonComponent::Start => match *interaction{
                Interaction::None => { *color = UiColor( BUTTON_NORMAL_COLOR ); },
                Interaction::Hovered => { *color = UiColor( BUTTON_HOVER_COLOR ); },
                Interaction::Clicked => {
                    *color = UiColor( BUTTON_SELECT_COLOR );
                    //TODO: check name, check stats all checks;
                    // if all good -> set name, start game;
                    profile.set_name( "Test Player Name".to_string() );
                    //TODO: start @new game intro@, then load loading_scene to load new global map and current ground scene;
                    state.set( SceneState::LoadingNewGameScene )
                    .expect("Couldn't switch state to Loading New Game Scene");
                },
            },
        }
    }
}

fn cleanup(
    mut commands: Commands,
    scene_data: Res<CreateCharSceneData>,
){
    commands.entity( scene_data.user_interface_root ).despawn_recursive();
}