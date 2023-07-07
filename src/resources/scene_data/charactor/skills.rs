use serde::Deserialize;
use std::collections::HashMap;

use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::effects::EffectType;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    BasicWeaponAttack,
}

#[derive(Deserialize, Default)]
pub enum CastSource {
    Mouse,
    #[default]
    Itself,
}

#[derive(Deserialize, Default)]
pub enum SkillDirectionType {
    #[default]
    Line,
    Arc90,
    Arc180,
    Around360,
    Point,
}

#[derive(Deserialize, Default)]
pub enum SkillTargetType {
    #[default]
    Enemy,
    Friendly,
    Any
}

#[derive(Default)]
pub struct Skill {
    pub skill_type: SkillType,
    pub skill_queue: u8,
    pub passive_skill: bool,

    pub trigger_chanse: u8,
    pub trigger_time: u16,    
    pub base_cool_dawn: i16,
    pub current_cool_dawn: i16,
    pub current_duration: f32,  

    pub projectiles: u8,
    pub range: u8,
    pub cast_source: CastSource,

    pub base_crit_chance: u8,

    pub base_damage: HashMap<DamageType, i16>,

    pub base_crit_multiplier: i16,

    pub stamina_cost: u8,

    pub skill_direction: SkillDirectionType,

    pub target: SkillTargetType,
    pub max_target: u8,

    pub effect: Vec<EffectType>,
}

#[derive(Deserialize)]
pub struct SkillDeploy {
    pub skill_type: SkillType,
    pub skill_queue: u8,
    pub passive_skill: bool,

    pub trigger_chanse: u8,
    pub trigger_time: u16,
    pub base_cool_dawn: i16,

    pub projectiles: u8,
    pub range: u8,
    pub cast_source: CastSource,

    pub base_crit_chance: u8,

    pub base_damage: HashMap<DamageType, i16>,

    pub base_crit_multiplier: i16,

    pub stamina_cost: u8,

    pub skill_direction: SkillDirectionType,

    pub target: SkillTargetType,
    pub max_target: u8,

    pub effect: Vec<EffectType>,
}