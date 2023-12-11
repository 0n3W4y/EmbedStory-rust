use bevy::prelude::*;

use crate::{resources::scene_data::thing::ThingType, config::TILE_SIZE};

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
    pub dungeon_enter_atlas: Handle<TextureAtlas>,
    pub dungeon_exit_atlas: Handle<TextureAtlas>,
}

impl ThingMaterial {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        load_things_material(asset_server, texture_atlases)
    }

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
            ThingType::DungeonEnter(_) => self.dungeon_enter_atlas.clone_weak(),
            ThingType::DungeonExit(_) => self.dungeon_exit_atlas.clone_weak(),
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
            ThingType::DungeonEnter(_) => 2,
            ThingType::DungeonExit(_) => 2,
        };
    }
}

fn load_things_material(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
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
    let dungeon_enter_texture_handle: Handle<Image> =
        asset_server.load("textures/things/dungeon/dungeon_enter00.png");
    let dungeon_exit_texture_handle: Handle<Image> =
        asset_server.load("textures/things/dungeon/dungeon_exit00.png");

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
    let dungeon_enter_texture_atlas = TextureAtlas::from_grid(
        dungeon_enter_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        2,
        1,
    );
    let dungeon_exit_texture_atlas = TextureAtlas::from_grid(
        dungeon_exit_texture_handle,
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
    let dungeon_enter_atlas = texture_atlases.add(dungeon_enter_texture_atlas);
    let dungeon_exit_atlas = texture_atlases.add(dungeon_exit_texture_atlas);

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
        dungeon_enter_atlas,
        dungeon_exit_atlas,
    };
}