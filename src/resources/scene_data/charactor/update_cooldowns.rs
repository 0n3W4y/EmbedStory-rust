use bevy::prelude::*;

use crate::components::charactor_component::SkillAndEffectComponent;

pub fn update_active_skills_cooldown(mut charactors_query: Query<&mut SkillAndEffectComponent>) {
    let delta_time = 0.1;                                                                                   //this function running with criteria, triggered by 0.1 sec;
    for mut skills in charactors_query.iter_mut() {
        for (_, skill) in skills.active_skills.iter_mut() {
            if skill.on_cooldown {
                skill.current_time_duration += delta_time;
                if skill.current_time_duration >= skill.cooldown_time {
                    skill.on_cooldown = false;
                    skill.current_time_duration = 0.0;
                }
            }
        }

        if skills.base_skill.on_cooldown {
            skills.base_skill.current_time_duration += delta_time;
            if skills.base_skill.current_time_duration >= skills.base_skill.cooldown_time {
                skills.base_skill.on_cooldown = false;
                skills.base_skill.current_time_duration = 0.0;
            }
        }
        /* 
        for(_, effect) in skills.effects.iter_mut() {
            effect.time_duration += delta_time;
            match effect.over_time_effect {
                Some(mut v) => {
                    v.time_duration += delta_time;
                },
                None => {},
            }
        }
        */
    }
}