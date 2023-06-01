use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    deploy::Deploy,
    scene_data::objects::{
        body_part::PartType,
        charactor::{
            self, stats::Stat, Charactor, CharactorSubType, CharactorType, ConditionType,
            GenderType, RaceType,
        },
    },
};
use crate::resources::deploy::charactor_deploy::{CharactorSubTypeConfig, RaceConfig};
use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::body_part::BodyPartType;
use crate::resources::scene_data::objects::resists::Resist;
use crate::scenes::game_scenes::{game_scene::GameScene, tilemap::tile::Tile};
use crate::resources::deploy::game_scene_biome_deploy::BiomeCharacters;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager {
    id: usize,
}

impl CharactorManager {
    //TODO: Function
    pub fn create_charactor(
        &mut self,
        deploy: &Deploy,
        charactor_type: &CharactorType,
        charactor_subtype: &CharactorSubType,
        race_type: &RaceType,
        gender: &GenderType,
    ) -> Charactor {
        let id = self.create_id();
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(race_type);
        let charactor_subtype_config: &CharactorSubTypeConfig = deploy
            .charactor_deploy
            .get_charactor_subtype_config(charactor_subtype);

        let stats: HashMap<Stat, u8> = generate_stats(
            &charactor_subtype_config.stats,
            charactor_subtype_config.stat_max_random_value,
        );

        let resists: HashMap<Resist, i16> = create_resists(&race_config.resists);

        let body_structure: HashMap<BodyPartType, BodyPart> = create_body_structure(
            &race_config.body_structure,
            &race_config.body_structure_part_type,
        );
        let total_health_points: i16 = charactor::calculate_total_health_points(&body_structure);
        let current_health_points: i16 =
            charactor::calculate_current_health_points(&body_structure);

        let condition: HashMap<ConditionType, u16> = create_conditions(&race_config.conditions);
        let condition_max: HashMap<ConditionType, u16> =
            create_conditions_max(&race_config.conditions);

        let mut charactor = Charactor {
            id,
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            charactor_subtype: charactor_subtype.clone(),
            gender_type: gender.clone(),
            stats_cache: stats.clone(),
            stats: stats,
            resists_cache: resists.clone(),
            resists,
            resist_min_value: race_config.resist_min_value,
            resist_max_value: race_config.resist_max_value,
            stat_min_value: race_config.stat_min_value,
            condition,
            condition_max,
            body_structure,
            total_health_points,
            current_health_points,
            ..Default::default()
        };

        initialize_character_after_creation(&mut charactor);

        return charactor;
    }

    pub fn palce_charator_on_tile(&self, charactor: &mut Charactor, tile: &mut Tile) {
        charactor.position = tile.position.clone();
        tile.charactor_type = Some(charactor.id);
    }

