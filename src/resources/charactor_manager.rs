//use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    deploy::Deploy,
    scene_data::
        charactor::{
            stats::Stat, Charactor, CharactorType,
            GenderType, RaceType, do_stat_dependences
        },
};
use crate::resources::deploy::charactor_deploy::RaceConfig;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager {
    id: usize,
}

impl CharactorManager {
    pub fn create_charactor(
        &mut self,
        deploy: &Deploy,
        charactor_type: &CharactorType,
        race_type: &RaceType,
        gender: &GenderType,
    ) -> Charactor {
        let id = self.create_id();
        let race_config: &RaceConfig = deploy.charactor_deploy.race_deploy.get_race_config(race_type);              // Stats resists ability;

        let mut charactor = Charactor {
            id,
            race_type: race_type.clone(),
            charactor_type: charactor_type.clone(),
            gender_type: gender.clone(),
            stats_cache: race_config.stats.clone(),
            stats: race_config.stats.clone(),
            resists: race_config.resists.clone(),
            ..Default::default()
        };

        initialize_character_after_creation(&mut charactor);

        return charactor;
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}

fn generate_stats(
    stats: &mut HashMap<Stat, u8>,
    level: u8,
) {
    //TODO:
}

pub fn initialize_character_after_creation(charactor: &mut Charactor) {
    //TODO:
    for (stat, value) in charactor.stats.iter() {
        do_stat_dependences(&mut charactor.resists, &mut charactor.ability, stat, *value, 0);
    }
}