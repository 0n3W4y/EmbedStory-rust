use bevy::prelude::*;

use crate::{
    resources::scene_data::objects::thing::ThingType,
    scenes::game_scenes::tilemap::tile::{CoverType, GroundType},
};

#[derive(Debug, Clone)]
pub struct CoverTileMaterial {
    pub grass: Vec<Handle<Image>>,
    pub sand: Vec<Handle<Image>>,
    pub flowers: Vec<Handle<Image>>,
    pub snow: Vec<Handle<Image>>,
    pub shallow: Vec<Handle<Image>>,
    pub water: Vec<Handle<Image>>,
    pub ice: Vec<Handle<Image>>,
    pub wooden_floor: Vec<Handle<Image>>,
    pub rocky_road: Vec<Handle<Image>>,
}

impl CoverTileMaterial {
    pub fn get_indexes(&self, cover_type: &CoverType) -> usize {
        match cover_type {
            CoverType::Grass => self.grass.len(),
            CoverType::Ice => self.ice.len(),
            CoverType::Flowers => self.flowers.len(),
            CoverType::None => 0,
            CoverType::RockyRoad => self.rocky_road.len(),
            CoverType::Sand => self.sand.len(),
            CoverType::Shallow => self.shallow.len(),
            CoverType::Snow => self.snow.len(),
            CoverType::Water => self.water.len(),
            CoverType::WoodenFloor => self.wooden_floor.len(),
        }
    }

    pub fn get_image(&self, cover_type: &CoverType, index: usize) -> &Handle<Image> {
        match cover_type {
            CoverType::Grass => &self.grass[index],
            CoverType::Ice => &self.ice[index],
            CoverType::Flowers => &self.flowers[index],
            CoverType::None => panic!(" Can not give a none image for CoverType::None"),
            CoverType::RockyRoad => &self.rocky_road[index],
            CoverType::Sand => &self.sand[index],
            CoverType::Shallow => &self.shallow[index],
            CoverType::Snow => &self.snow[index],
            CoverType::Water => &self.water[index],
            CoverType::WoodenFloor => &self.wooden_floor[index],
        }
    }
}

#[derive(Debug, Clone)]
pub struct GroundTileMaterial {
    pub earth: Handle<Image>,
    pub dry_earth: Handle<Image>,
    pub dirt: Handle<Image>,
    pub rock: Handle<Image>,
    pub rock_environment: Handle<Image>,
    pub clay: Handle<Image>,
}

