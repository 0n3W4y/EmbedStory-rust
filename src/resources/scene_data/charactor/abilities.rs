use serde::Deserialize;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Deserialize)]
pub enum Ability{
    Evasion,
    #[default]
    MovementSpeed,
    AttackSpeed,
    ActiveSkillsCoolDawn,
    BlockChance,
    BlockAmount,
    CritChance,
    CritDamage,
    Accuracy,
    StaminaRegen,
    HealthRegen,
    ExperienceMultiplier,
    MeleeWeaponDamage,
    RangedWeaponDamage,
    MagicWeaponDamage,
    MeleeTrinketDamage,
    RangedTrinketDamage,
    MagicTrinketDamage,
}