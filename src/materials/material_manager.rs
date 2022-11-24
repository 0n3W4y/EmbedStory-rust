use bevy::prelude::*;

use crate::materials::ground_tile_material::GroundTileMaterial;
use crate::materials::cover_tile_material::CoverTileMaterial;
use crate::materials::main_menu_scene_material::MainMenuSceneMaterial;
use crate::materials::options_scene_material::OptionsSceneMaterial;


#[derive( Debug, Clone )]
pub struct MaterialManager {
    pub main_menu_scene_material: MainMenuSceneMaterial,
    pub options_scene_material: OptionsSceneMaterial,
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
            options_scene_material: MaterialManager::load_options_scene_material( asset_server ),
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
            grass: asset_server.load( "textures/tiles/cover/grass_01.png" ),
            sand: asset_server.load( "texture/tiles/cover/sand_01.png" ),
            snow: asset_server.load( "textures/tiles/cover/snow_01.png" ),
            shallow: asset_server.load( "textures/tiles/cover/shallow_01.png" ),
            water: asset_server.load( "textures/tiles/cover/water_01.png" ),
            ice: asset_server.load( "textures/tiles/cover/ice_01.png" ),
            wooden_floor: asset_server.load( "textures/tiles/cover/wooden_floor_01.png" ),
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