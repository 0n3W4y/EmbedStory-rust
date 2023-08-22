use serde::Deserialize;
use std::collections::HashMap;

use crate::resources::scene_data::stuff::damage_type::DamageType;

use super::{effects::EffectType, CharactorType};

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SkillType {
    #[default]
    BasicWeaponAttack,
}

#[derive(Deserialize, Default, Debug, Eq, PartialEq)]
pub enum CastSource {
    Mouse,
    #[default]
    Itself,
    Target,
}

#[derive(Deserialize, Default, Debug, Eq, PartialEq)]
pub enum SkillDirectionType {
    #[default]
    Line,
    Arc45,
    Arc90,
    Arc180,
    Arc360,
    Point,
}

#[derive(Default, Debug)]
pub struct Skill {
    pub skill_type: SkillType,
   
    //for passive skill;
    pub passive_skill: bool,
    pub trigger_chanse: u8,
    pub trigger_time: f32,
    pub trigger_duration: f32,
    //-----------------

    pub base_cooldown: i16,
    pub current_cooldown: f32, // base + & from ability;
    pub current_duration: f32, // == 0.0;
    pub total_duration: f32,

    pub projectiles: u8,
    pub range: u8,
    pub cast_source: CastSource,
    pub skill_direction: SkillDirectionType,

    pub base_crit_chance: i8,
    pub current_crit_chance: i8,

    pub base_damage: HashMap<DamageType, i16>,
    pub current_damage: HashMap<DamageType, i16>,

    pub base_crit_multiplier: i16,
    pub current_crit_multiplier: i16,

    pub base_stamina_cost: u8,
    pub current_stamina_cost: u8,

    pub target: CharactorType,

    pub effect: HashMap<EffectType, u8>,
}

#[derive(Deserialize)]
pub struct SkillDeploy {
    pub skill_type: SkillType,
    pub skill_queue: u8,
    pub passive_skill: bool,

    pub trigger_chanse: u8,
    pub trigger_time: u16,
    pub trigger_duration: u16,
    pub base_cooldown: i16,

    pub projectiles: u8,
    pub range: u8,
    pub cast_source: CastSource,

    pub base_crit_chance: u8,

    pub base_damage: HashMap<DamageType, i16>,

    pub base_crit_multiplier: i16,

    pub stamina_cost: u8,

    pub skill_direction: SkillDirectionType,

    pub target: CharactorType,

    pub effect: HashMap<EffectType, u8>,
}