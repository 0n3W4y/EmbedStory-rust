use bevy::prelude::*;

use crate::materials::ground_tile::GroundTiles;
use crate::materials::cover_tile::CoverTiles;


#[derive( Debug, Clone )]
pub struct MaterialManager {
    pub ground_tile_textures: GroundTiles,
    pub cover_tile_texture: CoverTiles,
    //pub character_texture: Characters,
    //pub object_texture: Objects,
    //pub stuff_texture: Stuffs,
    //pub effect_texture: Effects,
}

impl MaterialManager{

    pub fn new( asset_server: &Res<AssetServer> ) -> Self {
        return MaterialManager { 
            ground_tile_textures: MaterialManager::load_ground_tile_textures( asset_server ),
            cover_tile_texture: MaterialManager::load_cover_tile_texture( asset_server ),
        }
    }


    fn load_ground_tile_textures( asset_server: &Res<AssetServer> ) -> GroundTiles {
        return GroundTiles{ 
            earth: asset_server.load( "textures/tiles/ground/earth_01.png" ),
            dry_earth: asset_server.load( "textures/tiles/ground/dryearth_01.png"),
            dirt: asset_server.load( "textures/tiles/ground/rock_01.png" ), //change
            rock: asset_server.load( "textures/tiles/ground/rock_01.png" ), //cnahge
            rock_envirounment: asset_server.load( "textures/tiles/ground/rock_01.png" ),//change
        }
    }

    fn load_cover_tile_texture( asset_server: &Res<AssetServer> ) -> CoverTiles{
        return CoverTiles{
            grass: asset_server.load( "" ),
            sand: asset_server.load( "" ),
            snow: asset_server.load( "" ),
            shallow: asset_server.load( "" ),
            water: asset_server.load( "" ),
            ice: asset_server.load( "" ),
            wooden_floor: asset_server.load( "" ),
        }
    }
}