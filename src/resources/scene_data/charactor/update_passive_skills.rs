use bevy::prelude::*;
use rand::Rng;

use crate::components::charactor_component::{SkillComponent, PositionComponent, CharactorComponent};

pub fn update_passive_skills(
    mut skills_query: Query<(&CharactorComponent, &mut SkillComponent, &PositionComponent)>,
    time: Res<Time>,    
) {
    let delta = time.delta_seconds();
    let mut rng = rand::thread_rng();
    for (charactor_component, mut skill_component, position_component) in skills_query.iter_mut(){
        for (skill_type, skill) in skill_component.passive_skills.iter_mut(){
            let trigger_time = skill.trigger_time;
            let trigger_chance = skill.trigger_chanse;
            let current_duration = skill.current_duration;
            //check for trigger time;
            if current_duration < trigger_time {
                //add time;
                skill.current_duration += delta;
                continue;
            } else {
                //update time;
                skill.current_duration -= skill.trigger_time;
            }

            //check for trigger chance 
            if trigger_chance < 100 {
                let trigger_chance_random_number: u8 = rng.gen_range(0..=99);
                if trigger_chance < trigger_chance_random_number {
                    //not triggered
                    continue;
                }
            }

            if skill.projectiles > 0 {
                todo!();
            } else {
                //AOE skill
                if skill.range == 0 {
                    //self buff or debuff skill

                } else {
                    // AOE Aura
                    
                }
            }

            //check for effects on self
        }
    }
}