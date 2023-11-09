use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::scene_data::charactor::Charactor;
use crate::resources::scene_data::scene_effect::SceneEffect;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_data::thing::Thing;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_manager::SceneType;
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap;
use crate::resources::scene_data::thing;
use crate::resources::scene_data::charactor;
use crate::scenes::SceneState;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ThingsStorage {
    pub rocks_and_ores: Vec<Thing>,
    pub trees: Vec<Thing>,
    pub bushes: Vec<Thing>,
    pub walls: Vec<Thing>,
    pub doors: Vec<Thing>,
    pub natural_barriers: Vec<Thing>,
}

impl ThingsStorage {
    pub fn store(&mut self, thing: Thing) {
        match thing.thing_type {
            ThingType::Tree | ThingType::FertileTree  => self.trees.push(thing),
            ThingType::Bush | ThingType::FertileBush => self.bushes.push(thing),
            ThingType::Rock => self.rocks_and_ores.push(thing),
            ThingType::Boulder | ThingType::Log => self.natural_barriers.push(thing),
            ThingType::CopperOre | ThingType::IronOre => self.rocks_and_ores.push(thing),
            ThingType::WoodenWall | ThingType::StoneWall |ThingType::IronWall | ThingType::SteelWall => self.walls.push(thing),
            ThingType::WoodenDoor | ThingType::ReinforcedWoodenDoor | ThingType::IronDoor | 
            ThingType::ReinforcedIronDoor | ThingType::SteelDoor | ThingType::ReinforcedSteelDoor => self.doors.push(thing),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GameScene {
    pub scene_type: SceneType,
    pub scene_id: usize,
    pub index: usize, // vector index in scene_manager.ground_scene;
    pub tilemap: Tilemap,
    pub things: ThingsStorage,
    pub stuff: Vec<Stuff>,
    pub charactors: Vec<Charactor>,
    pub effects: Vec<SceneEffect>,
    //pub roof: Vec<>,
}

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(SceneState::GameScene)
            //draw playable ground tilemap;
            .with_system(tilemap::draw::draw)
            // draw things;
            .with_system(thing::draw::draw)
            //draw all charactor and player
            .with_system(charactor::draw::draw)
            .with_system(charactor::draw::draw_player)
        );

        app.add_system_set(SystemSet::on_update(SceneState::GameScene)
            .with_system(thing::destroeyd_thing_handler::destroeyd_thing_handler)
            .with_system(tilemap::change_cover_type_handler::change_cover_type_handler)
            .with_system(charactor::update_move::move_charactor)
            .with_system(charactor::killed_charactor_handler::killed_charactor_handler)
            .with_system(charactor::player_click_function::player_click)
            .with_system(charactor::update_effects::update_effects)
            .with_system(charactor::update_cooldowns::update_active_skills_cooldown)
            .with_system(charactor::update_passive_skills::update_passive_skills)
            //.with_system(charactor::update_attack::player_attacking)
        );

        app.add_system_set(SystemSet::on_exit(SceneState::GameScene)
            //cleanup tilemap, all tiles and store them;
            .with_system(tilemap::cleanup::cleanup)
            //cleanup all things and store them;
            .with_system(thing::cleanup::cleanup)
            //cleanup charactors with player and store them;
            .with_system(charactor::cleanup::cleanup)
        );
    }
}

