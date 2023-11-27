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
    Wisdom,
    Luck,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Hash)]
pub enum Attribute {
    #[default]
    Health,
    Stamina,
}

use self::stuff::damage_type::DamageType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum ResistType {
    FireDamage,
    ColdDamage,
    #[default]
    PhisicalDamage,
    WaterDamage,
    ElectricDamage,
    AcidDamage,
    PoisonDamage,
    HealthDamage,
    StaminaDamage
}

impl ResistType {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            ResistType::FireDamage,
            ResistType::ColdDamage,
            ResistType::PhisicalDamage,
            ResistType::WaterDamage,
            ResistType::ElectricDamage,
            ResistType::AcidDamage,
            ResistType::PoisonDamage,
            ResistType::HealthDamage,
            ResistType::StaminaDamage,
        ].into_iter()
    }
}

pub fn get_resist_from_damage_type(damage_type: &DamageType) -> ResistType {
    return match *damage_type {
        DamageType::Fire => ResistType::FireDamage,
        DamageType::Cold => ResistType::ColdDamage,
        DamageType::Electric => ResistType::ElectricDamage,
        DamageType::Acid => ResistType::AcidDamage,
        DamageType::Poison => ResistType::PoisonDamage,
        DamageType::Health => ResistType::HealthDamage,
        DamageType::Stamina => ResistType::StaminaDamage,
        DamageType::Phisical => ResistType::PhisicalDamage,
        DamageType::Water => ResistType::WaterDamage,
    }
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
    ReducingEffectTime,
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