impl GroundTileMaterial {
    pub fn get_image(&self, ground_type: &GroundType) -> &Handle<Image> {
        match ground_type {
            GroundType::Earth => &self.earth,
            GroundType::Clay => &self.clay,
            GroundType::Dirt => &self.dirt,
            GroundType::DryEarth => &self.dry_earth,
            GroundType::Rock => &self.rock,
            GroundType::RockEnvironment => &self.rock_environment,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ThingMaterial {
    pub rock: Vec<Handle<Image>>,
    pub tree: Vec<Handle<Image>>,
    pub fertile_tree: Vec<Handle<Image>>,
    pub bush: Vec<Handle<Image>>,
    pub fertile_bush: Vec<Handle<Image>>,
    pub boulder: Vec<Handle<Image>>,
    pub log: Vec<Handle<Image>>,
    pub copper_ore: Vec<Handle<Image>>,
    pub iron_ore: Vec<Handle<Image>>,
    pub wooden_wall: Vec<Handle<Image>>,
    pub stone_wall: Vec<Handle<Image>>,
    pub iron_wall: Vec<Handle<Image>>,
    pub steel_wall: Vec<Handle<Image>>,
    pub wooden_door: Vec<Handle<Image>>,
    pub reinforced_wooden_door: Vec<Handle<Image>>,
    pub iron_door: Vec<Handle<Image>>,
    pub reinforced_iron_door: Vec<Handle<Image>>,
    pub steel_door: Vec<Handle<Image>>,
    pub reinforced_steel_door: Vec<Handle<Image>>,
}

impl ThingMaterial {
    pub fn get_image(&self, thing_type: &ThingType, index: usize) -> Handle<Image> {
        return match *thing_type {
            ThingType::Boulder => self.boulder[index].clone(),
            ThingType::Bush => self.bush[index].clone(),
            ThingType::CopperOre => self.copper_ore[index].clone(),
            ThingType::FertileBush => self.fertile_bush[index].clone(),
            ThingType::FertileTree => self.fertile_tree[index].clone(),
            ThingType::IronDoor => self.iron_door[index].clone(),
            ThingType::IronOre => self.iron_ore[index].clone(),
            ThingType::IronWall => self.iron_wall[index].clone(),
            ThingType::Log => self.log[index].clone(),
            ThingType::ReinforcedIronDoor => self.reinforced_iron_door[index].clone(),
            ThingType::ReinforcedSteelDoor => self.reinforced_steel_door[index].clone(),
            ThingType::ReinforcedWoodenDoor => self.reinforced_wooden_door[index].clone(),
            ThingType::Rock => self.rock[index].clone(),
            ThingType::SteelDoor => self.steel_door[index].clone(),
            ThingType::SteelWall => self.steel_wall[index].clone(),
            ThingType::StoneWall => self.stone_wall[index].clone(),
            ThingType::Tree => self.tree[index].clone(),
            ThingType::WoodenDoor => self.wooden_door[index].clone(),
            ThingType::WoodenWall => self.wooden_wall[index].clone(),
        };
    }

    pub fn get_indexes(&self, thing_type: &ThingType) -> usize {
        return match *thing_type {
            ThingType::Boulder => self.boulder.len(),
            ThingType::Bush => self.bush.len(),
            ThingType::CopperOre => self.copper_ore.len(),
            ThingType::FertileBush => self.fertile_bush.len(),
            ThingType::FertileTree => self.fertile_tree.len(),
            ThingType::IronDoor => self.iron_door.len(),
            ThingType::IronOre => self.iron_ore.len(),
            ThingType::IronWall => self.iron_wall.len(),
            ThingType::Log => self.log.len(),
            ThingType::ReinforcedIronDoor => self.reinforced_iron_door.len(),
            ThingType::ReinforcedSteelDoor => self.reinforced_steel_door.len(),
            ThingType::ReinforcedWoodenDoor => self.reinforced_wooden_door.len(),
            ThingType::Rock => self.rock.len(),
            ThingType::SteelDoor => self.steel_door.len(),
            ThingType::SteelWall => self.steel_wall.len(),
            ThingType::StoneWall => self.stone_wall.len(),
            ThingType::Tree => self.tree.len(),
            ThingType::WoodenDoor => self.wooden_door.len(),
            ThingType::WoodenWall => self.wooden_wall.len(),
        };
    }
}

#[derive(Debug, Clone)]
pub struct GameSceneMaterial {
    pub ground_tile: GroundTileMaterial,
    pub cover_tile: CoverTileMaterial,
    pub things: ThingMaterial,
}

impl GameSceneMaterial {
    pub fn load_ground_scene_material(asset_server: &Res<AssetServer>) -> Self {
        return GameSceneMaterial {
            ground_tile: GameSceneMaterial::load_ground_tile_material(asset_server),
            cover_tile: GameSceneMaterial::load_cover_tile_material(asset_server),
            things: GameSceneMaterial::load_things_material(asset_server),
        };
    }

    fn load_ground_tile_material(asset_server: &Res<AssetServer>) -> GroundTileMaterial {
        return GroundTileMaterial {
            earth: asset_server.load("textures/tiles/ground/earth_01.png"),
            dry_earth: asset_server.load("textures/tiles/ground/dryearth_01.png"),
            dirt: asset_server.load("textures/tiles/ground/dirt_01.png"),
            rock: asset_server.load("textures/tiles/ground/rock_01.png"),
            rock_environment: asset_server.load("textures/tiles/ground/rock_environment_01.png"),
            clay: asset_server.load("textures/tiles/ground/clay_01.png"),
        };
    }

    fn load_cover_tile_material(asset_server: &Res<AssetServer>) -> CoverTileMaterial {
        let mut grass = vec![];
        grass.push(asset_server.load("textures/tiles/cover/grass_00.png"));
        grass.push(asset_server.load("textures/tiles/cover/grass_01.png"));
        //grass.push( asset_server.load( "textures/tiles/cover/grass_02.png" ));
        //grass.push( asset_server.load( "textures/tiles/cover/grass_03.png" ));
        //grass.push( asset_server.load( "textures/tiles/cover/grass_04.png" ));

        let mut flowers: Vec<Handle<Image>> = vec![];
        flowers.push(asset_server.load("textures/tiles/cover/grass_02.png"));

        let mut sand = vec![];
        sand.push(asset_server.load("texture/tiles/cover/sand_00.png"));
        //sand.push( asset_server.load( "texture/tiles/cover/sand_01.png" ));
        //sand.push( asset_server.load( "texture/tiles/cover/sand_02.png" ));
        //sand.push( asset_server.load( "texture/tiles/cover/sand_03.png" ));
        //sand.push( asset_server.load( "texture/tiles/cover/sand_04.png" ));

        let mut snow = vec![];
        snow.push(asset_server.load("textures/tiles/cover/snow_00.png"));

        let mut shallow = vec![];
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_01.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_02.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_03.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_04.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_05.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_06.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_07.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_08.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_09.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_10.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_11.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_12.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_13.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_14.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_15.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));
        shallow.push(asset_server.load("textures/tiles/cover/shallow_00.png"));

        let mut water = vec![];
        water.push(asset_server.load("textures/tiles/cover/water_00.png"));
        water.push(asset_server.load("textures/tiles/cover/water_01.png"));
        water.push(asset_server.load("textures/tiles/cover/water_02.png"));
        water.push(asset_server.load("textures/tiles/cover/water_03.png"));
        water.push(asset_server.load("textures/tiles/cover/water_04.png"));
        water.push(asset_server.load("textures/tiles/cover/water_05.png"));
        water.push(asset_server.load("textures/tiles/cover/water_06.png"));
        water.push(asset_server.load("textures/tiles/cover/water_07.png"));
        water.push(asset_server.load("textures/tiles/cover/water_08.png"));
        water.push(asset_server.load("textures/tiles/cover/water_09.png"));
        water.push(asset_server.load("textures/tiles/cover/water_10.png"));
        water.push(asset_server.load("textures/tiles/cover/water_11.png"));
        water.push(asset_server.load("textures/tiles/cover/water_12.png"));
        water.push(asset_server.load("textures/tiles/cover/water_13.png"));
        water.push(asset_server.load("textures/tiles/cover/water_14.png"));
        water.push(asset_server.load("textures/tiles/cover/water_15.png"));
        water.push(asset_server.load("textures/tiles/cover/water_16.png"));
        water.push(asset_server.load("textures/tiles/cover/water_17.png"));
        water.push(asset_server.load("textures/tiles/cover/water_18.png"));
        water.push(asset_server.load("textures/tiles/cover/water_19.png"));
        water.push(asset_server.load("textures/tiles/cover/water_20.png"));
        water.push(asset_server.load("textures/tiles/cover/water_21.png"));
        water.push(asset_server.load("textures/tiles/cover/water_22.png"));
        water.push(asset_server.load("textures/tiles/cover/water_23.png"));
        water.push(asset_server.load("textures/tiles/cover/water_24.png"));
        water.push(asset_server.load("textures/tiles/cover/water_25.png"));
        water.push(asset_server.load("textures/tiles/cover/water_26.png"));
        water.push(asset_server.load("textures/tiles/cover/water_27.png"));
        water.push(asset_server.load("textures/tiles/cover/water_28.png"));
        water.push(asset_server.load("textures/tiles/cover/water_29.png"));
        water.push(asset_server.load("textures/tiles/cover/water_30.png"));
        water.push(asset_server.load("textures/tiles/cover/water_31.png"));
        water.push(asset_server.load("textures/tiles/cover/water_32.png"));
        water.push(asset_server.load("textures/tiles/cover/water_33.png"));
        water.push(asset_server.load("textures/tiles/cover/water_34.png"));
        water.push(asset_server.load("textures/tiles/cover/water_35.png"));
        water.push(asset_server.load("textures/tiles/cover/water_36.png"));
        water.push(asset_server.load("textures/tiles/cover/water_37.png"));

        let mut ice = vec![];
        ice.push(asset_server.load("textures/tiles/cover/ice_00.png"));
        //TODO: all 38 ice images

        let mut wooden_floor = vec![];
        wooden_floor.push(asset_server.load("textures/tiles/cover/wooden_floor_00.png"));

        let mut rocky_road = vec![];
        rocky_road.push(asset_server.load("textures/tiles/cover/rocky_road_00.png"));

        return CoverTileMaterial {
            grass,
            sand,
            flowers,
            snow,
            shallow,
            water,
            ice,
            wooden_floor,
            rocky_road,
        };
    }

    fn load_things_material(asset_server: &Res<AssetServer>) -> ThingMaterial {
        let mut rock: Vec<Handle<Image>> = vec![];
        rock.push(asset_server.load("textures/things/rock/rock_00.png"));
        rock.push(asset_server.load("textures/things/rock/rock_01.png"));
        rock.push(asset_server.load("textures/things/rock/rock_02.png"));
        rock.push(asset_server.load("textures/things/rock/rock_03.png"));
        rock.push(asset_server.load("textures/things/rock/rock_04.png"));
        rock.push(asset_server.load("textures/things/rock/rock_05.png"));
        rock.push(asset_server.load("textures/things/rock/rock_06.png"));
        rock.push(asset_server.load("textures/things/rock/rock_07.png"));
        rock.push(asset_server.load("textures/things/rock/rock_08.png"));
        rock.push(asset_server.load("textures/things/rock/rock_09.png"));
        rock.push(asset_server.load("textures/things/rock/rock_10.png"));
        rock.push(asset_server.load("textures/things/rock/rock_11.png"));
        rock.push(asset_server.load("textures/things/rock/rock_12.png"));
        rock.push(asset_server.load("textures/things/rock/rock_13.png"));
        rock.push(asset_server.load("textures/things/rock/rock_14.png"));
        rock.push(asset_server.load("textures/things/rock/rock_15.png"));
        rock.push(asset_server.load("textures/things/rock/rock_16.png"));
        rock.push(asset_server.load("textures/things/rock/rock_17.png"));
        rock.push(asset_server.load("textures/things/rock/rock_18.png"));
        rock.push(asset_server.load("textures/things/rock/rock_19.png"));
        rock.push(asset_server.load("textures/things/rock/rock_20.png"));
        rock.push(asset_server.load("textures/things/rock/rock_21.png"));
        rock.push(asset_server.load("textures/things/rock/rock_22.png"));
        rock.push(asset_server.load("textures/things/rock/rock_23.png"));
        rock.push(asset_server.load("textures/things/rock/rock_24.png"));
        rock.push(asset_server.load("textures/things/rock/rock_25.png"));
        rock.push(asset_server.load("textures/things/rock/rock_26.png"));
        rock.push(asset_server.load("textures/things/rock/rock_27.png"));
        rock.push(asset_server.load("textures/things/rock/rock_28.png"));
        rock.push(asset_server.load("textures/things/rock/rock_29.png"));
        rock.push(asset_server.load("textures/things/rock/rock_30.png"));
        rock.push(asset_server.load("textures/things/rock/rock_31.png"));
        rock.push(asset_server.load("textures/things/rock/rock_32.png"));
        rock.push(asset_server.load("textures/things/rock/rock_33.png"));
        rock.push(asset_server.load("textures/things/rock/rock_34.png"));
        rock.push(asset_server.load("textures/things/rock/rock_35.png"));
        rock.push(asset_server.load("textures/things/rock/rock_36.png"));
        rock.push(asset_server.load("textures/things/rock/rock_37.png"));

        let mut tree: Vec<Handle<Image>> = vec![];
        tree.push(asset_server.load("textures/things/tree/tree_00.png"));

        let mut fertile_bush: Vec<Handle<Image>> = vec![];
        fertile_bush.push(asset_server.load("textures/things/bush/fertile_bush_00.png"));

        let mut fertile_tree: Vec<Handle<Image>> = vec![];
        fertile_tree.push(asset_server.load("textures/things/tree/fertile_tree_00.png"));

        let mut boulder: Vec<Handle<Image>> = vec![];
        boulder.push(asset_server.load("textures/things/boulder/boulder_00.png"));

        let mut bush: Vec<Handle<Image>> = vec![];
        bush.push(asset_server.load("textures/things/bush/bush_00.png"));

        let mut log: Vec<Handle<Image>> = vec![];
        log.push(asset_server.load("textures/things/log/log_00.png"));

        let mut copper_ore: Vec<Handle<Image>> = vec![];
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_00.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_01.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_02.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_03.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_04.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_05.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_06.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_07.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_08.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_09.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_10.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_11.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_12.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_13.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_14.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_15.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_16.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_17.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_18.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_19.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_20.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_21.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_22.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_23.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_24.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_25.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_26.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_27.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_28.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_29.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_30.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_31.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_32.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_33.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_34.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_35.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_36.png"));
        copper_ore.push(asset_server.load("textures/things/ore/copper_ore_37.png"));

        let mut iron_ore: Vec<Handle<Image>> = vec![];
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_00.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_01.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_02.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_03.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_04.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_05.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_06.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_07.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_08.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_09.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_10.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_11.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_12.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_13.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_14.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_15.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_16.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_17.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_18.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_19.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_20.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_21.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_22.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_23.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_24.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_25.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_26.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_27.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_28.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_29.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_30.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_31.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_32.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_33.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_34.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_35.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_36.png"));
        iron_ore.push(asset_server.load("textures/things/ore/iron_ore_37.png"));

        let mut iron_door: Vec<Handle<Image>> = vec![];
        iron_door.push(asset_server.load("textures/things/door/iron_door_00.png"));

        let mut wooden_door: Vec<Handle<Image>> = vec![];
        wooden_door.push(asset_server.load("textures/things/door/wooden_door_00.png"));

        let mut wooden_wall: Vec<Handle<Image>> = vec![];
        wooden_wall.push(asset_server.load("textures/things/wall/wooden_wall_00.png"));

        let mut stone_wall: Vec<Handle<Image>> = vec![];
        stone_wall.push(asset_server.load("textures/things/wall/stone_wall_00.png"));

        let mut steel_door: Vec<Handle<Image>> = vec![];
        steel_door.push(asset_server.load("textures/things/door/steel_door_00.png"));

        let mut steel_wall: Vec<Handle<Image>> = vec![];
        steel_wall.push(asset_server.load("textures/things/wall/steel_wall_00.png"));

        let mut iron_wall: Vec<Handle<Image>> = vec![];
        iron_wall.push(asset_server.load("textures/things/wall/iron_wall_00.png"));

        let mut reinforced_iron_door: Vec<Handle<Image>> = vec![];
        reinforced_iron_door
            .push(asset_server.load("textures/things/door/reinforced_iron_door_00.png"));

        let mut reinforced_steel_door: Vec<Handle<Image>> = vec![];
        reinforced_steel_door
            .push(asset_server.load("textures/things/door/reinforced_steel_door_00.png"));

        let mut reinforced_wooden_door: Vec<Handle<Image>> = vec![];
        reinforced_wooden_door
            .push(asset_server.load("textures/things/door/reinforced_wooden_door_00.png"));

        return ThingMaterial {
            rock,
            tree,
            fertile_bush,
            fertile_tree,
            boulder,
            bush,
            log,
            copper_ore,
            iron_ore,
            iron_door,
            wooden_door,
            wooden_wall,
            stone_wall,
            steel_door,
            steel_wall,
            iron_wall,
            reinforced_iron_door,
            reinforced_steel_door,
            reinforced_wooden_door,
        };
    }
}
