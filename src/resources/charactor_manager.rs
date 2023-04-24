use serde::{Serialize, Deserialize};

use super::{scene_data::objects::charactor::{RaceType, CharactorType, Charactor, NPCType, stats::Stat}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_data::objects::resists::Resist;
use crate::resources::scene_data::objects::charactor::MonsterType;
use crate::resources::scene_data::objects::charactor::CompanionType;
use crate::resources::deploy_addiction::charactor_deploy::RaceConfig;
use crate::resources::scene_data::objects::body_part::BodyPart;

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

        let mut stat: Vec<Stat> = vec![];
        let mut resist: Vec<Resist> = vec![];
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