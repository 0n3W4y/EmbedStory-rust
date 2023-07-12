use bevy::prelude::*;

use crate::components::charactor_component::SkillComponent;

use super::skills::Skill;

pub fn update_skills_cooldown(mut charactors_query: Query<&SkillComponent>, time: Res<Time>) {
    let delta = time.delta_seconds();
    for skills in charactors_query.iter_mut() {
        for (_, skill) in skills.skills.iter_mut() {
            if skill.current_duration == 0.0 {
                continue;
            } else {
                do_cooldown(skill, delta);
            }
        }
    }
}

fn do_cooldown(skill: &mut Skill, delta: f32) {
    skill.current_duration -= delta;
    if skill.current_duration <= 0.0 {
        skill.current_duration = 0.0;
    };
}
