use bevy::prelude::*;

#[derive( Clone, Default, Reflect )]
pub struct GroundTiles{
    earth: Handle<Image>,
    dry_earth: Handle<Image>,
    dirt: Handle<Image>,
    rock: Handle<Image>,
    sandrock: Handle<Image>,
    rock_envirounment: Handle<Image>,
    sandrock_envirounment: Handle<Image>,
}

#[derive( Default , Reflect )]
pub struct TextureManager {
    pub ground_tile_textures: GroundTiles,
}

pub fn new( assest_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>> ) -> TextureManager {
    return TextureManager { 
        ground_tile_textures: load_ground_tile_textures( assest_server, texture_atlases ),
    }
}


fn load_ground_tile_textures( 
    asset_server: Res<AssetServer>, 
    mut texture_atlases: ResMut<Assets<TextureAtlas>> 
) -> GroundTiles {
    let texture_eart_handle: Handle<Image> = asset_server.load( "textures/tiles/earth_01.png" );
    let texture_dryearth_handle: Handle<Image> = asset_server.load( "textures/tiles/dryearth_01.png");
    let texture_rock_handle: Handle<Image> = asset_server.load( "textures/tiles/rock_01.png" );
    let texture_dirt_handle: Handle<Image> = asset_server.load( "textures/tiles/rock_01.png" ); // change
    let texture_sandrock_handle: Handle<Image> = asset_server.load( "textures/tiles/rock_01.png" ); // change
    let texture_rock_envirounment_handle: Handle<Image> = asset_server.load( "textures/tiles/rock_01.png" ); //change
    let texture_sandrock_envirounment_handle: Handle<Image> = asset_server.load( "textures/tiles/rock_01.png" ); //change 
    return GroundTiles{ 
        earth: texture_eart_handle,
        dry_earth: texture_dryearth_handle,
        dirt: texture_dirt_handle,
        rock: texture_rock_handle,
        sandrock: texture_sandrock_handle,
        rock_envirounment: texture_rock_envirounment_handle,
        sandrock_envirounment: texture_sandrock_envirounment_handle,
    }
}