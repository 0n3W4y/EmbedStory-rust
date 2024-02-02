use serde::{Deserialize, Serialize};

pub mod charactor;
pub mod scene_effect;
pub mod thing;
pub mod stuff;
pub mod projectiles;
pub mod damage_text_informer;
pub mod update_damage;


#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Stat{
    #[default]
    Strength,
    Dexterity,
    Wisdom,
    Vitality,
    Luck,
}

impl Stat {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            Stat::Strength,
            Stat::Dexterity,
            Stat::Wisdom,
            Stat::Vitality,
            Stat::Luck
        ].into_iter()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize, Hash)]
pub enum Attribute {
    #[default]
    Health,
    Stamina,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Resist {
    FireDamage,
    ColdDamage,
    #[default]
    PhisicalDamage,
    WaterDamage,
    ElectricDamage,
    AcidDamage,
    PoisonDamage,
    HealthDamage,
    StaminaDamage,
}

impl Resist {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            Resist::FireDamage,
            Resist::ColdDamage,
            Resist::PhisicalDamage,
            Resist::WaterDamage,
            Resist::ElectricDamage,
            Resist::AcidDamage,
            Resist::PoisonDamage,
        ].into_iter()
    }

    pub fn damage(damage: &Damage) -> Self {
        match *damage {
            Damage::Fire => Resist::FireDamage,
            Damage::Cold => Resist::ColdDamage,
            Damage::Electric => Resist::ElectricDamage,
            Damage::Acid => Resist::AcidDamage,
            Damage::Poison => Resist::PoisonDamage,
            Damage::Phisical => Resist::PhisicalDamage,
            Damage::Water => Resist::WaterDamage,
            Damage::Health => Resist::HealthDamage,
            Damage::Stamina => Resist::StaminaDamage,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Deserialize, Serialize)]
pub enum Ability{
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
    ExperienceMultiplier,
    PhisicalDamage,
    FireDamage,
    ElectricDamage,
    WaterDamage,
    AcidDamage,
    PoisonDamage,
    ColdDamage,
    ReducingEffectTime,
    HealthDamage,
    StaminaDamage,
}

impl Ability {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            Ability::Evasion,
            Ability::MovementSpeed,
            Ability::AttackSpeed,
            Ability::ActiveSkillsCoolDawn,
            Ability::BlockAmount,
            Ability::BlockChance,
            Ability::CriticalHitChanse,
            Ability::CriticalHitMultiplier,
            Ability::Accuracy,
            Ability::ExperienceMultiplier,
            Ability::PhisicalDamage,
            Ability::FireDamage,
            Ability::ElectricDamage,
            Ability::WaterDamage,
            Ability::AcidDamage,
            Ability::PoisonDamage,
            Ability::ColdDamage,
        ].into_iter()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum Damage {
    Fire,
    Cold,
    Electric,
    Acid,
    Poison,
    #[default]
    Phisical,
    Water,
    Health,
    Stamina
}

impl Damage {
    pub fn get_attribute(&self) -> Attribute {
        match *self {
            Damage::Fire => Attribute::Health,
            Damage::Cold => Attribute::Health,
            Damage::Electric => Attribute::Health,
            Damage::Acid => Attribute::Health,
            Damage::Poison => Attribute::Health,
            Damage::Phisical => Attribute::Health,
            Damage::Water => Attribute::Health,
            Damage::Health => Attribute::Health,
            Damage::Stamina => Attribute::Stamina,
        }
    }
}
