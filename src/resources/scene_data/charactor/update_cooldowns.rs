use bevy::prelude::*;

use crate::components::charactor_component::SkillComponent;

pub fn update_active_skills_cooldown(mut charactors_query: Query<&mut SkillComponent>) {
    //this function running with criteria, by triggered by 0.1 sec;
    let delta_time = 0.1;
    for mut skills in charactors_query.iter_mut() {
        for (_, skill) in skills.skills.iter_mut() {
            if skill.on_cooldown {
                skill.current_time_duration += delta_time;
                if skill.current_time_duration >= skill.cooldown_time {
                    skill.on_cooldown = false;
                    skill.current_time_duration = 0.0;
                }
            }
        }
    }
}