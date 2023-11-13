use bevy::prelude::*;

use crate::components::charactor_component::{SkillComponent, AbilityComponent, CharactorTargetComponent, CharactorComponent};


pub fn active_skill_handler (
    charactors_query: Query<(&mut SkillComponent, &mut AbilityComponent, &mut CharactorTargetComponent), With<CharactorComponent>>
){

}