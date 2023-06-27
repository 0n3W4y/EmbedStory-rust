use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ActiveSkill {
    #[default]
    BasicWeaponAttack,
    BasicTrinketAttack,
}


pub enum PassiveSkill {
    Evasion,
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
    MeleeAttackDamage,
    RangedAttackDamage,
    MagicAttackDamage,
    TrinketMeleeDamage,
    TrinketRangeDamage,
    TrinketMagicDamage,
    ExperienceMultiplier,

    LifeLich,
    StaminaLich,
    Stun,
    Slow,
    Moveless,
    Bleed,
    Burn,
    Electrification,
    Freeze,
    Blind,
    Poison,
    Acid,
    Wet,
}
