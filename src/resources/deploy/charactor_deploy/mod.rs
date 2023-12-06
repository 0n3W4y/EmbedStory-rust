use serde::Deserialize;

use self::{race_deploy::RaceDeploy, effects_deploy::EffectsDeploy, skills_deploy::SkillsDeploy, monster_deploy::MonsterDeploy, npc_deploy::NPCDeploy, companion_deploy::CompanionDeploy};

pub mod companion_deploy;
pub mod effects_deploy;
pub mod monster_deploy;
pub mod npc_deploy;
pub mod race_deploy;
pub mod skills_deploy;

#[derive(Deserialize, Debug)]
pub struct CharactorDeploy {
    pub race_deploy: RaceDeploy,
    pub effects_deploy: EffectsDeploy,
    pub skills_deploy: SkillsDeploy,
    pub monster_deploy: MonsterDeploy,
    pub npc_deploy: NPCDeploy,
    pub companion_deploy: CompanionDeploy,
}

impl CharactorDeploy {
    pub fn new() -> Self {
        
        let race_deploy: RaceDeploy = RaceDeploy::new();
        let effects_deploy: EffectsDeploy = EffectsDeploy::new();
        let skills_deploy: SkillsDeploy = SkillsDeploy::new();
        let monster_deploy: MonsterDeploy  = MonsterDeploy::new();
        let npc_deploy: NPCDeploy = NPCDeploy::new();
        let companion_deploy: CompanionDeploy = CompanionDeploy::new();

        return CharactorDeploy {
            race_deploy,
            effects_deploy,
            skills_deploy,
            monster_deploy,
            npc_deploy,
            companion_deploy
        };
    }
}