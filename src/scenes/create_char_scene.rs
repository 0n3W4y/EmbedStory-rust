use bevy::prelude::*;
use std::slice::Iter;

use crate::materials::font::FontMaterials;
use crate::materials::material_manager::MaterialManager;
use crate::resources::charactor_manager::CharactorManager;
use crate::resources::deploy::game_scene_deploy::Location;
use crate::resources::dictionary::Dictionary;
use crate::resources::profile::Profile;
use crate::resources::scene_data::charactor::{GenderType, RaceType};
use crate::resources::scene_manager::SceneManager;
use crate::resources::stuff_manager::StuffManager;
use crate::resources::thing_manager::ThingManager;
use crate::scenes::SceneState;
use crate::resources::deploy::Deploy;

const BUTTON_HEIGHT: f32 = 40.0;
const BUTTON_WIDTH: f32 = 100.0;
const BUTTON_FONT_SIZE: f32 = 32.0;
const BUTTON_NORMAL_COLOR: Color = Color::Rgba {
    red: (100.0 / 255.0),
    green: (50.0 / 255.0),
    blue: (20.0 / 255.0),
    alpha: 1.0,
};
const BUTTON_HOVER_COLOR: Color = Color::Rgba {
    red: (150.0 / 255.0),
    green: (75.0 / 255.0),
    blue: (45.0 / 255.0),
    alpha: 1.0,
};
const BUTTON_SELECT_COLOR: Color = Color::Rgba {
    red: (10.0 / 255.0),
    green: (200.0 / 255.0),
    blue: (70.0 / 255.0),
    alpha: 1.0,
};

#[derive(Component, Clone)]
enum MainButtonComponent {
    Back,
    Start,
}

impl MainButtonComponent {
    pub fn iterator() -> Iter<'static, MainButtonComponent> {
        [MainButtonComponent::Back, MainButtonComponent::Start].iter()
    }
}

enum IncreaseDecreaseButtonComponent {
    IncreaseStrength,
    DecreaseStrength,
    IncreaseDexterity,
    DecreaseDexterity,
    IncreasePerception,
    DecreasePerception,
    IcreaseEndurace,
    DecreaseEndurance,
    IncreaseIntellect,
    DecreaseIntellect,
}

#[derive(Component, Clone)]
enum StatsTextComponent{
    Strength,
    Endurance,
    Intellect,
    Dexterity,
    Perception,
}

impl StatsTextComponent{
    pub fn iterator() -> Iter<'static, StatsTextComponent>{
        [
            StatsTextComponent::Strength,
            StatsTextComponent::Endurance,
            StatsTextComponent::Intellect,
            StatsTextComponent::Dexterity,
            StatsTextComponent::Perception,
        ].iter()
    }
}

#[derive(Component, Clone)]
enum ResistsTextComponent{
    Kinetic,
    Fire,
    Electric,
    Plasma,
    Laser,
    Poison,
    Knockdown,
    Bleed,
    Disease,
    Pain,
    Fatigue,  
}

impl ResistsTextComponent{
    pub fn iterator() -> Iter<'static, ResistsTextComponent>{
        [
            ResistsTextComponent::Kinetic,
            ResistsTextComponent::Fire,
            ResistsTextComponent::Electric,
            ResistsTextComponent::Plasma,
            ResistsTextComponent::Laser,
            ResistsTextComponent::Poison,
            ResistsTextComponent::Knockdown,
            ResistsTextComponent::Bleed,
            ResistsTextComponent::Disease,
            ResistsTextComponent::Pain,
            ResistsTextComponent::Fatigue,
        ].iter()
    }
}

pub struct CreateCharSceneData {
    pub user_interface_root: Entity,
}

pub struct CreateCharScenePlugin;

impl Plugin for CreateCharScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::CreateCharScene).with_system(setup));
        app.add_system_set(
            SystemSet::on_update(SceneState::CreateCharScene).with_system(button_handle_system),
        );
        app.add_system_set(SystemSet::on_exit(SceneState::CreateCharScene).with_system(cleanup));
    }
}

