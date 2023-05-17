use bevy::prelude::*;

use crate::components::charactor_component::CharactorComponent;
use crate::resources::scene_manager::SceneManager;
use crate::resources::profile::Profile;
use crate::resources::scene_data::objects::charactor::{Charactor, CharactorType};

pub fn cleanup(
    mut commands: Commands,
    mut charactor_query: Query<(Entity, &CharactorComponent), With<CharactorComponent>>,
    mut scene_manager: ResMut<SceneManager>,
    mut profile: ResMut<Profile>,
){
    let scene = scene_manager.get_current_game_scene_mut();
    for (entity, component) in charactor_query.iter_mut(){
        if component.charactor_type == CharactorType::Player {
            copy_from_component_to_charactor(&mut profile.charactor, component);
        }else{
            let charactor_id = component.id;
            let charactor = match scene.get_charactor_by_id_mut(charactor_id) {
                Some(v) => v,
                None => {
                    println!("Can't find charactor with id '{}'. So i create default()", charactor_id );
                    let new_charactor: Charactor = Charactor {id: charactor_id, ..Default::default()};
                    scene.charactors.push(new_charactor);
                    let index = scene.charactors.len();
                    &mut scene.charactors[index -1]
                },
            };
            copy_from_component_to_charactor(charactor, component);
        }
        commands.entity(entity).despawn_recursive();
    }    
}

pub fn copy_from_component_to_charactor(
    charactor: &mut Charactor,
    charactor_component: &CharactorComponent,
){
    charactor.id = charactor_component.id;
    charactor.charactor_type = charactor_component.charactor_type.clone();
    charactor.attitude_to_player = charactor_component.attitude_to_player.clone();
    charactor.fraction = charactor_component.fraction.clone();
    charactor.race_type = charactor_component.race_type.clone();

    charactor.position = charactor_component.position.clone();
    charactor.destination_point = charactor_component.destination_point.clone();
    charactor.graphic_position = charactor_component.graphic_position.clone();

    charactor.resists = charactor_component.resists.clone();
    charactor.resists_cache = charactor_component.resists_cache.clone();
    charactor.resist_min_value = charactor_component.resist_min_value;
    charactor.resist_max_value = charactor_component.resist_max_value;

    charactor.stats = charactor_component.stats.clone();
    charactor.stats_cache = charactor_component.stats_cache.clone();
    charactor.stat_min_value = charactor_component.stat_min_value;

    charactor.skills = charactor_component.skills.clone();
    charactor.skills_cache = charactor_component.skills_cache.clone();

    charactor.stuff_storage = charactor_component.stuff_storage.to_vec();
    charactor.stuff_storage_max_slots = charactor_component.stuff_storage_max_slots;
    charactor.stuff_wear = charactor_component.stuff_wear.clone();

    //charactor.charactor_effect: Vec<CharactorEffect>,

    charactor.body_structure = charactor_component.body_structure.clone();
    charactor.current_health_points = charactor_component.current_health_points;
    charactor.total_health_points = charactor_component.total_health_points;
}