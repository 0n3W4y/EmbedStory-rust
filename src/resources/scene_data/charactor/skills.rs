use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{resources::{scene_data::{stuff::{Stuff, StuffType}, projectiles::ProjectileType, Ability, Damage}, deploy::Deploy}, components::StatsComponent};

use super::{effects::{EffectType, Effect}, get_ability_type_from_damage_type};

pub const MINIMAL_TIME_FOR_COOLDOWN_SKILL: f32 = 0.20;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ActiveSkillType {
    #[default]
    BaseSkill
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PassiveSkillType {
    #[default]
    ChainlightingPassive
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum TargetType {
    #[default]
    Allies,
    Enemies,
    All,
}

#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone)]
pub enum SkillDirectionType {
    #[default]
    Itself,
    Target,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PassiveSkill {
    pub skill_type: PassiveSkillType,
    pub trigger_time_frequency: f32,
    pub trigger_chance: u8,
    pub skill_life_time: f32,
    pub current_time_duration: f32,
    pub total_duration: f32,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effects: HashMap<EffectType, (Effect, u8)>,

    pub skill_range: u8,
    pub skill_direction: SkillDirectionType,
    pub target_type: TargetType,
    pub target_quantity: u8,                                // max target quantity in skill range;
    pub area_on_impact: u8,                                 //0 - only target, 1 - +1 position for all direction, 2 - +2 position for all direction;

    pub projectile_type: Option<ProjectileType>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveSkill {
    pub skill_type: ActiveSkillType,
    pub is_activated: bool,                                     // activeated skill will start logic to dealt damage to target;
    pub on_cooldown: bool,                                  // can use this skill now;
    pub cooldown_time: f32,
    pub current_time_duration: f32,                            // == 0.0;
    pub stamina_cost: i16,
    
    pub projectile_type: Option<ProjectileType>,
    pub skill_range: u8,                                    // max range; min range = 1;
    pub skill_direction: SkillDirectionType,
    pub target: TargetType,
    pub target_quantity: u8,                                // max target quantity in skill range;
    pub area_on_impact: u8,                                 //0 - only target, 1 - +1 position for all direction, 2 - +2 position for all direction;

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effects: HashMap<EffectType, (Effect, u8)>,
    pub passive_skills: HashMap<PassiveSkillType, (PassiveSkill, u8)>,
}

impl ActiveSkill {
    pub fn new (deploy: &Deploy, skill_type: &ActiveSkillType) -> Self {
        let mut effects: HashMap<EffectType, (Effect, u8)> = HashMap::new();
        let mut passive_skills: HashMap<PassiveSkillType, (PassiveSkill, u8)> = HashMap::new();
        let skill_config = deploy.charactor_deploy.skills_deploy.get_active_skill_deploy(skill_type);
        for (effect_type, effect_chance) in skill_config.effects.iter() {
            let effect = Effect::new(deploy, effect_type);
            effects.insert(effect_type.clone(), (effect, *effect_chance));
        }

        for (passive_skill_type, skill_chance) in skill_config.passive_skills.iter() {
            let passive_skill = PassiveSkill::new(deploy, passive_skill_type);
            passive_skills.insert(passive_skill_type.clone(), (passive_skill, *skill_chance));
        }

        ActiveSkill {
            skill_type: skill_type.clone(),
            is_activated: false,
            cooldown_time: skill_config.cooldown_time as f32 / 10.0,
            on_cooldown: false,
            current_time_duration: 0.0,
            projectile_type: skill_config.projectile_type.clone(),
            skill_direction: skill_config.skill_direction.clone(),
            stamina_cost: skill_config.stamina_cost,
            target: skill_config.target.clone(),
            crit_chance: skill_config.crit_chance,
            crit_multiplier: skill_config.crit_multiplier,
            damage: skill_config.damage.clone(),
            effects,
            passive_skills: passive_skills,
            skill_range: skill_config.skill_range,
            target_quantity: skill_config.target_quantity,
            area_on_impact: skill_config.area_on_impact,
        }
    }
}

impl PassiveSkill {
    pub fn new (deploy: &Deploy, skill_type: &PassiveSkillType) -> Self {
        let mut effects: HashMap<EffectType, (Effect, u8)> = HashMap::new();
        let skill_config = deploy.charactor_deploy.skills_deploy.get_passive_skill_deploy(skill_type);
        for (effect_type, effect_chance) in skill_config.effects.iter() {
            let effect = Effect::new(deploy, effect_type);
            effects.insert(effect_type.clone(), (effect, *effect_chance ));
        }

        PassiveSkill {
            skill_type: skill_type.clone(),
            trigger_time_frequency: skill_config.trigger_time_frequency,
            trigger_chance: skill_config.trigger_chance,
            skill_life_time: skill_config.skill_life_time,
            current_time_duration: 0.0,
            total_duration: 0.0,
            crit_chance: skill_config.crit_chance,
            crit_multiplier: skill_config.crit_multiplier,
            damage: skill_config.damage.clone(),
            effects: effects,
            skill_range: skill_config.skill_range,
            skill_direction: skill_config.skill_direction.clone(),
            target_type: skill_config.target_type.clone(),
            projectile_type: skill_config.projectile_type.clone(),
            target_quantity: skill_config.target_quantity,
            area_on_impact: skill_config.area_on_impact,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PassiveSkillDeploy {
    pub skill_type: PassiveSkillType,
    pub trigger_time_frequency: f32,
    pub trigger_chance: u8,
    pub skill_life_time: f32,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effects: HashMap<EffectType, u8>,

    pub skill_range: u8,
    pub skill_direction: SkillDirectionType,
    pub target_type: TargetType,
    pub target_quantity: u8,
    pub area_on_impact: u8, 

    pub projectile_type: Option<ProjectileType>,
}

#[derive(Deserialize, Debug)]
pub struct ActiveSkillDeploy {
    pub skill_type: ActiveSkillType,
    pub cooldown_time: f32,
    
    pub projectile_type: Option<ProjectileType>,
    pub skill_range: u8, // max range; min range = 1;
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: TargetType,
    pub target_quantity: u8,
    pub area_on_impact: u8, 

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effects: HashMap<EffectType, u8>,
    pub passive_skills: HashMap<PassiveSkillType, u8>,
}


pub fn setup_base_skill(deploy: &Deploy, base_skill: &mut ActiveSkill, stats: &StatsComponent, weapon: &Option<Stuff>) {
    if base_skill.skill_type != ActiveSkillType::BaseSkill {
        println!("Try to change not base skill!");
        return;
    }

    let mut new_base_skill = ActiveSkill::new(deploy, &ActiveSkillType::BaseSkill);
    match stats.ability.get(&Ability::CriticalHitChance) {
        Some(v) => new_base_skill.crit_chance += *v,
        None => {},
    }

    match stats.ability.get(&Ability::CriticalHitMultiplier) {
        Some(v) => new_base_skill.crit_multiplier += *v,
        None => {}
    }

    let attack_speed_from_ability = match stats.ability.get(&Ability::AttackSpeed) {      
        Some(v) => *v,
        None => 100,
    };

    match weapon {
        Some(v) => {
            match &v.stuff_type {
                StuffType::Weapon(val) => {
                    new_base_skill.crit_chance += val.critical_hit_chance;
                    new_base_skill.skill_range = val.weapon_range;
                    new_base_skill.crit_multiplier += val.critical_hit_multiplier;
                    new_base_skill.damage.clear();                                                         //setting up new damage from weapon to skill;
                    let mut new_damage = val.damage.clone();
                    update_damage_by_ability(&mut new_damage, &stats.ability);
                    new_base_skill.damage = new_damage;

                    new_base_skill.effects.clear();                                                         //setting up new effects from weapon to skill;
                    for (effect_type, value) in val.effects.iter() {
                        let mut effect = Effect::new(deploy, effect_type);
                        update_over_time_effect_damage_by_ability(&mut effect, &stats.ability);
                        new_base_skill.effects.insert(effect_type.clone(), (effect, *value));
                    }

                    new_base_skill.passive_skills.clear();                                                  //setting up new passive skills from weapon to skill;
                    for (passive_skill_type, chance) in val.passive_skills.iter() {
                        let mut new_passive_skill = PassiveSkill::new(deploy, passive_skill_type);
                        for (_, (effect, _)) in new_passive_skill.effects.iter_mut() {
                            update_over_time_effect_damage_by_ability(effect, &stats.ability);
                        }
                        update_damage_by_ability(&mut new_passive_skill.damage, &stats.ability);
                        new_base_skill.passive_skills.insert(passive_skill_type.clone(), (new_passive_skill, *chance));
                    }
                    let new_cooldown = (val.attack_cooldown - val.attack_cooldown * attack_speed_from_ability / 100) as f32;
                    new_base_skill.cooldown_time = if new_cooldown < MINIMAL_TIME_FOR_COOLDOWN_SKILL {
                        MINIMAL_TIME_FOR_COOLDOWN_SKILL
                    } else {
                        new_cooldown
                    };
                },
                _ => {
                    println!("Wrong weapon! Stuff type is '{:#?}'", v.stuff_type);
                },
            }
        },
        None => {},
    }
}

pub fn update_over_time_effect_damage_by_ability(effect: &mut Effect, ability_storage: &HashMap<Ability, i16>) {
    for over_time_effect in effect.over_time_effect.iter_mut() {
        let damage_multiplier = match ability_storage.get(&get_ability_type_from_damage_type(&over_time_effect.effect_damage_type)) {
            Some(v) => *v,
            None => 0,
        };
        over_time_effect.effect_damage_value += over_time_effect.effect_damage_value * damage_multiplier / 100;
    }
}

pub fn update_damage_by_ability(damage: &mut HashMap<Damage, i16>, ability_storage: &HashMap<Ability, i16>) {
    for (damage_type, value) in damage.iter_mut() {
        match ability_storage.get(&get_ability_type_from_damage_type(damage_type)) {
            Some(v) => *value += *value * *v / 100,
            None => {},
        };
    }
}
