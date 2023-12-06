use rand::Rng;
use serde::{Deserialize, Serialize};

use super::{
    deploy::{Deploy, charactor_deploy::race_deploy::RaceConfig},
    scene_data::{
        charactor::{
            Charactor, CharactorType,
            GenderType, RaceType, do_stat_dependences, CharactorStrength, STATS_POINTS_EVERY_LEVEL
    }, Stat},
};
use crate::components::AttributesComponent;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CharactorManager {
    id: usize,
}

impl CharactorManager {
    pub fn create_player(){}
    pub fn create_companion(){}
    pub fn create_npc(){}

    pub fn create_monster(
        &mut self,
        deploy: &Deploy, 
        level: usize, 
        monster_race: &RaceType, 
        monster_strength: &CharactorStrength,
        monster_gender: &GenderType,
    ) -> Charactor {
        let mut random = rand::thread_rng();
        let race_config: &RaceConfig = deploy.charactor_deploy.race_deploy.get_race_config(monster_race);
        let monster_strength = deploy.charactor_deploy.monster_deploy.get_monster_strength(monster_strength);
        let id = self.create_id();
        let mut charactor = Charactor {
            id,
            race_type: monster_race.clone(),
            charactor_type: CharactorType::Monster,
            gender_type: monster_gender.clone(),
            stats_cache: race_config.stats.clone(),
            stats: race_config.stats.clone(),
            resists: race_config.resists.clone(),
            ..Default::default()
        };

        let total_points = level as i16 * STATS_POINTS_EVERY_LEVEL as i16;

        let priority_stat_points = total_points / 2;
        let other_points = total_points - priority_stat_points;
        let middle_stat_ratio =  random.gen_range(0..=80) / 40;                     // DEX first 40, LUCK other 40; at random = 40 = we have 1:1 ratio; DEX / LUCK;
        let middle_stat_points = other_points * 80 / 100;
        let middle_first_points = middle_stat_points / 2 * middle_stat_ratio;
        let middle_second_points = middle_stat_points - middle_first_points;
        let non_priority_stat_points = other_points - middle_stat_points;

        let random_number: u8 = random.gen_range(0..=2);
        match random_number {
            0 => {
                //melee STR - 1, DEX - 0.4, LUCK - 0.4, WIS - 0.2;
                for (stat, value) in charactor.stats.iter_mut() {
                    match *stat {
                        Stat::Strength => {
                            *value += priority_stat_points;
                            charactor.stats_cache.entry(stat.clone()).and_modify(|x| *x = *value).or_insert(*value);
                        },
                        Stat::Dexterity => {
                            *value += middle_first_points;
                            charactor.stats_cache.entry(stat.clone()).and_modify(|x| *x = *value).or_insert(*value);
                        },
                        Stat::Wisdom => {
                            *value += non_priority_stat_points;
                            charactor.stats_cache.entry(stat.clone()).and_modify(|x| *x = *value).or_insert(*value);
                        },
                        Stat::Luck => {
                            *value += middle_second_points;
                            charactor.stats_cache.entry(stat.clone()).and_modify(|x| *x = *value).or_insert(*value);
                        },
                    }
                }
            },
            1 => {
                //range DEX -1 , STR - 0.4, LUCK - 0.4, WIS - 0.2;
            },
            2 => {
                //magic WIS - 1, DEX - 0.4, LUCK - 0.4, STR - 0.2;
            },
        }

        initialize_character_after_creation(&mut charactor);
        return charactor;
    }

    fn create_id(&mut self) -> usize {
        let id = self.id;
        self.id += 1;
        return id;
    }
}




pub fn initialize_character_after_creation(charactor: &mut Charactor) {
    //TODO:
    for (stat, value) in charactor.stats.iter() {
        let mut component: AttributesComponent = AttributesComponent { attributes: charactor.attributes.clone(), attributes_cache: charactor.attributes_cache.clone() };
        do_stat_dependences(&mut charactor.resists, &mut charactor.ability, &mut component, stat, *value, 0);
        charactor.attributes = component.attributes;
        charactor.attributes_cache = component.attributes_cache;
    }
}