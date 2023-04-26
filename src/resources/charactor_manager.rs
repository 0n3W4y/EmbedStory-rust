use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{scene_data::objects::{charactor::{RaceType, CharactorType, Charactor, stats::Stat, CharactorSubType}, body_part::PartType}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::deploy_addiction::charactor_deploy::RaceConfig;
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
        let charactor = Charactor{
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            charactor_subtype: charactor_subtype.clone(),
            stats: create_stats(),
            stats_cache: create_stats(),
            id,
            ..Default::default()
        };
        return charactor;
    }
    pub fn create_player(&mut self, deploy: &Deploy) -> Charactor {
        let race_type = RaceType::Human;
        let charactor_type = CharactorType::Player;
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);
        let mut charactor = self.create_charactor(&charactor_type, &race_type);

        let resists: HashMap<Resist, i16> = create_resists(&race_config.resists);
        charactor.resists = resists;
        charactor.resists_cache = resists;
        charactor.resist_min_value = race_config.resist_min_value;
        charactor.resist_max_value = race_config.resist_max_value;
        charactor.stat_min_value = race_config.stat_min_value;
        let mut body_structure:Vec<BodyPart> = vec![];

        
        return charactor;
    }
    pub fn create_npc(&mut self, race_type: &RaceType, charactor_subtype: &CharactorSubType, deploy: &Deploy) -> Charactor{
        let charactor_type = CharactorType::NPC;
        let mut charactor = self.create_charactor(&charactor_type, &charactor_subtype, &race_type);
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);

        return charactor;
    }

    pub fn create_monster(&mut self, race_type: &RaceType, charactor_subtype: &CharactorSubType, deploy: &Deploy) -> Charactor {
        let charactor_type = CharactorType::Monster(monster_type.clone());
        let mut charactor = self.create_charactor(&charactor_type, &charactor_subtype, &race_type);
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);

        return charactor;
    }

    pub fn create_compnaion(&mut self, race_type: &RaceType, charactor_subtype: &CharactorSubType, deploy: &Deploy) -> Charactor {
        let charactor_type = CharactorType::PlayerCompanion(companion_type.clone());
        let mut charactor = self.create_charactor(&charactor_type, &charactor_subtype, &race_type);
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);

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

fn create_stats() -> HashMap<Stat, u8>{
    let stats: HashMap<Stat, u8> = HashMap::from([ 
        (Stat::Strength, 1),
        (Stat::Intellect, 1),
        (Stat::Endurance, 1),
        (Stat::Vitality, 1),
        (Stat::Agility, 1),
        (Stat::Mobility, 1)
        ]);
    return stats;
}

fn create_resists(resists: &HashMap<Resist, i16>) -> HashMap<Resist, i16>{
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

fn create_body_structure(config: &HashMap<BodyPartType, i16>, part_type: &PartType) -> Vec<BodyPart> {
    let mut vec: Vec<BodyPart> = vec![];
    for (body_part_type, value) in config {
        let mut bodypart = BodyPart{ 
            bodypart_type: body_part_type.clone(),
            ..Default::default()
        };
        bodypart.set_modified_health_points(*value);
        bodypart.set_total_health_points(*value);
        bodypart.set_current_health_points(*value);
        vec.push(bodypart);
    };

    return vec;
}

fn generate_stat_for_monster(vec_stat: &mut Vec<Stat>, stat_points: u8){

}

fn generate_stats_for_npc(vec_stat: &mut Vec<Stat>, stat_points: u8, npc_type: &CharactorType){

}