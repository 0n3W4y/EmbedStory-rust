use bevy::prelude::*;

use crate::{components::charactor_component::{CharactorComponent, SkillComponent, AbilityComponent, InventoryComponent, PlayerComponent, MonsterComponent, ExtraStatsComponent, ResistsComponent, EffectComponent, CompanionComponent, PositionComponent}, resources::deploy::Deploy};

use super::{CharactorStatus, skills::Skill};

pub fn player_attacking(
    mut player_query: Query<(&CharactorComponent, &mut SkillComponent, &AbilityComponent, &InventoryComponent, &PositionComponent), With<PlayerComponent>>,
    mut monsters_query: Query<(&CharactorComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent, &PositionComponent), With<MonsterComponent>>,
    deploy: Res<Deploy>,
) {
    let (
        player, 
        mut player_skill,
        player_ability,
        player_inventory,
        player_position
    ) = player_query.single_mut();
    
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
        monster,  
        mut monster_extra_stats,
        monster_resist,
        mut monster_effect,
        monster_position
    ) in monsters_query.iter_mut() {
        if monster.id == player_target_id {
            try_to_attack(
                player_position,
                monster_position,
                &mut player_skill,
                &player_ability,
                &monster_resist,
                &mut monster_extra_stats,
                &mut monster_effect
            );
            break;                
        };
    }

    

    
}

pub fn companion_attacking(
    mut companion_query: Query<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<CompanionComponent>>,
    mut monsters_query: Query<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<MonsterComponent>>,
) {
    let (
        companion, 
        companion_skill, 
        mut companion_extra_stats,
        companion_resist,
        mut companion_effect,
    ) = companion_query.single_mut();

    let companion_target_id: usize = if companion.status == CharactorStatus::Attacking {
        match companion.target {
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
    mut monsters_query: Query<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<MonsterComponent>>,
    mut companion_query: Query<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<CompanionComponent>>,
    mut player_query: Query<(&CharactorComponent, &SkillComponent, &mut ExtraStatsComponent, &ResistsComponent, &mut EffectComponent), With<PlayerComponent>>,
){}

fn try_to_attack(
    position: &PositionComponent,
    target_position: &PositionComponent,
    skill_component: &mut SkillComponent, 
    ability_component: &AbilityComponent, 
    target_resist_component: &ResistsComponent, 
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
){
    //get attacking skill;
    let mut skill = match skill_component.skills.get(&1) {
        Some(mut v) => v,
        _ => &mut Skill{..Default::default()}
    };

    // check for default skill
    if skill.base_cooldown == 0 {
        println!("can't attacking target, because autoattack skill not found in skills storage");
        return;
    };

    //check for position;
    let skill_range = skill.range;
    let diff_x = (position.position.x.abs() - target_position.position.x.abs()).abs(); // always positive value;
    let diff_y = (position.position.y.abs() - target_position.position.y.abs()).abs(); // always positive value;
    let diff = diff_x.max(diff_y);
    if skill_range as i32 >= diff {
        if skill.current_duration == 0.0 {
            //if all finem we attack;
            attack(skill_component, ability_component, target_resist_component, target_extra_stats, target_effect);
        }
    }
}

fn attack(
    skill_component: &mut SkillComponent, 
    ability_component: &AbilityComponent, 
    target_resist_component: &ResistsComponent, 
    target_extra_stats: &mut ExtraStatsComponent,
    target_effect: &mut EffectComponent,
) {   


}