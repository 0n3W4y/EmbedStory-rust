use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{
    deploy::{Deploy, charactor_deploy::race_deploy::RaceConfig},
    scene_data::{
        charactor::{
            Charactor, CharactorType,
            GenderType, RaceType, do_stat_dependences, CharactorStrength, STATS_POINTS_EVERY_LEVEL
    }, Stat, Ability, Resist},
};
use crate::{components::AttributesComponent, scenes::game_scenes::{game_scene::GameScene, tilemap::tile::{TilePermissions, Position}}};

#[derive(Default, Clone, Serialize, Deserialize, Resource)]
pub struct CharactorManager {
    id: usize,
}

impl CharactorManager {
    pub fn create_player(
        &mut self,
        deploy: &Deploy,
        player_race: &RaceType,
        player_gender: &GenderType
    ) -> Charactor {
        let mut charactor = self.create_charactor(deploy, player_race, player_gender, CharactorType::Player);
        self.initialize_character_after_creation(&mut charactor);
        return charactor;
    }

    pub fn create_companion(){}
    pub fn create_npc(){}

    pub fn create_monster(
        &mut self,
        deploy: &Deploy, 
        level: u8, 
        monster_race: &RaceType, 
        monster_strength: &CharactorStrength,
        monster_gender: &GenderType,
    ) -> Charactor {
        let mut random = rand::thread_rng();
        let monster_strength = deploy.charactor_deploy.monster_deploy.get_monster_strength(monster_strength);
        let mut charactor = self.create_charactor(deploy, monster_race, monster_gender, CharactorType::Monster);

        let total_points = level as i16 * STATS_POINTS_EVERY_LEVEL as i16;

        let priority_stat_points = total_points / 2 + monster_strength.stats;
        let other_points = total_points - priority_stat_points;
        let middle_stat_ratio =  random.gen_range(0..=80) / 40;                     // DEX first 40, LUCK other 40; at random = 40 = we have 1:1 ratio; DEX / LUCK;
        let middle_stat_points = other_points * 80 / 100;
        let middle_first_points = middle_stat_points / 2 * middle_stat_ratio + monster_strength.stats;
        let middle_second_points = middle_stat_points - middle_first_points + monster_strength.stats;
        let non_priority_stat_points = other_points - middle_stat_points + monster_strength.stats;

        let random_number: u8 = random.gen_range(0..=2);
        match random_number {
            0 => {
                //melee STR - 1, DEX - 0.4, LUCK - 0.4, WIS - 0.2;
                let proirity_stat_value = charactor.stats.entry(Stat::Strength).and_modify(|x| *x += priority_stat_points).or_insert(priority_stat_points);
                charactor.stats_cache.entry(Stat::Strength).and_modify(|x| *x = *proirity_stat_value).or_insert(*proirity_stat_value);

                let middle_first_stat_value = charactor.stats.entry(Stat::Dexterity).and_modify(|x| *x += middle_first_points).or_insert(middle_first_points);
                charactor.stats_cache.entry(Stat::Dexterity).and_modify(|x| *x = *middle_first_stat_value).or_insert(*middle_first_stat_value);

                let middle_second_stat_value = charactor.stats.entry(Stat::Luck).and_modify(|x| *x += middle_second_points).or_insert(middle_second_points);
                charactor.stats_cache.entry(Stat::Luck).and_modify(|x| *x = *middle_second_stat_value).or_insert(*middle_second_stat_value);

                let non_priority_stat_value = charactor.stats.entry(Stat::Wisdom).and_modify(|x| *x += non_priority_stat_points).or_insert(non_priority_stat_points);
                charactor.stats_cache.entry(Stat::Wisdom).and_modify(|x| *x = *non_priority_stat_value).or_insert(*non_priority_stat_value);
            },
            1 => {
                //range DEX -1 , STR - 0.4, WIS - 0.4, LUCK - 0.2;
                let proirity_stat_value = charactor.stats.entry(Stat::Dexterity).and_modify(|x| *x += priority_stat_points).or_insert(priority_stat_points);
                charactor.stats_cache.entry(Stat::Dexterity).and_modify(|x| *x = *proirity_stat_value).or_insert(*proirity_stat_value);

                let middle_first_stat_value = charactor.stats.entry(Stat::Wisdom).and_modify(|x| *x += middle_first_points).or_insert(middle_first_points);
                charactor.stats_cache.entry(Stat::Wisdom).and_modify(|x| *x = *middle_first_stat_value).or_insert(*middle_first_stat_value);

                let middle_second_stat_value = charactor.stats.entry(Stat::Strength).and_modify(|x| *x += middle_second_points).or_insert(middle_second_points);
                charactor.stats_cache.entry(Stat::Strength).and_modify(|x| *x = *middle_second_stat_value).or_insert(*middle_second_stat_value);

                let non_priority_stat_value = charactor.stats.entry(Stat::Luck).and_modify(|x| *x += non_priority_stat_points).or_insert(non_priority_stat_points);
                charactor.stats_cache.entry(Stat::Luck).and_modify(|x| *x = *non_priority_stat_value).or_insert(*non_priority_stat_value);
            },
            2 => {
                //magic WIS - 1, DEX - 0.4, LUCK - 0.4, STR - 0.2;
                let proirity_stat_value = charactor.stats.entry(Stat::Wisdom).and_modify(|x| *x += priority_stat_points).or_insert(priority_stat_points);
                charactor.stats_cache.entry(Stat::Wisdom).and_modify(|x| *x = *proirity_stat_value).or_insert(*proirity_stat_value);

                let middle_first_stat_value = charactor.stats.entry(Stat::Dexterity).and_modify(|x| *x += middle_first_points).or_insert(middle_first_points);
                charactor.stats_cache.entry(Stat::Dexterity).and_modify(|x| *x = *middle_first_stat_value).or_insert(*middle_first_stat_value);

                let middle_second_stat_value = charactor.stats.entry(Stat::Luck).and_modify(|x| *x += middle_second_points).or_insert(middle_second_points);
                charactor.stats_cache.entry(Stat::Luck).and_modify(|x| *x = *middle_second_stat_value).or_insert(*middle_second_stat_value);

                let non_priority_stat_value = charactor.stats.entry(Stat::Strength).and_modify(|x| *x += non_priority_stat_points).or_insert(non_priority_stat_points);
                charactor.stats_cache.entry(Stat::Strength).and_modify(|x| *x = *non_priority_stat_value).or_insert(*non_priority_stat_value);
            },
            _ => {},
        }
       
        for (_, value) in charactor.resists.iter_mut() {                            //increase resists by monster strength
            *value += monster_strength.resists;
        }

        for (_, value) in charactor.attributes.iter_mut(){                              //increase attributes;
            *value += monster_strength.attributes;
        }

        for (_, value) in charactor.attributes_cache.iter_mut(){                        //increase attributes_cache;
            *value += monster_strength.attributes;
        }

        for (ability_type, value) in charactor.ability.iter_mut() {         //increase abilities
            let new_value = match monster_strength.abilities.get(ability_type) {
                Some(v) => *v,
                None => 0,
            };

            *value += new_value;
        }


        self.initialize_character_after_creation(&mut charactor);
        return charactor;
    }

