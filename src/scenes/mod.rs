pub mod loading_scene;
pub mod main_menu_scene;
pub mod options_scene;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    LoadingScene,
    MainMenuScene,
    OptionsScene,
    CreateCharacterScene,
    LoadPreviousGameScene,
}