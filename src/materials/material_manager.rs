use bevy::prelude::*;

use crate::materials::ground_tile_material::GroundTileMaterial;
use crate::materials::cover_tile_material::CoverTileMaterial;
use crate::materials::main_menu_scene_material::MainMenuSceneMaterial;
use crate::materials::options_scene_material::OptionsSceneMaterial;


#[derive( Debug, Clone )]
pub struct MaterialManager {
    pub main_menu_scene_material: MainMenuSceneMaterial,
    pub option_scene_material: OptionsSceneMaterial,
    pub ground_tile_material: GroundTileMaterial,
    pub cover_tile_material: CoverTileMaterial,
    //pub character_texture: Characters,
    //pub object_texture: Objects,
    //pub stuff_texture: Stuffs,
    //pub effect_texture: Effects,
}

impl MaterialManager{

    pub fn new( asset_server: &Res<AssetServer> ) -> Self {
        return MaterialManager { 
            main_menu_scene_material: MaterialManager::load_main_menu_scene_material( asset_server ),
            option_scene_material: MaterialManager::load_options_scene_material( asset_server ),
            ground_tile_material: MaterialManager::load_ground_tile_material( asset_server ),
            cover_tile_material: MaterialManager::load_cover_tile_material( asset_server ),
        }
    }


    fn load_ground_tile_material( asset_server: &Res<AssetServer> ) -> GroundTileMaterial{
        return GroundTileMaterial{ 
            earth: asset_server.load( "textures/tiles/ground/earth_01.png" ),
            dry_earth: asset_server.load( "textures/tiles/ground/dryearth_01.png"),
            dirt: asset_server.load( "textures/tiles/ground/rock_01.png" ), //change
            rock: asset_server.load( "textures/tiles/ground/rock_01.png" ), //cnahge
            rock_envirounment: asset_server.load( "textures/tiles/ground/rock_01.png" ),//change
        }
    }

    fn load_cover_tile_material( asset_server: &Res<AssetServer> ) -> CoverTileMaterial{
        return CoverTileMaterial{
            grass: asset_server.load( "" ),
            sand: asset_server.load( "" ),
            snow: asset_server.load( "" ),
            shallow: asset_server.load( "" ),
            water: asset_server.load( "" ),
            ice: asset_server.load( "" ),
            wooden_floor: asset_server.load( "" ),
        }
    }

    fn load_main_menu_scene_material( asset_server: &Res<AssetServer> ) -> MainMenuSceneMaterial{
        return MainMenuSceneMaterial { 
            main_menu_background_image: asset_server.load( "images/main_menu_scene/main_menu_background.png" ), 
        }
    }

    fn load_options_scene_material( asset_server: &Res<AssetServer> ) -> OptionsSceneMaterial{
        return OptionsSceneMaterial {
            sound_off: asset_server.load( "images/options_scene/sound_off.png" ),
            sound_on: asset_server.load( "images/options_scene/sound_on.png" ),
            sound_hovered: asset_server.load( "images/options_scene/sound_hovered.png" ),
            music_off: asset_server.load( "images/options_scene/music_off.png" ),
            music_on: asset_server.load( "images/options_scene/music_on.png" ),
            music_hovered: asset_server.load( "images/options_scene/music_hovered.png" ),
            language_ru: asset_server.load( "images/options_scene/language_ru.png" ),
            language_en: asset_server.load( "images/options_scene/language_en.png" ),
        }
    }
}