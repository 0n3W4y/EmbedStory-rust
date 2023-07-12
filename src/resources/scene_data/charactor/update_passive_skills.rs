use bevy::prelude::*;

use crate::components::charactor_component::SkillComponent;

pub fn update_passive_skills(
    mut skills_query: Query<&mut SkillComponent>,
    time: Res<Time>,    
) {
    let delta = time.delta_seconds();
    for mut skill_component in skills_query.iter_mut(){
        for skill in skill_component.skills.iter_mut(){

        }
    }
}