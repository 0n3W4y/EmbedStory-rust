use bevy::prelude::*;

use crate::resources::tilemap::tile::ground_tilemap_tile::CoverType;

#[derive( Debug, Clone )]
pub struct CoverTileMaterial{
    pub grass: Vec<Handle<Image>>,
    pub sand: Vec<Handle<Image>>,
    pub snow: Vec<Handle<Image>>,
    pub shallow: Vec<Handle<Image>>,
    pub water: Vec<Handle<Image>>,
    pub ice: Vec<Handle<Image>>,
    pub wooden_floor: Vec<Handle<Image>>,
    pub rocky_road: Vec<Handle<Image>>,
}

impl CoverTileMaterial{
    pub fn get_indexes( &self, cover_type: &CoverType ) -> usize{
        match cover_type{
            CoverType::Grass => { self.grass.len() },
            CoverType::Ice => { self.ice.len() },
            CoverType::None => { 0 },
            CoverType::RockyRoad => { self.rocky_road.len() },
            CoverType::Sand => { self.sand.len() },
            CoverType::Shallow => { self.shallow.len() },
            CoverType::Snow => { self.snow.len() },
            CoverType::Water => { self.water.len() },
            CoverType::WoodenFloor => { self.wooden_floor.len() },
        }
    }

    pub fn get_image( &self, cover_type: &CoverType, index: usize ) -> &Handle<Image>{
        match cover_type {
            CoverType::Grass => { &self.grass[ index ] },
            CoverType::Ice => { &self.ice[ index ]},
            CoverType::None => { panic!(" Can not give a none image for CoverType::None" ) },
            CoverType::RockyRoad => { &self.rocky_road[ index ]},
            CoverType::Sand => { &self.sand[ index ]},
            CoverType::Shallow => { &self.shallow[ index ]},
            CoverType::Snow => { &self.snow[ index ]},
            CoverType::Water => { &self.water[ index ]},
            CoverType::WoodenFloor => { &self.wooden_floor[ index ]},
        }
    }
}

#[derive( Debug, Clone )]
pub struct GroundTileMaterial{
    pub earth: Handle<Image>,
    pub dry_earth: Handle<Image>,
    pub dirt: Handle<Image>,
    pub rock: Handle<Image>,
    pub rock_environment: Handle<Image>,
}


#[derive( Debug, Clone )]
pub struct GameGroundSceneMaterial{
    pub ground_tile: GroundTileMaterial,
    pub cover_tile: CoverTileMaterial,
}

impl GameGroundSceneMaterial{
    pub fn load_ground_scene_material( asset_server: &Res<AssetServer> ) -> Self{
        return GameGroundSceneMaterial { 
            ground_tile: GameGroundSceneMaterial::load_ground_tile_material( asset_server ), 
            cover_tile: GameGroundSceneMaterial::load_cover_tile_material( asset_server ), 
        };
    }

    fn load_ground_tile_material( asset_server: &Res<AssetServer> ) -> GroundTileMaterial{
        return GroundTileMaterial{ 
            earth: asset_server.load( "textures/tiles/ground/earth_01.png" ),
            dry_earth: asset_server.load( "textures/tiles/ground/dryearth_01.png"),
            dirt: asset_server.load( "textures/tiles/ground/dirt_01.png" ),
            rock: asset_server.load( "textures/tiles/ground/rock_01.png" ),
            rock_environment: asset_server.load( "textures/tiles/ground/rock_environment_01.png" ),
        }
    }

    fn load_cover_tile_material( asset_server: &Res<AssetServer> ) -> CoverTileMaterial{
        let mut grass = vec![];
        grass.push( asset_server.load( "textures/tiles/cover/grass_00.png" ));
        grass.push( asset_server.load( "textures/tiles/cover/grass_01.png" ));
        grass.push( asset_server.load( "textures/tiles/cover/grass_02.png" ));
        grass.push( asset_server.load( "textures/tiles/cover/grass_03.png" ));
        grass.push( asset_server.load( "textures/tiles/cover/grass_04.png" ));

        let mut sand = vec![];
        sand.push( asset_server.load( "texture/tiles/cover/sand_00.png" ));
        sand.push( asset_server.load( "texture/tiles/cover/sand_01.png" ));
        sand.push( asset_server.load( "texture/tiles/cover/sand_02.png" ));
        sand.push( asset_server.load( "texture/tiles/cover/sand_03.png" ));
        sand.push( asset_server.load( "texture/tiles/cover/sand_04.png" ));

        let mut snow = vec![];
        snow.push( asset_server.load( "textures/tiles/cover/snow_00.png" ));

        let mut shallow = vec![];
        shallow.push( asset_server.load( "textures/tiles/cover/shallow_00.png" ));

        let mut water = vec![];
        water.push( asset_server.load( "textures/tiles/cover/water_00.png" ));

        let mut ice = vec![];
        ice.push( asset_server.load( "textures/tiles/cover/ice_00.png" ));

        let mut wooden_floor = vec![];
        wooden_floor.push( asset_server.load( "textures/tiles/cover/wooden_floor_00.png" ));

        let mut rocky_road = vec![];
        rocky_road.push( asset_server.load( "textures/tiles/cover/rocky_road_00.png" ));

        return CoverTileMaterial{
            grass,
            sand,
            snow,
            shallow,
            water,
            ice,
            wooden_floor,
            rocky_road,
        }
    }
}