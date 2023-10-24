use serde::{Serialize, Deserialize};

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