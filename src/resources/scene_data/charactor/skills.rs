use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    BasicWeaponAttack,
    BasicTrinketAttack,
}

pub struct Skill {
    pub skill_type: SkillType,
    //pub passive_skill: bool,
    pub trigger_time: u16,
    pub trigger_chanse: u8,
    pub current_duration: f32,
    pub triggered: u32,
}