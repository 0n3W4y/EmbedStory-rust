use bevy::prelude::*;

use crate::{components::charactor_component::{CharactorComponent, SkillComponent, AbilityComponent, InventoryComponent, PlayerComponent, MonsterComponent, ExtraStatsComponent, ResistsComponent, EffectComponent}, resources::deploy::Deploy};

use super::CharactorStatus;

pub fn player_attacking(
    player_queue: Query<(&CharactorComponent, &mut SkillComponent, &AbilityComponent, &InventoryComponent), With<PlayerComponent>>,
    monsters_queue: Query<(&CharactorComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<MonsterComponent>>,
    time: Res<Time>,
    deploy: Res<Deploy>,
) {
    let delta = time.delta_seconds();
    let (
        player, 
        mut player_skill,
        player_ability,
        player_inventory,
    ) = player_queue.single_mut();
    
    let player_target_id: usize = if player.status == CharactorStatus::Attacking {
        match player.target {
            Some(v) => v,
            _ => {
                println!("Player has no target, but status Attacking!");
                0
            }
        }
    } else {
        0
    };

    for (
        monster_component,  
        mut monster_extra_stats_component,
        monster_resist_component,
        mut monster_effect_component
    ) in monsters_queue.iter_mut() {
        if monster_component.id == player_target_id {
            try_to_attack(&mut player_skill, &player_ability, &monster_resist, &mut monster_extra_stats, &mut monster_effect, delta);
            break;                
        };
    }

    

    
}

pub fn companion_attacking(
    companion_queue: Queue<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<CompanionComponent>>,
    monsters_queue: Queue<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<MonsterComponent>>,
) {
    let (
        companion_component, 
        companion_skill_component, 
        mut companion_extra_stats_component,
        companion_resist_component,
        mut companion_effect_component
    ) = companion_queue.single_mut();

    let companion_target_id: usize = if companion_component.status == CharactorStatus::Attacking {
        match companion_component.target {
            Some(v) => v,
            _ => {
                println!("Companion has no target, but status Attacking!");
                0
            }
        }
    } else {
        0
    };
}
pub fn monster_attacking(
    monsters_queue: Queue<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<MonsterComponent>>,
    companion_queue: Queue<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<CompanionComponent>>,
    player_queue: Queue<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<PlayerComponent>>,
){}


fn try_to_attack(
    skill_component: &mut SkillComponent, 
    ability_component: &AbilityComponent, 
    target_resist_component: &ResistsComponent, 
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
    delta: f32,
) {
    let skill = match skill_component.skills.get(1) {
        Some(v) => *v,
        _ => Skill::Default()
    };

    // check for default skill
    if skill.base_cooldown == 0 {
        println!("can't attacking target, because autoattack skill not found in skills storage");
        return;
    };

    skill.current_duration += delta;
    if skill.current_cooldown >= skill.current_duration


}