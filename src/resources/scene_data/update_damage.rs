use bevy::prelude::*;
use rand::Rng;

use crate::{components::{
    charactor_component::{AbilityComponent, EffectComponent, SkillComponent},
    tile_component::TileComponent,
    AttributesComponent, IdentificationComponent, ObjectType, TakenDamageComponent, DamageTextInformerComponent, ResistsComponent,
}, resources::scene_data::Ability};

use super::{damage_text_informer::DamageTextInformer, Resist, charactor};

pub fn update_damage(
    mut objects_query: Query<
        (
            &IdentificationComponent,
            &mut AttributesComponent,
            &mut EffectComponent,
            &mut SkillComponent,
            &AbilityComponent,
            &ResistsComponent,
            &mut TakenDamageComponent,
            &mut DamageTextInformerComponent,
        ),
        Without<TileComponent>,
    >,
) {
    for (
        identification, 
        mut attributes, 
        mut effects,
        mut skills, 
        abilities,
        resists,
        mut damage,
        mut damage_text_informer,
        ) in objects_query.iter_mut()
    {
        let mut random = rand::thread_rng();
        match identification.object_type {
            ObjectType::Charactor(_) => {
                for taken_damage in damage.damage.iter(){
                    let chance_to_evade = match abilities.ability.get(&Ability::Evasion) {
                        Some(v) => *v,
                        None => 0,
                    };
                    let random_number_for_evade_chance = random.gen_range(0..100);
                    if chance_to_evade > random_number_for_evade_chance {
                        let damage_text = DamageTextInformer::new("Evaded".to_string(), false, None);
                        damage_text_informer.text.push(damage_text);
                        return;                                                                                                     //damage evaded
                    }
                    
                    let block_amount = match abilities.ability.get(&Ability::BlockChance) {
                        Some(v) => {
                            match abilities.ability.get(&Ability::BlockAmount) {
                                Some(a) => {
                                    let random_chace_to_block = random.gen_range(0..100);
                                    if *v > random_chace_to_block {
                                        *a
                                    } else {
                                        0
                                    }
                                },
                                None => 0,
                            }
                        },
                        None => 0,
                    };

                    for (damage_type, value) in taken_damage.damage.iter() {
                        let resist = match resists.resists.get(&Resist::damage(&damage_type)) {
                            Some(v) => *v,
                            None => 0,
                        };
                        let blocked_value = *value - *value * block_amount / 100;
                        let overall_value = blocked_value - blocked_value * resist / 100;
                        charactor::change_attribute_points(&mut attributes, damage_type, overall_value, false);
                        let text_informer = DamageTextInformer::new(overall_value.to_string(),taken_damage.is_critical_hit, Some(damage_type));
                        damage_text_informer.text.push(text_informer);
                    }

                    for effect in taken_damage.effects.iter() {
                        match effects.effect_immunes.iter().find(|&x| *x == effect.effect_type) {
                            Some(_) => continue,                                                                                    //ignore effect;
                            None => {
                                match abilities.ability.get(&Ability::ReducingEffectTime) {
                                    Some(v) => {
                                        effect.effect_lifetime -= effect.effect_lifetime * *v as f32 / 100.0;
                                    },
                                    None => {},
                                };
                                
                                match effects.effects.get_mut(&effect.effect_type) {                                           //get effect if it already in; prolong lifetime effect, and replace with new effect
                                    Some(v) => {
                                        effect.effect_lifetime += v.effect_lifetime;
                                        match v.over_time_effect {
                                            Some(over_time_effect) => {
                                                let time_duration = over_time_effect.time_duration;
                                                match effect.over_time_effect {
                                                    Some(mut val) => {
                                                        val.time_duration += time_duration;
                                                    },
                                                    None => {},
                                                }
                                            },
                                            None => {},
                                        }
                                        *v = effect.clone();
                                    },
                                    None => {
                                        for effect_status in effect.effect_status.iter(){                               //store effect status to charactor effect status;
                                            effects.effect_status.push(effect_status.clone());
                                        }
                                        effects.effects.insert(effect.effect_type.clone(), effect.clone());
                                    },
                                }
                            }
                        }
                    }

                    for passive_skill in taken_damage.passive_skills.iter() {
                        todo!();
                        
                    }
                }
                damage.damage.clear();
            },
            _ => {},
        }
    }
}
