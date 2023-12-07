use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::resources::deploy::game_scene_biome_deploy::BiomeType;
use crate::resources::deploy::game_scene_deploy::Location;
use crate::resources::deploy::game_scene_deploy::LocationType;
use crate::resources::scene_data::charactor::Charactor;
use crate::resources::scene_data::projectiles;
use crate::resources::scene_data::scene_effect::SceneEffect;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_data::thing::Thing;
use crate::resources::scene_data::thing::ThingType;
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

    pub fn get_all_things(&self) -> Vec<&Thing> {
        let mut result: Vec<&Thing> = vec![];
        for rock_and_ore in self.rocks_and_ores.iter() {
            result.push(rock_and_ore);
        }

        for tree in self.trees.iter() {
            result.push(tree);
        }

        for bush in self.bushes.iter() {
            result.push(bush);
        }

        for wall in self.walls.iter() {
            result.push(wall);
        }

        for door in self.doors.iter() {
            result.push(door);
        }

        for natural_barrier in self.natural_barriers.iter() {
            result.push(natural_barrier);
        }

        result
    }

    pub fn get_all_things_mut(&mut self) -> Vec<&mut Thing> {
        let mut result: Vec<&mut Thing> = vec![];
        for rock_and_ore in self.rocks_and_ores.iter_mut() {
            result.push(rock_and_ore);
        }

        for tree in self.trees.iter_mut() {
            result.push(tree);
        }

        for bush in self.bushes.iter_mut() {
            result.push(bush);
        }

        for wall in self.walls.iter_mut() {
            result.push(wall);
        }

        for door in self.doors.iter_mut() {
            result.push(door);
        }

        for natural_barrier in self.natural_barriers.iter_mut() {
            result.push(natural_barrier);
        }

        result
    }

    pub fn clear_all(&mut self) {
        self.rocks_and_ores.clear();
        self.bushes.clear();
        self.doors.clear();
        self.natural_barriers.clear();
        self.trees.clear();
        self.walls.clear();
    }
 }

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct CharactorStorage {
    pub player: Vec<Charactor>,
    pub companion: Vec<Charactor>,
    pub npc: Vec<Charactor>,
    pub monster: Vec<Charactor>,
}

impl CharactorStorage {
    pub fn store(&mut self, charactor: Charactor){
        match charactor.charactor_type {
            charactor::CharactorType::Player => self.player.push(charactor),
            charactor::CharactorType::NPC => self.npc.push(charactor),
            charactor::CharactorType::Monster => self.monster.push(charactor),
            charactor::CharactorType::Companion => self.companion.push(charactor),
        }
    }

    pub fn get_all_charactors(&self) -> Vec<&Charactor> {
        let mut result: Vec<&Charactor> = vec![];
        for player in self.player.iter() {
            result.push(player);
        }

        for npc in self.npc.iter(){
            result.push(npc);
        }

        for monster in self.monster.iter() {
            result.push(monster);
        }

        for companion in self.companion.iter() {
            result.push(companion);
        }
        result
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GameScene {
    pub location: Location,
    pub location_type: LocationType,
    pub biome_type: BiomeType,
    pub scene_id: usize,
    pub tilemap: Tilemap,
    pub things: ThingsStorage,
    pub stuff: Vec<Stuff>,
    pub charactors: CharactorStorage,
    pub effects: Vec<SceneEffect>,
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
            .with_system(charactor::active_skill_handler::active_skill_handler)
            .with_system(projectiles::update_projectile::update_projectiles)
            .with_system(charactor::update_attack::update_attack_from_basic_skill)
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

