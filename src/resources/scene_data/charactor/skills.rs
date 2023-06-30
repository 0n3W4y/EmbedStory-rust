use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    BasicWeaponAttack,
    BasicTrinketAttack,
}

pub struct Skill {
    skill_type: SkillType,
    passive_skill: bool,
}