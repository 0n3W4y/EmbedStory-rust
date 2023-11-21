use bevy::prelude::*;

use crate::components::{IdentificationComponent, PositionComponent, AttributesComponent, StatsComponent, ResistsComponent};
use crate::components::charactor_component::{CharactorComponent, SkillComponent, EffectComponent, AbilityComponent, InventoryComponent, DestinationComponent};
use crate::resources::scene_manager::SceneManager;
use crate::resources::profile::Profile;
use crate::resources::scene_data::charactor::{Charactor, CharactorType};

pub fn cleanup(
    mut commands: Commands,
    mut charactor_query: Query<(
        Entity,
        &IdentificationComponent,
        &CharactorComponent,
        &ResistsComponent,
        &SkillComponent,
        &PositionComponent,
        &DestinationComponent,
        &EffectComponent,
        &StatsComponent,
        &AttributesComponent,
        &AbilityComponent,
        &InventoryComponent,
    ), With<CharactorComponent>>,
    mut scene_manager: ResMut<SceneManager>,
    mut profile: ResMut<Profile>,
){
    let scene = scene_manager.get_current_game_scene_mut();
    scene.charactors.companion.clear();
    scene.charactors.monster.clear();
    scene.charactors.player.clear();
    scene.charactors.npc.clear();

    for (
        entity, 
        identification_component,
        charactor_component, 
        resist_component, 
        skill_component, 
        position_component,
        destination_component,
        effect_component,
        stats_component,
        attributes_component,
        ability_component,
        inventory_component,
    ) in charactor_query.iter_mut(){
        let mut charactor = Charactor::default();
        copy_from_component_to_charactor(
            &mut charactor, 
            identification_component,
            charactor_component, 
            resist_component, 
            skill_component, 
            position_component,
            destination_component,
            effect_component, 
            stats_component, 
            attributes_component,
            ability_component,
            inventory_component
        );
        match charactor_component.charactor_type {
            CharactorType::Player => {
                profile.charactor = Some(charactor);
            },
            CharactorType::Companion => {
                profile.companion = Some(charactor);
            },
            CharactorType::Monster | CharactorType::NPC => {
                scene.charactors.store(charactor);
            },
        }
        commands.entity(entity).despawn_recursive();
    }    
}

pub fn copy_from_component_to_charactor(
    charactor: &mut Charactor,
    identefication_component: &IdentificationComponent,
    charactor_component: &CharactorComponent,
    resist_component: &ResistsComponent,
    skill_component: &SkillComponent,
    position_component: &PositionComponent,
    destination_component: &DestinationComponent,
    effect_component: &EffectComponent,
    stats_component: &StatsComponent,
    attributes_component: &AttributesComponent,
    ability_component: &AbilityComponent,
    inventory_component: &InventoryComponent,
){
    charactor.id = identefication_component.id;
    charactor.charactor_type = charactor_component.charactor_type.clone();
    charactor.race_type = charactor_component.race_type.clone();
    charactor.gender_type = charactor_component.gender_type.clone();
    charactor.status = charactor_component.status.clone();
    //charactor.fraction = charactor_component.fraction.clone();
    charactor.level = charactor_component.level;
    charactor.experience = charactor_component.experience;

    charactor.resists = resist_component.resists.clone();

    charactor.skills = skill_component.skills.clone();
    charactor.passive_skills = skill_component.passive_skills.clone();

    charactor.position = position_component.position.clone();
    charactor.destination_direction = destination_component.destination_direction.clone();
    charactor.destination_path = destination_component.destination_path.clone();
    charactor.destination_point = destination_component.destination_point.clone();

    charactor.effects = effect_component.effects.clone();

    charactor.stats = stats_component.stats.clone();
    charactor.stats_cache = stats_component.stats_cache.clone();

    charactor.attributes = attributes_component.attributes.clone();
    charactor.attributes_cache = attributes_component.attributes_cache.clone();

    charactor.ability = ability_component.ability.clone();

    charactor.stuff_storage = inventory_component.stuff_storage.clone();
    charactor.stuff_wear = inventory_component.stuff_wear.clone();
    charactor.stuff_storage_max_slots = inventory_component.stuff_storage_max_slots;
}