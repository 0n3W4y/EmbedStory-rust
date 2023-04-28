use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use rand::Rng;

use super::{scene_data::objects::{charactor::{RaceType, CharactorType, Charactor, stats::Stat, CharactorSubType, self}, body_part::PartType}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::deploy::charactor_deploy::{RaceConfig, CharactorSubTypeConfig};
use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::body_part::BodyPartType;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager{
    id: usize,
}

impl CharactorManager {
    //TODO: Function
    pub fn create_charactor(& mut self, deploy: &Deploy, charactor_type: &CharactorType, charactor_subtype: &CharactorSubType, race_type: &RaceType) -> Charactor{
        let id = self.create_id();
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(race_type);
        let charactor_subtype_config: &CharactorSubTypeConfig = deploy.charactor_deploy.get_charactor_subtype_config(charactor_subtype);

        let resists: HashMap<Resist, i16> = create_resists(&race_config.resists);

        let body_structure: HashMap<BodyPartType, BodyPart> = create_body_structure(&race_config.body_structure, &race_config.body_structure_part_type);
        let total_health_points: i16 = charactor::calculate_total_health_points(&body_structure);
        let current_health_points: i16 = charactor::calculate_current_health_points(&body_structure);

        let stats: HashMap<Stat, u8> = generate_stats(&charactor_subtype_config.stats, charactor_subtype_config.stat_max_random_value);

        let charactor = Charactor{
            id,
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            charactor_subtype: charactor_subtype.clone(),
            stats_cache: stats.clone(),
            stats: stats,            
            resists_cache: resists.clone(),
            resists,            
            resist_min_value: race_config.resist_min_value,
            resist_max_value: race_config.resist_max_value,
            stat_min_value: race_config.stat_min_value,
            body_structure,
            total_health_points,
            current_health_points,
            ..Default::default()
        };

        return charactor;
    }

    pub fn palce_charator_on_tile(&self, charactor:&mut Charactor, tile: &mut Tile){
        
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}

pub fn create_resists(resists: &HashMap<Resist, i16>) -> HashMap<Resist, i16>{
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
        (Resist::Fatigue, 0)
    ]);

    for (resist, value) in resists.iter() {
        match new_resists.get_mut(resist) {
            Some(v) => *v = *value,
            None => println!("Can't set resist '{:?}', key not available", resist),
        };
    };
    return new_resists;
}

pub fn create_body_structure(config: &HashMap<BodyPartType, i16>, part_type: &PartType) -> HashMap<BodyPartType, BodyPart> {
    let mut body_structure: HashMap<BodyPartType, BodyPart> = HashMap::new();
    for (body_part_type, value) in config {
        let mut bodypart = BodyPart{
            part_type: part_type.clone(),
            ..Default::default()
        };
        bodypart.set_current_health_points(*value);
        bodypart.set_total_health_points(*value);
        bodypart.set_modified_health_points(*value);
        body_structure.insert(body_part_type.clone(), bodypart);
    };

    return body_structure;
}

fn generate_stats(charactor_subtype_stats_config: &HashMap<Stat, u8>, stat_max_random_value: i8) -> HashMap<Stat, u8>{
    let mut stats: HashMap<Stat, u8> = HashMap::from([ 
        (Stat::Strength, 1),
        (Stat::Intellect, 1),
        (Stat::Endurance, 1),
        (Stat::Vitality, 1),
        (Stat::Agility, 1),
        (Stat::Mobility, 1)
    ]);
    let mut rand = rand::thread_rng();
    for (stat, value) in charactor_subtype_stats_config {
        let min_stat_value = *value as i8 - stat_max_random_value;
        let max_stat_value = *value as i8 + stat_max_random_value;
        let stat_value = rand.gen_range(min_stat_value..max_stat_value);
        match stats.get_mut(stat){
            Some(v) => *v = stat_value as u8,
            None => {
                println!("Can't add stat into stats because: '{:?}' from config not available in stats", stat);
            }
        }
    }

    return stats;
}