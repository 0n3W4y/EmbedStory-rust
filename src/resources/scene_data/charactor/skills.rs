use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::resources::{scene_data::{stuff::Stuff, projectiles::ProjectileType, Ability, Damage}, deploy::Deploy};

use super::{effects::{EffectType, Effect}, StuffWearSlot, get_ability_type_from_damage_type};

pub const MINIMAL_TIME_FOR_COOLDOWN_SKILL: f32 = 0.20;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum ActiveSkillType {
    #[default]
    BaseSkill
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum PassiveSkillType {
    #[default]
    ChainLighting
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
    pub skill_life_time: f32,
    pub current_time_duration: f32,
    pub total_duration: f32,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effect: HashMap<EffectType, (Effect, u8)>,

    pub skill_range: u8,
    pub skill_direction: SkillDirectionType,
    pub target_type: TargetType,
    pub target_quantity: u8,                                // max target quantity in skill range;
    pub area_on_impact: u8,                                 //0 - only target, 1 - +1 position for all direction, 2 - +2 position for all direction;

    pub projectiles: u8,
    pub projectile_type: ProjectileType,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ActiveSkill {
    pub skill_type: ActiveSkillType,
    pub is_activated: bool,                                     // activeated skill will start logic to dealt damage to target;
    pub on_cooldown: bool,                                  // can use this skill now;
    pub cooldown_time: f32,
    pub current_time_duration: f32,                            // == 0.0;
    pub stamina_cost: i16,
    
    pub projectiles: u8,
    pub projectile_type: ProjectileType,
    pub skill_range: u8,                                    // max range; min range = 1;
    pub skill_direction: SkillDirectionType,
    pub target: TargetType,
    pub target_quantity: u8,                                // max target quantity in skill range;
    pub area_on_impact: u8,                                 //0 - only target, 1 - +1 position for all direction, 2 - +2 position for all direction;

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effect: HashMap<EffectType, (Effect, u8)>,
    pub passive_skills: HashMap<PassiveSkillType, (PassiveSkill, u8)>,
}

impl ActiveSkill {
    pub fn new (deploy: &Deploy, skill_type: &ActiveSkillType) -> Self {
        let mut effects: HashMap<EffectType, (Effect, u8)> = HashMap::new();
        let mut passive_skills: HashMap<PassiveSkillType, (PassiveSkill, u8)> = HashMap::new();
        let skill_config = deploy.charactor_deploy.skills_deploy.get_active_skill_deploy(skill_type);
        for (effect_type, effect_chance) in skill_config.effect.iter() {
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
            projectiles: skill_config.projectiles,
            projectile_type: skill_config.projectile_type.clone(),
            skill_direction: skill_config.skill_direction.clone(),
            stamina_cost: skill_config.stamina_cost,
            target: skill_config.target.clone(),
            crit_chance: skill_config.crit_chance,
            crit_multiplier: skill_config.crit_multiplier,
            damage: skill_config.damage.clone(),
            effect: effects,
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
        for (effect_type, effect_chance) in skill_config.effect.iter() {
            let effect = Effect::new(deploy, effect_type);
            effects.insert(effect_type.clone(), (effect, *effect_chance ));
        }

        PassiveSkill {
            skill_type: skill_type.clone(),
            trigger_time_frequency: skill_config.trigger_time_frequency,
            skill_life_time: skill_config.skill_life_time,
            current_time_duration: 0.0,
            total_duration: 0.0,
            crit_chance: skill_config.crit_chance,
            crit_multiplier: skill_config.crit_multiplier,
            damage: skill_config.damage.clone(),
            effect: effects,
            skill_range: skill_config.skill_range,
            skill_direction: skill_config.skill_direction.clone(),
            target_type: skill_config.target_type.clone(),
            projectiles: skill_config.projectiles,
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
    pub skill_life_time: f32,

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effect: HashMap<EffectType, u8>,

    pub skill_range: u8,
    pub skill_direction: SkillDirectionType,
    pub target_type: TargetType,
    pub target_quantity: u8,
    pub area_on_impact: u8, 

    pub projectiles: u8,
    pub projectile_type: ProjectileType,
}

#[derive(Deserialize, Debug)]
pub struct ActiveSkillDeploy {
    pub skill_type: ActiveSkillType,
    pub cooldown_time: f32,
    
    pub projectiles: u8,
    pub projectile_type: ProjectileType,
    pub skill_range: u8, // max range; min range = 1;
    pub skill_direction: SkillDirectionType,
    pub stamina_cost: i16,
    pub target: TargetType,
    pub target_quantity: u8,
    pub area_on_impact: u8, 

    pub crit_chance: i16,
    pub crit_multiplier: i16,

    pub damage: HashMap<Damage, i16>,
    pub effect: HashMap<EffectType, u8>,
    pub passive_skills: HashMap<PassiveSkillType, u8>,
}


pub fn update_base_skill_by_changes_in_ability(deploy: &Deploy, base_skill: &mut ActiveSkill, ability_storage: &HashMap<Ability, i16>, wear_stuff: &HashMap<StuffWearSlot, Option<Stuff>>) {
    if base_skill.skill_type != ActiveSkillType::BaseSkill {
        println!("Try to change not base skill!");
        return;
    }

    

    /* 
            let critical_hit_chance_from_ability = match ability_storage.get(&AbilityType::CriticalHitChanse) {     //get critical hit chance from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Chance from ability, use 0 instead");
                    0
                },
            };
            let critical_hit_multiplier_from_ability = match ability_storage.get(&AbilityType::CriticalHitMultiplier) {     //get critical hit multiplier from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Critical Multiplier from ability, use 0 instead");
                    0
                }
            };
            let attack_speed_from_ability = match ability_storage.get(&AbilityType::AttackSpeed) {      //get atk speed from ability;
                Some(v) => *v,
                None => {
                    println!("Can not get Atack Speed from ability, use 100% instead");
                    100
                }
            };

            let mut skill_cooldown: i16;
            let mut critical_chance: i16 = 0;
            let mut critical_multiplier: i16 = 0;
            let mut damage_from_weapon: HashMap<DamageType, i16> = HashMap::new();
            let mut effects_from_weapon: HashMap<EffectType, u8> = HashMap::new();
            let mut extra_skills_from_weapon: HashMap<SkillType, u8> = HashMap::new();
            let mut skip_left_hand: bool = false;             //check for TwoHanded weapon;

            match wear_stuff.get(&StuffWearSlot::RightHand).unwrap() {          //get weapon from right hand
                Some(weapon) => {
                    critical_chance = weapon.critical_hit_chance;
                    critical_multiplier = weapon.critical_hit_multiplier;
                    skill_cooldown = weapon.attack_cooldown;
                    damage_from_weapon = weapon.damage.clone();
                    effects_from_weapon = weapon.effects.clone();
                    extra_skills_from_weapon = weapon.extra_skills.clone();
                    if weapon.wear_slot.unwrap() == StuffWearSlot::RightAndLeftHand {                        //safe unwrap;
                        skip_left_hand = true;
                    }

                },
                None => {
                    skill_cooldown = 100;                                                                       // by default 1 shot per second;
                    damage_from_weapon.insert(DamageType::Phisical, 5);                                 // default punch;
                },
            };

            if !skip_left_hand {
                match wear_stuff.get(&StuffWearSlot::LeftHand).unwrap() {                                       //get weapon from left hand
                    Some(weapon) => {
                        critical_chance = critical_chance + weapon.critical_hit_chance;                       //middle value from 2 weapons;
                        critical_multiplier = critical_multiplier + weapon.critical_hit_multiplier;           //middle value from 2 weapons;
                        skill_cooldown = skill_cooldown + weapon.attack_cooldown;                                //middle value from 2 weapons;
    
                        for (damage_type, value) in weapon.damage.iter() {                     //stocking damage values into 1 hashmap
                            damage_from_weapon.entry(damage_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
    
                        for(effect_type, value) in weapon.effects.iter() {                      //stocking effects into 1 hashmap;
                            effects_from_weapon.entry(effect_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
    
                        for(skill_type, value) in weapon.extra_skills.iter() {                           //stocking passive skills into 1 hashmap;
                            extra_skills_from_weapon.entry(skill_type.clone()).and_modify(|x| {*x += *value}).or_insert(*value);
                        }
                    },
                    None => {},
                };
            }            

            let interim_cooldown = (skill_cooldown as f32 / 100.0) - ((skill_cooldown * attack_speed_from_ability) as f32 / 100.0);               //calculate cooldown value;
            skill.cooldown_time = if interim_cooldown >= MINIMAL_TIME_FOR_COOLDOWN_SKILL {          //check for cooldown value;
                MINIMAL_TIME_FOR_COOLDOWN_SKILL
            } else {
                interim_cooldown
            };

            skill.crit_chance = critical_chance + critical_hit_chance_from_ability;             //set to skill crit chance;
            skill.crit_multiplier = critical_multiplier + critical_hit_multiplier_from_ability;             //set to skill crit multiplier;


            for (damage_type, value) in damage_from_weapon.iter() {             //collect damage from abilities and calculate new values;
                let ability_type = get_ability_type_from_damage_type(damage_type);
                let damage_multiplier_from_ability = match ability_storage.get(&ability_type){
                    Some(v) => *v,
                    None => 0,
                };
                let new_value = *value + (*value * damage_multiplier_from_ability / 100);
                skill.damage.insert(damage_type.clone(), new_value);            //insert new damage values into skill;
            }

            skill.effect = effects_from_weapon;
            */
}

