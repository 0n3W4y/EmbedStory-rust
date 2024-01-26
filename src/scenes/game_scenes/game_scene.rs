use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use serde::{Deserialize, Serialize};

use crate::components::projectile_component::Projectile;
use crate::resources::deploy::game_scene_biome_deploy::BiomeType;
use crate::resources::deploy::game_scene_deploy::Location;
use crate::resources::scene_data::charactor::Charactor;
use crate::resources::scene_data::damage_text_informer;
use crate::resources::scene_data::projectiles;
use crate::resources::scene_data::scene_effect::SceneEffect;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_data::thing::Thing;
use crate::resources::scene_data::thing::ThingType;
use crate::resources::scene_data::update_damage;
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap;
use crate::resources::scene_data::thing;
use crate::resources::scene_data::charactor;
use crate::scenes::AppState;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ThingsStorage {
    pub rocks_and_ores: Vec<Thing>,
    pub trees: Vec<Thing>,
    pub bushes: Vec<Thing>,
    pub walls: Vec<Thing>,
    pub doors: Vec<Thing>,
    pub natural_barriers: Vec<Thing>,
    pub dungeon_entrace: Vec<Thing>,
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
            ThingType::DungeonEnter(_) | ThingType::DungeonExit(_) => self.dungeon_entrace.push(thing),
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

        for dungeon_entrance in self.dungeon_entrace.iter() {
            result.push(dungeon_entrance);
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

        for dungeon_entrance in self.dungeon_entrace.iter_mut() {
            result.push(dungeon_entrance);
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
        self.dungeon_entrace.clear();
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
    pub biome_type: BiomeType,
    pub scene_id: usize,
    pub tilemap: Tilemap,
    pub things: ThingsStorage,
    pub stuff: Vec<Stuff>,
    pub charactors: CharactorStorage,
    pub effects: Vec<SceneEffect>,
    pub projectiles: Vec<Projectile>,
}

pub struct GameScenePlugin;

impl Plugin for GameScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(tilemap::draw::draw.in_schedule(OnEnter(AppState::GameScene)))
            .add_system(thing::draw::draw.in_schedule(OnEnter(AppState::GameScene)))
            .add_system(charactor::draw::draw.in_schedule(OnEnter(AppState::GameScene)))

            .add_systems(
                (
                    thing::destroeyd_thing_handler::destroeyd_thing_handler,
                    tilemap::change_cover_type_handler::change_cover_type_handler,
                    charactor::update_move::move_charactor,
                    charactor::killed_charactor_handler::killed_charactor_handler,
                    charactor::player_click_function::player_click,
                    charactor::update_passive_skills::update_passive_skills,
                    charactor::active_skill_handler::active_skill_handler,
                    charactor::update_attack::update_attack_from_basic_skill,
                    projectiles::update_projectile::create_projectiles
                )
                .in_set(OnUpdate(AppState::GameScene))
            )

            .add_system(
                charactor::update_cooldowns::update_active_skills_cooldown
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.1))))
            .add_system(
                update_damage::update_damage
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.1))))
            .add_system(
                charactor::update_effects::update_effects
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.1))))
            .add_system(
                projectiles::update_projectile::update_projectiles
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.1))))
            .add_system(
                damage_text_informer::update_damage_text_informer
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.25))))
            .add_system(
                charactor::update_target_position::update_target_position
                .in_set(OnUpdate(AppState::GameScene))
                .run_if(on_timer(Duration::from_secs_f32(0.5))))
            //on exit
            .add_systems(
                (
                    tilemap::cleanup::cleanup,
                    thing::cleanup::cleanup,
                    charactor::cleanup::cleanup
                )
                .in_schedule(OnExit(AppState::GameScene))
            );
    }
}

