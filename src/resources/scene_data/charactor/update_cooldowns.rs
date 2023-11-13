use bevy::prelude::*;

use crate::components::charactor_component::SkillComponent;

use super::skills::Skill;

pub fn update_active_skills_cooldown(mut charactors_query: Query<&SkillComponent>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for skills in charactors_query.iter_mut() {
        for (_, skill) in skills.skills.iter_mut() {
            if skill.on_cooldown {
                skill.current_time_duration += delta;
                check_for_cooldown_ends(skill);
            }
        }
    }
}

fn check_for_cooldown_ends(skill: &mut Skill) {
    if skill.current_time_duration >= skill.cooldown_time {
        skill.on_cooldown = false;
        skill.current_time_duration = 0.0;
    }
}
