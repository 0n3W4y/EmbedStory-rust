pub mod loading_scene;
pub mod main_menu_scene;
pub mod options_scene;
pub mod game_scenes;
pub mod create_char_scene;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    LoadingScene,
    MainMenuScene,
    OptionsScene,
    CreateCharScene,
    LoadPreviousGameScene,
    GameGroundScene,
    GameUndergroundScene,
    GameGlobalMapScene,
}