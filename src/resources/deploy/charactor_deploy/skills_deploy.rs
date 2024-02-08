use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::{deploy::DEPLOY_SKILLS_PATH, scene_data::charactor::skills::{ActiveSkillDeploy, PassiveSkillDeploy, ActiveSkillType, PassiveSkillType}};

#[derive(Deserialize, Debug)]
pub struct SkillsDeploy{
    pub active_skills: ActiveSkillsDeploy,
    pub passive_skills: PassiveSkillsDeploy,
}

#[derive(Deserialize, Debug)]
pub struct ActiveSkillsDeploy {
    pub base_skill: ActiveSkillDeploy,
}

#[derive(Deserialize, Debug)]
pub struct PassiveSkillsDeploy {
    pub chainlighting: PassiveSkillDeploy,
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
    pub fn get_active_skill_deploy (&self, skill: &ActiveSkillType) -> &ActiveSkillDeploy {
        match *skill {
            ActiveSkillType::BaseSkill => &self.active_skills.base_skill,
        }
    }

    pub fn get_passive_skill_deploy (&self, skill: &PassiveSkillType) -> &PassiveSkillDeploy {
        match *skill {
            PassiveSkillType::ChainlightingPassive => &self.passive_skills.chainlighting,
        }
    }
}