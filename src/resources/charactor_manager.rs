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
use crate::{components::AttributesComponent, scenes::game_scenes::game_scene::GameScene};

#[derive(Default, Clone, Serialize, Deserialize)]
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
        initialize_character_after_creation(&mut charactor);
        return charactor;
    }

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


        initialize_character_after_creation(&mut charactor);
        return charactor;
    }

    pub fn generate_charactors_for_scene(&mut self, scene: &mut GameScene, deploy: &Deploy, player_level: u8) {
        todo!();
    }

    fn create_charactor(&mut self, deploy: &Deploy, race_type: &RaceType, gender: &GenderType, charactor_type: CharactorType) -> Charactor {
        let id = self.create_id();
        let race_config: &RaceConfig = deploy.charactor_deploy.race_deploy.get_race_config(race_type);
        Charactor {
            id,
            race_type: race_type.clone(),
            charactor_type: charactor_type,
            gender_type: gender.clone(),
            stats_cache: race_config.stats.clone(),
            stats: race_config.stats.clone(),
            resists: race_config.resists.clone(),
            ..Default::default()
        }
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