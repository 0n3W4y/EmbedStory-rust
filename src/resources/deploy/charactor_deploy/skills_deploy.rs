use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{deploy::DEPLOY_SKILLS_PATH, scene_data::charactor::skills::{SkillType, SkillDeploy}};

#[derive(Deserialize, Debug)]
pub struct SkillsDeploy{
    pub base_skill: SkillDeploy,
}

impl SkillsDeploy {
    pub fn new() -> Self {
        let skills_deploy: SkillsDeploy = match File::open(DEPLOY_SKILLS_PATH) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                serde_json::from_str(&content).expect("JSON was not well-formatted")
            }
            Err(e) => panic!("Can not open objects data file: {}, {}", e, DEPLOY_SKILLS_PATH),
        };
        return skills_deploy;
    }
    pub fn get_skill_deploy( &self, skill: &SkillType ) -> &SkillDeploy {
        match *skill {
            SkillType::BaseSkill => &self.base_skill,
        }
    }
}