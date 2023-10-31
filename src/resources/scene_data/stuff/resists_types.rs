use serde::{Serialize, Deserialize};

use crate::resources::scene_data::charactor::effects::EffectType;

use super::damage_type::DamageType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Default, Hash)]
pub enum ResistType {
    FireDamage,
    ColdDamage,
    ElectricDamage,
    AcidDamage,
    PoisonDamage,
    HealthDamage,
    StaminaDamage,
    #[default]
    PhisicalDamage,
    WaterDamage,
    StunEffect,
    AcidEffect,
    MovelessEffect,
    SlowEffect,
    BleedingEffect,
    BurnEffect,
    ElectrificationEffect,
    FreezeEffect,
    BlindEffect,
    PoisonEffect,
    WetEffect,
    BrokenArmorEffect,
    BrokenWeaponEffect,
    IncreaseMovementEffect,
    FrostbiteEffect,
}

impl ResistType {
    pub fn all_values() -> impl Iterator<Item = Self> {
        vec![
            ResistType::FireDamage,
            ResistType::ColdDamage,
            ResistType::ElectricDamage,
            ResistType::AcidDamage,
            ResistType::PoisonDamage,
            ResistType::HealthDamage,
            ResistType::StaminaDamage,
            ResistType::PhisicalDamage,
            ResistType::WaterDamage,
            ResistType::StunEffect,
            ResistType::AcidEffect,
            ResistType::MovelessEffect,
            ResistType::SlowEffect,
            ResistType::BleedingEffect,
            ResistType::BurnEffect,
            ResistType::ElectrificationEffect,
            ResistType::FreezeEffect,
            ResistType::BlindEffect,
            ResistType::PoisonEffect,
            ResistType::WetEffect,
            ResistType::BrokenArmorEffect,
            ResistType::BrokenWeaponEffect,
            ResistType::IncreaseMovementEffect,
            ResistType::FrostbiteEffect
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

pub fn get_resist_from_effect_type(effect_type: &EffectType) -> ResistType {
    return match *effect_type {
        EffectType::Stun => ResistType::StunEffect,
        EffectType::Acid => ResistType::AcidEffect,
        EffectType::Moveless => ResistType::MovelessEffect,
        EffectType::Slow => ResistType::SlowEffect,
        EffectType::Bleeding => ResistType::BleedingEffect,
        EffectType::Burn => ResistType::BurnEffect,
        EffectType::Electrification => ResistType::ElectrificationEffect,
        EffectType::Freeze => ResistType::FreezeEffect,
        EffectType::Blind => ResistType::BlindEffect,
        EffectType::Poison => ResistType::PoisonEffect,
        EffectType::Wet => ResistType::WetEffect,
        EffectType::BrokeArmor => ResistType::BrokenArmorEffect,
        EffectType::BrokeWeapon => ResistType::BrokenWeaponEffect,
        EffectType::IncreaseMovement => ResistType::IncreaseMovementEffect,
        EffectType::Frostbite => ResistType::FrostbiteEffect,
    }
}