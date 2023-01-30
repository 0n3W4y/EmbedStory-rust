use bevy::prelude::*;

use crate::materials::main_menu_scene_material::MainMenuSceneMaterial;
use crate::materials::options_scene_material::OptionsSceneMaterial;
use crate::materials::create_char_scene_material::CreateCharSceneMaterial;
use crate::materials::loading_new_game_scene_material::LoadingNewGameSceneMaterial;
use crate::materials::game_ground_scene_material::GameGroundSceneMaterial;


#[derive( Debug, Clone )]
pub struct MaterialManager {
    pub main_menu_scene: MainMenuSceneMaterial,
    pub options_scene: OptionsSceneMaterial,
    pub create_char_scene: CreateCharSceneMaterial,
    pub loading_new_game_scene: LoadingNewGameSceneMaterial,
    pub ground_scene: GameGroundSceneMaterial,
}

impl MaterialManager{

    pub fn new( asset_server: &Res<AssetServer> ) -> Self {
        return MaterialManager { 
            main_menu_scene: MaterialManager::load_main_menu_scene_material( asset_server ),
            options_scene: MaterialManager::load_options_scene_material( asset_server ),
            ground_scene: GameGroundSceneMaterial::load_ground_scene_material( asset_server ),
            create_char_scene: MaterialManager::load_create_char_scene_material( asset_server ),
            loading_new_game_scene: MaterialManager::load_loading_new_game_scene_material( asset_server ),
        }
    }


    

    fn load_main_menu_scene_material( asset_server: &Res<AssetServer> ) -> MainMenuSceneMaterial{
        return MainMenuSceneMaterial { 
            background_image: asset_server.load( "images/main_menu_scene/background_image.png" ), 
        }
    }

    fn load_options_scene_material( asset_server: &Res<AssetServer> ) -> OptionsSceneMaterial{
        return OptionsSceneMaterial {
            background_image: asset_server.load( "images/options_scene/background_image.png" ),
            language_ru: asset_server.load( "images/options_scene/language_ru.png" ),
            language_en: asset_server.load( "images/options_scene/language_en.png" ),
        }
    }

    fn load_create_char_scene_material( asset_server: &Res<AssetServer> ) -> CreateCharSceneMaterial{
        return CreateCharSceneMaterial {
            background_image: asset_server.load( "images/create_char_scene/background_image.png" ),
        }
    }

    fn load_loading_new_game_scene_material( asset_server: &Res<AssetServer> ) -> LoadingNewGameSceneMaterial{
        return LoadingNewGameSceneMaterial { 
            background_image: asset_server.load( "images/loading_new_game_scene/background_image.png" ), 
        }
    }
}