    pub fn generate_monsters_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy, player_level: u8) {
        let mut random = rand::thread_rng();
        let monster_strength_variants: Vec<CharactorStrength> = vec![CharactorStrength::Weak, CharactorStrength::Normal, CharactorStrength::Elite, CharactorStrength::Boss, CharactorStrength::Champion];
        let monster_gender_variants: Vec<GenderType> = vec![GenderType::Male, GenderType::Female];
        let location_config = deploy.game_scene.get_scene_setting(&scene.location);
        let min_value = location_config.monsters_min;
        let max_value = location_config.monsters_max;
        let monsters = random.gen_range(min_value..=max_value);
        for _ in 0..monsters {
            let monster_strength = &monster_strength_variants[random.gen_range(0..monster_strength_variants.len())];
            let monster_race = &location_config.races[random.gen_range(0..location_config.races.len())];
            let monster_level = random.gen_range(player_level..=(player_level+5));
            let monster_gender = &monster_gender_variants[random.gen_range(0..monster_gender_variants.len())];
            let monster = self.create_monster(deploy, monster_level, monster_race, monster_strength, monster_gender);
            //create stuff for monster, fill inventory;
    
            scene.charactors.store(monster);
        }

        self.generate_positions_for_monsters(scene);
    }

    fn create_charactor(&mut self, deploy: &Deploy, race_type: &RaceType, gender: &GenderType, charactor_type: CharactorType) -> Charactor {
        let id = self.create_id();
        let race_config: &RaceConfig = deploy.charactor_deploy.race_deploy.get_race_config(race_type);
        let mut charactor = Charactor {
            id,
            race_type: race_type.clone(),
            charactor_type: charactor_type,
            gender_type: gender.clone(),
            ..Default::default()
        };

        for ability_type in Ability::all_values() {
            let ability_value = match race_config.ability.get(&ability_type) {
                Some(v) => *v,
                None => 0,
            };
            charactor.ability.insert(ability_type.clone(), ability_value);
        }

        for resist_type in Resist::all_values() {
            let resist_value = match race_config.resists.get(&resist_type) {
                Some(v) => *v,
                None => 0,
            };
            charactor.resists.insert(resist_type.clone(), resist_value);
        }

        for stat_type in Stat::all_values() {
            let stat_value = match race_config.stats.get(&stat_type) {
                Some(v) => *v,
                None => 0,
            };
            charactor.stats.insert(stat_type.clone(), stat_value);
            charactor.stats_cache.insert(stat_type.clone(), stat_value);
        }
        return charactor;
    }
    fn generate_positions_for_monsters(&self, scene: &mut GameScene) {
        let mut random = rand::thread_rng();
        let mut position_pool: Vec<Position<i32>> = vec![];
        for tile in scene.tilemap.get_tilemap_tile_storage().iter() {
            match tile.permissions.iter().find(|&x| *x == TilePermissions::Walk) {
                Some(_) => position_pool.push(tile.position.clone()),
                None => {},
            }
        }

        if position_pool.len() <= scene.charactors.get_all_charactors().len() {
            panic!("Positions pool is not enough for charactors positions");
        }

        for monster in scene.charactors.monster.iter_mut() {
            let random_index = random.gen_range(0..position_pool.len());
            let position = &position_pool[random_index];
            monster.position = position.clone();
            position_pool.remove(random_index);
        }

    }

    fn initialize_character_after_creation(&self, charactor: &mut Charactor) {
        //TODO:
        for (stat, value) in charactor.stats.iter() {
            let mut component: AttributesComponent = AttributesComponent { attributes: charactor.attributes.clone(), attributes_cache: charactor.attributes_cache.clone() };
            do_stat_dependences(&mut charactor.resists, &mut charactor.ability, &mut component, stat, *value, 0);
            charactor.attributes = component.attributes;
            charactor.attributes_cache = component.attributes_cache;
        }
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}




