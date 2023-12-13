use bevy::prelude::*;

use crate::{config::TILE_SIZE, scenes::game_scenes::tilemap::tile::{GroundType, CoverType}};

#[derive(Debug, Clone)]
pub struct TileMaterial {
    //ground;
    pub earth_atlas: Handle<TextureAtlas>,
    pub clay_atlas: Handle<TextureAtlas>,
    pub dirt_atlas: Handle<TextureAtlas>,
    pub dry_earth_atlas: Handle<TextureAtlas>,
    pub rock_atlas: Handle<TextureAtlas>,
    //cover;
    pub none_atlas: Handle<TextureAtlas>,
    pub grass_atlas: Handle<TextureAtlas>,
    pub ice_atlas: Handle<TextureAtlas>,
    pub flowers_atlas: Handle<TextureAtlas>,
    pub sand_atlas: Handle<TextureAtlas>,
    pub shallow_atlas: Handle<TextureAtlas>,
    pub snow_atlas: Handle<TextureAtlas>,
    pub water_atlas: Handle<TextureAtlas>,
    pub wooden_floor_atlas: Handle<TextureAtlas>,
    pub rocky_road_atlas: Handle<TextureAtlas>,
}

impl TileMaterial {

    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        load_tile_material(asset_server, texture_atlases)
    }

    pub fn get_ground_atlas(
        &self,
        ground_type: &GroundType,
    ) -> Handle<TextureAtlas>{
        match *ground_type {
            GroundType::Earth => self.earth_atlas.clone_weak(),
            GroundType::Clay => self.clay_atlas.clone_weak(),
            GroundType::Dirt => self.dirt_atlas.clone_weak(),
            GroundType::DryEarth => self.dry_earth_atlas.clone_weak(),
            GroundType::Rock => self.rock_atlas.clone_weak(),
        }
    }

    pub fn get_ground_atlas_indexes(&self, ground_type: &GroundType) -> usize {
        match *ground_type {
            GroundType::Earth => 0,
            GroundType::Clay => 0,
            GroundType::Dirt => 0,
            GroundType::DryEarth => 0,
            GroundType::Rock => 0,
        }
    }

    pub fn get_cover_atlas(
        &self,
        cover_type: &CoverType,
    ) -> Handle<TextureAtlas> {
        match *cover_type {
            CoverType::Flowers => self.flowers_atlas.clone_weak(),
            CoverType::Grass => self.grass_atlas.clone_weak(),
            CoverType::Ice => self.ice_atlas.clone_weak(),
            CoverType::RockyRoad => self.rocky_road_atlas.clone_weak(),
            CoverType::Sand => self.sand_atlas.clone_weak(),
            CoverType::Shallow => self.shallow_atlas.clone_weak(),
            CoverType::Snow => self.snow_atlas.clone_weak(),
            CoverType::Water => self.water_atlas.clone_weak(),
            CoverType::WoodenFloor => self.wooden_floor_atlas.clone_weak(),
            CoverType::None => self.none_atlas.clone_weak(),
        }
    }

    pub fn get_cover_atlas_indexes(&self, cover_type: &CoverType) -> usize {
        match *cover_type {
            CoverType::Flowers =>  2,
            CoverType::Grass =>  4,
            CoverType::Ice =>  37,
            CoverType::RockyRoad => 15,
            CoverType::Sand =>  3,
            CoverType::Shallow =>  37,
            CoverType::Snow =>  3,
            CoverType::Water => 37,
            CoverType::WoodenFloor =>  15,
            CoverType::None => 0,
        }
    }
}

fn load_tile_material(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> TileMaterial {
    let earth_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/ground/earth_atlas.png");
    let clay_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/ground/clay_atlas.png");
    let dirt_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/ground/dirt_atlas.png");
    let dry_earth_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/ground/dry_earth_atlas.png");
    let rock_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/ground/rock_atlas.png");
    let none_texture_handle: Handle<Image> = 
        asset_server.load("textures/tiles/cover/none_atlas.png");
    let grass_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/grass_atlas.png");
    let ice_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/ice_atlas.png");
    let flowers_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/flowers_atlas.png");
    let sand_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/sand_atlas.png");
    let shallow_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/shallow_atlas.png");
    let snow_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/snow_atlas.png");
    let water_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/water_atlas.png");
    let wooden_floor_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/wooden_floor_atlas.png");
    let rocky_road_texture_handle: Handle<Image> =
        asset_server.load("textures/tiles/cover/rocky_road_atlas.png");

    let earth_texture_atlas = TextureAtlas::from_grid(
        earth_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let clay_texture_atlas = TextureAtlas::from_grid(
        clay_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let dirt_texture_atlas = TextureAtlas::from_grid(
        dirt_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let dry_earth_texture_atlas = TextureAtlas::from_grid(
        dry_earth_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let rock_texture_atlas = TextureAtlas::from_grid(
        rock_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let none_texture_atlas = TextureAtlas::from_grid(
        none_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        1,
        1,
        None,
        None,
    );
    let grass_texture_atlas = TextureAtlas::from_grid(
        grass_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let ice_texture_atlas = TextureAtlas::from_grid(
        ice_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let flowers_texture_atlas = TextureAtlas::from_grid(
        flowers_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let sand_texture_atlas = TextureAtlas::from_grid(
        sand_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let shallow_texture_atlas = TextureAtlas::from_grid(
        shallow_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let snow_texture_atlas = TextureAtlas::from_grid(
        snow_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let water_texture_atlas = TextureAtlas::from_grid(
        water_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let wooden_floor_texture_atlas = TextureAtlas::from_grid(
        wooden_floor_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );
    let rocky_road_texture_atlas = TextureAtlas::from_grid(
        rocky_road_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        6,
        1,
        None,
        None,
    );

    let earth_atlas = texture_atlases.add(earth_texture_atlas);
    let clay_atlas = texture_atlases.add(clay_texture_atlas);
    let dirt_atlas = texture_atlases.add(dirt_texture_atlas);
    let dry_earth_atlas = texture_atlases.add(dry_earth_texture_atlas);
    let rock_atlas = texture_atlases.add(rock_texture_atlas);
    let none_atlas = texture_atlases.add(none_texture_atlas);
    let grass_atlas = texture_atlases.add(grass_texture_atlas);
    let ice_atlas = texture_atlases.add(ice_texture_atlas);
    let flowers_atlas = texture_atlases.add(flowers_texture_atlas);
    let sand_atlas = texture_atlases.add(sand_texture_atlas);
    let shallow_atlas = texture_atlases.add(shallow_texture_atlas);
    let snow_atlas = texture_atlases.add(snow_texture_atlas);
    let water_atlas = texture_atlases.add(water_texture_atlas);
    let wooden_floor_atlas = texture_atlases.add(wooden_floor_texture_atlas);
    let rocky_road_atlas = texture_atlases.add(rocky_road_texture_atlas);

    return TileMaterial {
        earth_atlas,
        clay_atlas,
        dirt_atlas,
        dry_earth_atlas,
        rock_atlas,
        none_atlas,
        grass_atlas,
        ice_atlas,
        flowers_atlas,
        sand_atlas,
        shallow_atlas,
        snow_atlas,
        water_atlas,
        wooden_floor_atlas,
        rocky_road_atlas,
    };
}