use serde::{Deserialize, Serialize};

pub mod charactor;
pub mod scene_effect;
pub mod thing;
pub mod stuff;
pub mod projectiles;
pub mod damage_text_informer;


#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Stat{
    #[default]
    Strength,
    Dexterity,
    Intellect,
    Luck,
    HealthPoints,
    StaminaPoints,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Hash)]
pub enum Attribute {
    #[default]
    Health,
    Stamina,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Deserialize, Serialize)]
pub enum AbilityType{
    Evasion,
    #[default]
    MovementSpeed,
    AttackSpeed,
    ActiveSkillsCoolDawn,
    BlockChance,
    BlockAmount,
    CriticalHitChanse,
    CriticalHitMultiplier,
    Accuracy,
    StaminaRegen,
    HealthRegen,
    ExperienceMultiplier,
    PhisicalDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    AcidDamage,
    PoisonDamage,
    ColdDamage,
    HealthDamage,
    StaminaDamage,
}

impl AbilityType {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            AbilityType::Evasion,
            AbilityType::MovementSpeed,
            AbilityType::AttackSpeed,
            AbilityType::ActiveSkillsCoolDawn,
            AbilityType::BlockAmount,
            AbilityType::BlockChance,
            AbilityType::CriticalHitChanse,
            AbilityType::CriticalHitMultiplier,
            AbilityType::Accuracy,
            AbilityType::StaminaRegen,
            AbilityType::HealthRegen,
            AbilityType::ExperienceMultiplier,
            AbilityType::PhisicalDamage,
            AbilityType::FireDamage,
            AbilityType::ElectricDamage,
            AbilityType::WaterDamage,
            AbilityType::AcidDamage,
            AbilityType::PoisonDamage,
            AbilityType::ColdDamage,
            AbilityType::HealthDamage,
            AbilityType::StaminaDamage,
        ].into_iter()
    }
}
