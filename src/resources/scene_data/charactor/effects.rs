use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::scene_data::{Stat, AbilityType, Attribute, ResistType};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EffectType{
    Stun,
    AcidDebuff,
    Moveless,
    BleedingDebuff,
    BurnDebuff,
    ColdDebuff,
    ElectricDebuff,
    WaterDebuff,
    Freeze,
    Blind,
    PosisonDebuff,
    MovementBuff,
    MovementDebuff,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EffectStatus {
    CanNotMove,
    CanNotAttack,
}

#[derive(Deserialize, Debug, Clone)]
pub struct EffectDeploy {
    pub effect_type: EffectType,
    pub effect_duration: f32,
    
    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
    pub effect_status: Vec<EffectStatus>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Effect {
    pub effect_type: EffectType,
    pub effect_duration: f32,
    pub total_time_duration: f32,

    pub change_stat: HashMap<Stat, i16>,
    pub change_attribute_cache: HashMap<Attribute, i16>,
    pub change_resist: HashMap<ResistType, i16>,
    pub change_ability: HashMap<AbilityType, i16>,
    pub effect_status: Vec<EffectStatus>,
}

impl Effect {
    pub fn new(config: &EffectDeploy) -> Self {
        Effect {
            effect_type: config.effect_type.clone(),
            effect_duration: config.effect_duration,
            total_time_duration: 0.0,
            change_stat: config.change_stat.clone(),
            change_ability: config.change_ability.clone(),
            change_attribute_cache: config.change_attribute_cache.clone(),
            change_resist: config.change_resist.clone(),
            effect_status: config.effect_status.clone(),
        }
    }
}
