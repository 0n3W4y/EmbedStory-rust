use std::{fs::File, collections::HashMap, io::Read};

use serde::Deserialize;

use crate::resources::{deploy::DEPLOY_RACE_PATH, scene_data::{charactor::{RaceType, skills::PassiveSkillType}, Stat, Attribute, Resist, Ability}};


#[derive(Deserialize, Debug)]
pub struct RaceDeploy {
    pub human: RaceConfig,
    pub elf: RaceConfig,
    pub orc: RaceConfig,
    pub halfling: RaceConfig,
    pub dwarf: RaceConfig,
    pub lizardfolk: RaceConfig,
    pub naga: RaceConfig,
    pub gnome: RaceConfig,
    pub goblin: RaceConfig,
    pub minotaur: RaceConfig,
    pub harpia: RaceConfig,
    pub dryada: RaceConfig,
    pub fairy: RaceConfig,
    pub celestial: RaceConfig,
    pub elemental: RaceConfig,
    pub skeleton: RaceConfig,
    pub zombie: RaceConfig,
    pub ogre: RaceConfig,
    pub demon: RaceConfig,
    pub ghost: RaceConfig,
    pub wolf: RaceConfig,
    pub bear: RaceConfig,
    pub crocodile: RaceConfig,
    pub scorpion: RaceConfig,
    pub eagle: RaceConfig,
    pub spider: RaceConfig,
    pub komodo_dragon: RaceConfig,
    pub rhinocerops: RaceConfig,
    pub snake: RaceConfig,
}

impl RaceDeploy {
    pub fn new() -> Self {
        let race_deploy: RaceDeploy = match File::open(DEPLOY_RACE_PATH) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!(
                "Can not open objects data file: {}, {}",
                e, DEPLOY_RACE_PATH
            ),
        };
        return race_deploy;
    }

    pub fn get_race_config(&self, race_type: &RaceType) -> &RaceConfig {
        match race_type {
            RaceType::Human => &self.human,
            RaceType::Elf => &self.elf,
            RaceType::Orc => &self.orc,
            RaceType::Halfling => &self.halfling,
            RaceType::Dwarf => &self.dwarf,
            RaceType::Naga => &self.naga,
            RaceType::Gnome => &self.gnome,
            RaceType::Goblin => &self.goblin,
            RaceType::Lizardfolk => &self.lizardfolk,
            RaceType::Minotaur => &self.minotaur,
            RaceType::Harpia => &self.harpia,
            RaceType::Dryada => &self.dryada,
            RaceType::Fairy => &self.fairy,
            RaceType::Celestial => &self.celestial,
            RaceType::Elemental => &self.elemental,
            RaceType::Skeleton => &self.skeleton,
            RaceType::Zombie => &self.zombie,
            RaceType::Ogre => &self.ogre,
            RaceType::Demon => &self.demon,
            RaceType::Ghost => &self.ghost,
            RaceType::Wolf => &self.wolf,
            RaceType::Bear => &self.bear,
            RaceType::Crocodile => &self.crocodile,
            RaceType::Scorpion => &self.scorpion,
            RaceType::Eagle => &self.eagle,
            RaceType::Spider => &self.spider,
            RaceType::KomodoDragon => &self.komodo_dragon,
            RaceType::Rhinocerops => &self.rhinocerops,
            RaceType::Snake => &self.snake,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct RaceConfig {
    pub stats: HashMap<Stat, i16>,
    pub attributes: HashMap<Attribute, i16>,
    pub resists: HashMap<Resist, i16>,
    pub ability: HashMap<Ability, i16>,
    pub passive_skills: Vec<PassiveSkillType>,
}