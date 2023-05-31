use bevy::prelude::*;

use crate::config::TILE_SIZE;
use crate::{
    resources::scene_data::objects::{
        charactor::{GenderType, RaceType},
        thing::ThingType,
    },
    scenes::game_scenes::tilemap::tile::{CoverType, GroundType},
};

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
            _ => {
                println!("Can't get '{:?}', there is no option for it", cover_type);
                self.flowers_atlas.clone_weak()
            }
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
            _ => {
                println!("Can't get '{:?}', there is no option for it", cover_type);
                0
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThingMaterial {
    pub rock_atlas: Handle<TextureAtlas>,
    pub tree_atlas: Handle<TextureAtlas>,
    pub fertile_tree_atlas: Handle<TextureAtlas>,
    pub bush_atlas: Handle<TextureAtlas>,
    pub fertile_bush_atlas: Handle<TextureAtlas>,
    pub boulder_atlas: Handle<TextureAtlas>,
    pub log_atlas: Handle<TextureAtlas>,
    pub copper_ore_atlas: Handle<TextureAtlas>,
    pub iron_ore_atlas: Handle<TextureAtlas>,
    pub wooden_wall_atlas: Handle<TextureAtlas>,
    pub stone_wall_atlas: Handle<TextureAtlas>,
    pub iron_wall_atlas: Handle<TextureAtlas>,
    pub steel_wall_atlas: Handle<TextureAtlas>,
    pub wooden_door_atlas: Handle<TextureAtlas>,
    pub reinforced_wooden_door_atlas: Handle<TextureAtlas>,
    pub iron_door_atlas: Handle<TextureAtlas>,
    pub reinforced_iron_door_atlas: Handle<TextureAtlas>,
    pub steel_door_atlas: Handle<TextureAtlas>,
    pub reinforced_steel_door_atlas: Handle<TextureAtlas>,
}

impl ThingMaterial {
    pub fn get_atlas(&self, thing_type: &ThingType) -> Handle<TextureAtlas> {
        return match *thing_type {
            ThingType::Boulder => self.boulder_atlas.clone_weak(),
            ThingType::Bush => self.bush_atlas.clone_weak(),
            ThingType::CopperOre => self.copper_ore_atlas.clone_weak(),
            ThingType::FertileBush => self.fertile_bush_atlas.clone_weak(),
            ThingType::FertileTree => self.fertile_tree_atlas.clone_weak(),
            ThingType::IronDoor => self.iron_door_atlas.clone_weak(),
            ThingType::IronOre => self.iron_ore_atlas.clone_weak(),
            ThingType::IronWall => self.iron_wall_atlas.clone_weak(),
            ThingType::Log => self.log_atlas.clone_weak(),
            ThingType::ReinforcedIronDoor => self.reinforced_iron_door_atlas.clone_weak(),
            ThingType::ReinforcedSteelDoor => self.reinforced_steel_door_atlas.clone_weak(),
            ThingType::ReinforcedWoodenDoor => self.reinforced_wooden_door_atlas.clone_weak(),
            ThingType::Rock => self.rock_atlas.clone_weak(),
            ThingType::SteelDoor => self.steel_door_atlas.clone_weak(),
            ThingType::SteelWall => self.steel_wall_atlas.clone_weak(),
            ThingType::StoneWall => self.stone_wall_atlas.clone_weak(),
            ThingType::Tree => self.tree_atlas.clone_weak(),
            ThingType::WoodenDoor => self.wooden_door_atlas.clone_weak(),
            ThingType::WoodenWall => self.wooden_wall_atlas.clone_weak(),
        };
    }

    pub fn get_atlas_indexes(&self, thing_type: &ThingType) -> usize {
        return match *thing_type {
            ThingType::Boulder => 2,
            ThingType::Bush => 2,
            ThingType::CopperOre => 37,
            ThingType::FertileBush => 2,
            ThingType::FertileTree => 2,
            ThingType::IronDoor => 1,
            ThingType::IronOre => 37,
            ThingType::IronWall => 1,
            ThingType::Log => 2,
            ThingType::ReinforcedIronDoor => 1,
            ThingType::ReinforcedSteelDoor => 1,
            ThingType::ReinforcedWoodenDoor => 1,
            ThingType::Rock => 37,
            ThingType::SteelDoor => 1,
            ThingType::SteelWall => 37,
            ThingType::StoneWall => 37,
            ThingType::Tree => 2,
            ThingType::WoodenDoor => 1,
            ThingType::WoodenWall => 37,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CharactorsMaterial {
    human_female_atlas: Handle<TextureAtlas>,
    human_male_atlas: Handle<TextureAtlas>,
}

impl CharactorsMaterial {
    pub fn get_atlas(
        &self,
        charactor_racetype: &RaceType,
        gender: &GenderType,
    ) -> Handle<TextureAtlas> {
        match *charactor_racetype {
            RaceType::Human => match *gender {
                GenderType::Male => self.human_male_atlas.clone_weak(),
                GenderType::Female => self.human_female_atlas.clone_weak(),
            },
            _ => match *gender {
                GenderType::Male => self.human_male_atlas.clone_weak(),
                GenderType::Female => self.human_female_atlas.clone_weak(),
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameSceneMaterial {
    pub tile: TileMaterial,
    pub things: ThingMaterial,
    pub charactors: CharactorsMaterial,
}

impl GameSceneMaterial {
    pub fn load_ground_scene_material(
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        return GameSceneMaterial {
            tile: GameSceneMaterial::load_tile_material(asset_server, texture_atlases),
            things: GameSceneMaterial::load_things_material(asset_server, texture_atlases),
            charactors: GameSceneMaterial::load_charactors_material(asset_server, texture_atlases),
        };
    }

    fn load_tile_material(
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
        );
        let clay_texture_atlas = TextureAtlas::from_grid(
            clay_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let dirt_texture_atlas = TextureAtlas::from_grid(
            dirt_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let dry_earth_texture_atlas = TextureAtlas::from_grid(
            clay_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let rock_texture_atlas = TextureAtlas::from_grid(
            rock_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let none_texture_atlas = TextureAtlas::from_grid(
            none_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            1,
            1,
        );
        let grass_texture_atlas = TextureAtlas::from_grid(
            grass_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let ice_texture_atlas = TextureAtlas::from_grid(
            ice_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let flowers_texture_atlas = TextureAtlas::from_grid(
            flowers_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let sand_texture_atlas = TextureAtlas::from_grid(
            sand_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let shallow_texture_atlas = TextureAtlas::from_grid(
            shallow_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let snow_texture_atlas = TextureAtlas::from_grid(
            snow_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let water_texture_atlas = TextureAtlas::from_grid(
            water_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let wooden_floor_texture_atlas = TextureAtlas::from_grid(
            wooden_floor_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let rocky_road_texture_atlas = TextureAtlas::from_grid(
            rocky_road_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
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

    fn load_things_material(
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> ThingMaterial {
        let rock_texture_handle: Handle<Image> =
            asset_server.load("textures/things/rock/rock_atlas.png");
        let tree_texture_handle: Handle<Image> =
            asset_server.load("textures/things/tree/tree_atlas.png");
        let fertile_bush_texture_handle: Handle<Image> =
            asset_server.load("textures/things/bush/fertile_bush_00.png");
        let fertile_tree_texture_handle: Handle<Image> =
            asset_server.load("textures/things/tree/fertile_tree_00.png");
        let boulder_texture_handle: Handle<Image> =
            asset_server.load("textures/things/boulder/boulder_00.png");
        let bush_texture_handle: Handle<Image> =
            asset_server.load("textures/things/bush/bush_00.png");
        let log_texture_handle: Handle<Image> = asset_server.load("textures/things/log/log_00.png");
        let copper_ore_texture_handle: Handle<Image> =
            asset_server.load("textures/things/ore/copper_ore_00.png");
        let iron_ore_texture_handle: Handle<Image> =
            asset_server.load("textures/things/ore/iron_ore_00.png");
        let iron_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/iron_door_00.png");
        let wooden_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/wooden_door_00.png");
        let wooden_wall_texture_handle: Handle<Image> =
            asset_server.load("textures/things/wall/wooden_wall_00.png");
        let stone_wall_texture_handle: Handle<Image> =
            asset_server.load("textures/things/wall/stone_wall_00.png");
        let steel_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/steel_door_00.png");
        let steel_wall_texture_handle: Handle<Image> =
            asset_server.load("textures/things/wall/steel_wall_00.png");
        let iron_wall_texture_handle: Handle<Image> =
            asset_server.load("textures/things/wall/iron_wall_00.png");
        let reinforced_iron_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/reinforced_iron_door_00.png");
        let reinforced_steel_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/reinforced_steel_door_00.png");
        let reinforced_wooden_door_texture_handle: Handle<Image> =
            asset_server.load("textures/things/door/reinforced_wooden_door_00.png");

        let rock_texture_atlas = TextureAtlas::from_grid(
            rock_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let tree_texture_atlas = TextureAtlas::from_grid(
            tree_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let fertile_bush_texture_atlas = TextureAtlas::from_grid(
            fertile_bush_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let fertile_tree_texture_atlas = TextureAtlas::from_grid(
            fertile_tree_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let boulder_texture_atlas = TextureAtlas::from_grid(
            boulder_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let bush_texture_atlas = TextureAtlas::from_grid(
            bush_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let log_texture_atlas = TextureAtlas::from_grid(
            log_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let copper_ore_texture_atlas = TextureAtlas::from_grid(
            copper_ore_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        ); //37
        let iron_ore_texture_atlas = TextureAtlas::from_grid(
            iron_ore_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        ); //37
        let iron_door_texture_atlas = TextureAtlas::from_grid(
            iron_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let wooden_door_texture_atlas = TextureAtlas::from_grid(
            wooden_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let wooden_wall_texture_atlas = TextureAtlas::from_grid(
            wooden_wall_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let stone_wall_texture_atlas = TextureAtlas::from_grid(
            stone_wall_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let steel_wall_texture_atlas = TextureAtlas::from_grid(
            steel_wall_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let steel_door_texture_atlas = TextureAtlas::from_grid(
            steel_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let iron_wall_texture_atlas = TextureAtlas::from_grid(
            iron_wall_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let reinforced_iron_door_texture_atlas = TextureAtlas::from_grid(
            reinforced_iron_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let reinforced_steel_door_texture_atlas = TextureAtlas::from_grid(
            reinforced_steel_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );
        let reinforced_wooden_door_texture_atlas = TextureAtlas::from_grid(
            reinforced_wooden_door_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            6,
            1,
        );

        let rock_atlas = texture_atlases.add(rock_texture_atlas);
        let tree_atlas = texture_atlases.add(tree_texture_atlas);
        let fertile_tree_atlas = texture_atlases.add(fertile_bush_texture_atlas);
        let bush_atlas = texture_atlases.add(fertile_tree_texture_atlas);
        let fertile_bush_atlas = texture_atlases.add(boulder_texture_atlas);
        let boulder_atlas = texture_atlases.add(bush_texture_atlas);
        let log_atlas = texture_atlases.add(log_texture_atlas);
        let copper_ore_atlas = texture_atlases.add(copper_ore_texture_atlas);
        let iron_ore_atlas = texture_atlases.add(iron_ore_texture_atlas);
        let wooden_wall_atlas = texture_atlases.add(wooden_wall_texture_atlas);
        let stone_wall_atlas = texture_atlases.add(stone_wall_texture_atlas);
        let iron_wall_atlas = texture_atlases.add(iron_wall_texture_atlas);
        let steel_wall_atlas = texture_atlases.add(steel_wall_texture_atlas);
        let wooden_door_atlas = texture_atlases.add(wooden_door_texture_atlas);
        let reinforced_wooden_door_atlas =
            texture_atlases.add(reinforced_wooden_door_texture_atlas);
        let iron_door_atlas = texture_atlases.add(iron_door_texture_atlas);
        let reinforced_iron_door_atlas = texture_atlases.add(reinforced_iron_door_texture_atlas);
        let steel_door_atlas = texture_atlases.add(steel_door_texture_atlas);
        let reinforced_steel_door_atlas = texture_atlases.add(reinforced_steel_door_texture_atlas);

        return ThingMaterial {
            rock_atlas,
            tree_atlas,
            fertile_tree_atlas,
            bush_atlas,
            fertile_bush_atlas,
            boulder_atlas,
            log_atlas,
            copper_ore_atlas,
            iron_ore_atlas,
            wooden_wall_atlas,
            stone_wall_atlas,
            iron_wall_atlas,
            steel_wall_atlas,
            wooden_door_atlas,
            reinforced_wooden_door_atlas,
            iron_door_atlas,
            reinforced_iron_door_atlas,
            steel_door_atlas,
            reinforced_steel_door_atlas,
        };
    }

    pub fn load_charactors_material(
        asset_server: &Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    ) -> CharactorsMaterial {
        let human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactor/human/male.png");
        let human_female_texture_handle: Handle<Image> = asset_server.load("textures/charactor/human/female.png");

        let human_male_texture_atlas = TextureAtlas::from_grid(
            human_male_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            4,
            1,
        );
        let human_female_texture_atlas = TextureAtlas::from_grid(
            human_female_texture_handle,
            Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
            4,
            1,
        );

        let human_male_atlas = texture_atlases.add(human_male_texture_atlas);
        let human_female_atlas = texture_atlases.add(human_female_texture_atlas);

        CharactorsMaterial {
            human_male_atlas,
            human_female_atlas,
        }
    }
}
