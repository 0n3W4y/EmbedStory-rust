use bevy::prelude::*;

use crate::components::{charactor_component::{SkillAndEffectComponent, CharactorTargetComponent, CharactorComponent}, StatsComponent};


pub fn active_skill_handler (
    charactors_query: Query<(&mut SkillAndEffectComponent, &mut StatsComponent, &mut CharactorTargetComponent), With<CharactorComponent>>
){

}