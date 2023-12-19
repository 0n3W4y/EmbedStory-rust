use bevy::ecs::schedule::States;

pub mod loading_scene;
pub mod main_menu_scene;
pub mod options_scene;
pub mod game_scenes;
pub mod create_char_scene;
pub mod loading_new_game_scene;


#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    #[default]
    LoadingScene,
    MainMenuScene,
    OptionsScene,
    CreateCharScene,
    LoadPreviousGameScene,
    GameScene,
    LoadingNewGameScene,
}