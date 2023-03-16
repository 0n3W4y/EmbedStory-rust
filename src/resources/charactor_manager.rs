use serde::{Serialize, Deserialize};

use super::{scene_data::objects::charactor::{RaceType, CharactorType, Charactor, RaceDeploy}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;
use crate::resources::scene_data::objects::resists::Resist;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager{
    id: usize,
}

impl CharactorManager {
    pub fn create_player(&mut self, deploy: &Deploy) -> Charactor {
        let race_type = RaceType::Human;
        let config: RaceDeploy = deploy.charactor_deploy.race_deploy.get_race_config(&race_type);
        let id = self.create_id();
        let mut resist: Vec<Resist> = vec![];

        let mut charactor = Charactor {
            charactor_type: CharactorType::Player,
            race_type,
            id,

            ..Default::default()
        };
        return charactor;
    }
    pub fn create_charactor(&mut self, race_type: &RaceType, charactor_type: &CharactorType, deploy: &Deploy) -> Charactor{
        let id = self.create_id();
        let mut charactor = Charactor{
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            id,
            ..Default::default()
        };

        return charactor;
    }

    pub fn create_charator_on_tile(&mut self, race_type: &RaceType, charactor_type: &CharactorType, deploy: &Deploy, tile: &mut Tile){
        let mut caharctor = self.create_charactor(race_type, charactor_type, deploy);
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}