    pub fn generate_mosnters_for_scene(
        &mut self,
        scene: &mut GameScene,
        deploy: &Deploy,
        config: &BiomeCharacters,
    ) {
        
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}

pub fn create_resists(resists: &HashMap<Resist, i16>) -> HashMap<Resist, i16> {
    let mut new_resists: HashMap<Resist, i16> = HashMap::from([
        (Resist::Kinetic, 0),
        (Resist::Fire, 0),
        (Resist::Electric, 0),
        (Resist::Plasma, 0),
        (Resist::Laser, 0),
        (Resist::Poison, 0),
        (Resist::Knockdown, 0),
        (Resist::Bleed, 0),
        (Resist::Disease, 0),
        (Resist::Pain, 0),
        (Resist::Fatigue, 0),
    ]);

    for (resist, value) in resists.iter() {
        match new_resists.get_mut(resist) {
            Some(v) => *v = *value,
            None => println!("Can't set resist '{:?}', key not available", resist),
        };
    }
    return new_resists;
}

pub fn create_conditions(conditions: &HashMap<ConditionType, u16>) -> HashMap<ConditionType, u16> {
    let mut condition: HashMap<ConditionType, u16> =
        HashMap::from([(ConditionType::Pain, 0), (ConditionType::Fatigue, 0)]);

    for (key, value) in conditions {
        match condition.get_mut(key) {
            Some(v) => {}
            None => println!(
                "charactor_manager.create_conditions. There is no '{:?}' in condition",
                key
            ),
        }
    }

    return condition;
}

pub fn create_conditions_max(
    conditions: &HashMap<ConditionType, u16>,
) -> HashMap<ConditionType, u16> {
    let mut condition: HashMap<ConditionType, u16> =
        HashMap::from([(ConditionType::Pain, 0), (ConditionType::Fatigue, 0)]);

    for (key, value) in conditions {
        match condition.get_mut(key) {
            Some(v) => *v = *value,
            None => println!(
                "charactor_manager.create_conditions_max. There is no '{:?}' in condition",
                key
            ),
        }
    }

    return condition;
}

pub fn create_body_structure(
    config: &HashMap<BodyPartType, i16>,
    part_type: &PartType,
) -> HashMap<BodyPartType, BodyPart> {
    let mut body_structure: HashMap<BodyPartType, BodyPart> = HashMap::new();
    for (body_part_type, value) in config {
        let mut bodypart = BodyPart {
            part_type: part_type.clone(),
            ..Default::default()
        };
        bodypart.set_current_health_points(*value);
        bodypart.set_total_health_points(*value);
        bodypart.set_modified_health_points(*value);
        body_structure.insert(body_part_type.clone(), bodypart);
    }

    return body_structure;
}

fn generate_stats(
    charactor_subtype_stats_config: &HashMap<Stat, u8>,
    stat_max_random_value: i8,
) -> HashMap<Stat, u8> {
    let mut stats: HashMap<Stat, u8> = HashMap::from([
        (Stat::Strength, 1),
        (Stat::Intellect, 1),
        (Stat::Endurance, 1),
        (Stat::Dexterity, 1),
        (Stat::Perception, 1),
    ]);
    let mut rand = rand::thread_rng();
    for (stat, value) in charactor_subtype_stats_config {
        let min_stat_value = *value as i8 - stat_max_random_value;
        let max_stat_value = *value as i8 + stat_max_random_value;
        let stat_value = rand.gen_range(min_stat_value..max_stat_value);
        match stats.get_mut(stat) {
            Some(v) => *v = stat_value as u8,
            None => {
                println!(
                    "Can't add stat into stats because: '{:?}' from config not available in stats",
                    stat
                );
            }
        }
    }

    return stats;
}

pub fn initialize_character_after_creation(charactor: &mut Charactor) {
    //calculate resists by stats
    let str = charactor.stats.get(&Stat::Strength).unwrap();
    let end = charactor.stats.get(&Stat::Endurance).unwrap();
    let int = charactor.stats.get(&Stat::Intellect).unwrap();
    let dex = charactor.stats.get(&Stat::Dexterity).unwrap();
    let per = charactor.stats.get(&Stat::Perception).unwrap();

    let resist_max_value = charactor.resist_max_value;
    let resist_min_value = charactor.resist_min_value;

    let resists = charactor.resists.clone();

    for (key, _) in resists {
        match key {
            Resist::Kinetic => {
                let value_to_change: i16 = calculate_resist_kinetic_from_stat(*str, *end);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            Resist::Bleed => {
                let value_to_change: i16 = calculate_resist_bleed_from_stat(*str, *end, *dex);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            Resist::Disease => {
                let value_to_change: i16 = calculate_resist_disease_from_stat(*str, *end, *per);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            Resist::Fatigue => {
                let value_to_change: i16 = calculate_resist_fatigue_from_stat(*str, *end, *dex);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            Resist::Knockdown => {
                let value_to_change: i16 =
                    calculate_resist_knockdown_from_stat(*str, *end, *dex, *int);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            Resist::Pain => {
                let value_to_change: i16 = calculate_resist_pain_from_stat(*str, *end, *int, *per);
                charactor::change_resist(
                    &mut charactor.resists,
                    &mut charactor.resists_cache,
                    &key,
                    value_to_change,
                    resist_min_value,
                    resist_max_value,
                );
            }
            _ => {} // all other resist does not depend on stats;
        };
    }

    //calculate skills by stats;
    //TODO
}

pub fn calculate_resist_kinetic_from_stat(str: u8, end: u8) -> i16 {
    //formula: str -5 + end -6;
    let value: i16 = str as i16 + end as i16 - 11;
    return value;
}

pub fn calculate_resist_bleed_from_stat(str: u8, end: u8, dex: u8) -> i16 {
    //formula: end - 5 + 5 - str + 5 - dex;
    let value: i16 = end as i16 + 5 + (5 - str as i16) - (5 - dex as i16);
    return value;
}

pub fn calculate_resist_disease_from_stat(str: u8, end: u8, per: u8) -> i16 {
    //formula str - 3 + end - 3 + per - 5;
    let value: i16 = str as i16 + end as i16 + per as i16 - 11;
    return value;
}

pub fn calculate_resist_fatigue_from_stat(str: u8, end: u8, dex: u8) -> i16 {
    //formula str - 6 + end - 8 - dex;
    let value: i16 = str as i16 + end as i16 - dex as i16 - 14;
    return value;
}

pub fn calculate_resist_knockdown_from_stat(str: u8, end: u8, dex: u8, int: u8) -> i16 {
    //formula end - 2 + dex - 8 + str - 6 - int;
    let value: i16 = end as i16 + dex as i16 + str as i16 - 16 - int as i16;
    return value;
}

pub fn calculate_resist_pain_from_stat(str: u8, end: u8, int: u8, per: u8) -> i16 {
    //formula: str + end - 5 - int - (per - 3);
    let value: i16 = str as i16 + end as i16 - 5 - int as i16 - (per as i16 - 3);
    return value;
}

//TODO: calculate_skill_xxx_from_stat;
