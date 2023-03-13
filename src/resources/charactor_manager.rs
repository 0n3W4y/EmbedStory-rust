use serde::{Serialize, Deserialize};

use super::{scene_data::objects::charactor::{RaceType, CharactorType, Charactor}, deploy::Deploy};
use crate::scenes::game_scenes::tilemap::tile::Tile;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager{
    id: usize,
}

impl CharactorManager {
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