fn setup(
    mut commands: Commands,
    dictionary: Res<Dictionary>,
    font: Res<FontMaterials>,
    material_manager: Res<MaterialManager>,
    deploy: Res<Deploy>,
) {
    let user_interface_root = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                ..Default::default()
            },
            image: UiImage(material_manager.create_char_scene.background_image.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            create_buttons(parent, &font, dictionary);
            //create_text(parent, &font, dictionary);
        })
        .id();

    commands.insert_resource(CreateCharSceneData {
        user_interface_root: user_interface_root,
    });

    let mut charactor_manager: CharactorManager = Default::default();
    let player = charactor_manager.create_player(
        &deploy, 
        &RaceType::Human,
        &GenderType::Male,
    );

    let mut profile: Profile = Default::default();
    profile.charactor = Some(player);

    let mut scene_manager: SceneManager = Default::default();                   // Create new scene_manager;     
    let mut thing_manager: ThingManager = Default::default();                      //create new object manager;
    let stuff_manager: StuffManager = Default::default();                       //create new stuff manager;

    let next_scene = scene_manager.generate_new_scenes(
        &deploy, 
        &mut thing_manager, 
        &mut charactor_manager, 
        &profile,
        &Location::ElvenPlains
    );
    scene_manager.next_game_scene = Some(next_scene.scene_id);

    commands.insert_resource(charactor_manager);
    commands.insert_resource(profile);
    commands.insert_resource(scene_manager);
    commands.insert_resource(thing_manager);
    commands.insert_resource(stuff_manager);
}

fn create_buttons(root: &mut ChildBuilder, font: &Res<FontMaterials>, dictionary: Res<Dictionary>) {
    let glossary = dictionary.get_glossary();

    for (index, button_component) in MainButtonComponent::iterator().enumerate() {
        let position: UiRect<Val> = UiRect {
            left: Val::Auto,
            right: Val::Px(BUTTON_WIDTH + index as f32 * BUTTON_WIDTH), // 2 buttons together
            top: Val::Auto,
            bottom: Val::Px(BUTTON_HEIGHT),
        };

        let size: Size<Val> = Size {
            width: Val::Px(BUTTON_WIDTH),
            height: Val::Px(BUTTON_HEIGHT),
        };

        root.spawn_bundle(ButtonBundle {
            style: Style {
                size,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                align_self: AlignSelf::FlexEnd,
                position,
                ..Default::default()
            },
            color: UiColor(BUTTON_NORMAL_COLOR),
            ..Default::default()
        })
        .with_children(|parent| {
            let text: &str = match button_component {
                MainButtonComponent::Back => glossary.create_char_scene.back.as_str(),
                MainButtonComponent::Start => glossary.create_char_scene.start.as_str(),
            };

            parent.spawn_bundle(TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: font.get_font(dictionary.get_current_language()),
                        font_size: BUTTON_FONT_SIZE,
                        color: Color::GRAY,
                    }
                ),
                ..Default::default()
            }
            .with_text_alignment(
                TextAlignment { 
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                }
            )
        );
        })
        .insert(button_component.clone());
    }
}
/*
fn create_text(
    root: &mut ChildBuilder,
    font_material: &Res<FontMaterials>, 
    dictionary: Res<Dictionary>
){
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
*/
fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &MainButtonComponent, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut state: ResMut<State<SceneState>>,
    mut profile: ResMut<Profile>,
    //mut charactor_manager: ResMut<CharactorManager>,
    //deploy: Res<Deploy>,
) {
    for (interaction, button_component, mut color) in button_query.iter_mut() {
        match *button_component {
            MainButtonComponent::Back => match *interaction {
                Interaction::None => {
                    *color = UiColor(BUTTON_NORMAL_COLOR);
                }
                Interaction::Hovered => {
                    *color = UiColor(BUTTON_HOVER_COLOR);
                }
                Interaction::Clicked => {
                    *color = UiColor(BUTTON_SELECT_COLOR);
                    state
                        .set(SceneState::MainMenuScene)
                        .expect("Couldn't switch state to Main Menu Scene");
                }
            },
            MainButtonComponent::Start => match *interaction {
                Interaction::None => {
                    *color = UiColor(BUTTON_NORMAL_COLOR);
                }
                Interaction::Hovered => {
                    *color = UiColor(BUTTON_HOVER_COLOR);
                }
                Interaction::Clicked => {
                    *color = UiColor(BUTTON_SELECT_COLOR);
                    //TODO: check name, check stats all checks;
                    // if all good -> set name, start game;
                    profile.set_name("Test Player Name".to_string());
                    //TODO: start @new game intro@, then load loading_scene to load new global map and current ground scene;
                    state
                        .set(SceneState::LoadingNewGameScene)
                        .expect("Couldn't switch state to Loading New Game Scene");
                }
            },
        }
    }
}

fn cleanup(mut commands: Commands, scene_data: Res<CreateCharSceneData>) {
    commands
        .entity(scene_data.user_interface_root)
        .despawn_recursive();
}
