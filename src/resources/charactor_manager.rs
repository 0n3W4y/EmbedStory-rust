use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use super::{scene_data::objects::{charactor::{RaceType, CharactorType, Charactor, NPCType, stats::Stat}, body_part::PartType}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::scene_data::objects::charactor::MonsterType;
use crate::resources::scene_data::objects::charactor::CompanionType;
use crate::resources::deploy_addiction::charactor_deploy::RaceConfig;
use crate::resources::scene_data::objects::body_part::BodyPart;
use crate::resources::scene_data::objects::body_part::BodyPartType;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager{
    id: usize,
}

impl CharactorManager {
    pub fn create_player(&mut self, deploy: &Deploy) -> Charactor {
        let race_type = RaceType::Human;
        let charactor_type = CharactorType::Player;
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);
        let mut charactor = self.create_charactor(&charactor_type, &race_type);

        let mut stat = create_stats();
        let mut resist: HashMap<Resist, i16> = create_resists(&race_config.resists);
        let mut body_structure:Vec<BodyPart> = vec![];

        
        return charactor;
    }
    pub fn create_npc(&mut self, race_type: &RaceType, npc_type: &NPCType, deploy: &Deploy) -> Charactor{
        let charactor_type = CharactorType::NPC(npc_type.clone());
        let mut charactor = self.create_charactor(&charactor_type, &race_type);
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);

        return charactor;
    }

    pub fn create_monster(&mut self, race_type: &RaceType, monster_type: &MonsterType, deploy: &Deploy) -> Charactor {
        let charactor_type = CharactorType::Monster(monster_type.clone());
        let mut charactor = self.create_charactor(&charactor_type, &race_type);
        let race_config: &RaceConfig = deploy.charactor_deploy.get_race_config(&race_type);

        return charactor;
    }

    pub fn create_compnaion(&mut self, race_type: &RaceType, companion_type: &CompanionType, deploy: &Deploy) -> Charactor {
        let charactor_type = CharactorType::PlayerCompanion(companion_type.clone());
        let mut charactor = self.create_charactor(&charactor_type, race_type);
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

    fn create_charactor(&mut self, charactor_type: &CharactorType, race_type: &RaceType) -> Charactor {
        let id = self.create_id();
        let charactor = Charactor{
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            id,
            ..Default::default()
        };
        return charactor;
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
    let mut resist_hashmap: HashMap<Resist, i16> = HashMap::from([ 
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

    for resist in resists.iter() {
        let value = resist.get_resist();
        match *resist {
            Resist::Kinetic(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Kinetic(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Fire(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Fire(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Electric(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Electric(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Plasma(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Plasma(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Laser(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Laser(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Poison(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Poison(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Knockdown(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Knockdown(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Bleed(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Bleed(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Disease(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Disease(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Pain(_) => {
                let val = vec.iter_mut().find(|x: &&mut Resist|{x == &&Resist::Pain(0)}).unwrap();
                *val = resist.clone();
            },
            Resist::Fatigue(_) => {
                let val = vec.iter_mut().find(|x|{x == &&Resist::Fatigue(0)}).unwrap();
                *val = resist.clone();
            },
        }
    };
    return vec;
}

fn create_body_structure(config: &HashMap<BodyPartType, u16>, part_type: &PartType) -> Vec<BodyPart> {
    let mut vec: Vec<BodyPart> = vec![];
    for (body_part_type, value) in config {
        let bodypart = BodyPart{ 
            bodypart_type: body_part_type.clone(),
            current_health_points: Stat::HealthPoints(*value as i16),
            total_health_points: Stat::HealthPoints(*value as i16), 
            modified_health_points: Stat::HealthPoints(*value as i16),
            ..Default::default()
        };
        vec.push(bodypart);
    };

    return vec;
}

fn generate_stat_for_monster(vec_stat: &mut Vec<Stat>, stat_points: u8){

}

fn generate_stats_for_npc(vec_stat: &mut Vec<Stat>, stat_points: u8, npc_type: &CharactorType){

}