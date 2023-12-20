use bevy::prelude::*;

use crate::{components::{charactor_component::{CharactorComponent, AbilityComponent}, AttributesComponent}, resources::scene_data::{AbilityType, Attribute}};

use super::{CharactorStatus, change_attribute_points};

pub fn update_health_and_stamina_regen(
    mut charactors: Query<(&CharactorComponent, &mut AttributesComponent, &AbilityComponent), With<CharactorComponent>>
) {
    //this funcion updates 1 per 10 seconds;
    for (charactor, mut attributes, abilities) in charactors.iter_mut(){
        if charactor.status == CharactorStatus::Dead {
            continue;
        }

        let regened_health = match abilities.ability.get(&AbilityType::HealthRegen) {
            Some(v) => *v,
            None => 0,
        };

        let regened_stamina = match abilities.ability.get(&AbilityType::StaminaRegen) {
            Some(v) => *v,
            None => 0,
        };

        change_attribute_points(&mut attributes, &Attribute::Health, regened_health, false);
        change_attribute_points(&mut attributes, &Attribute::Stamina, regened_stamina, false);        
    }
}