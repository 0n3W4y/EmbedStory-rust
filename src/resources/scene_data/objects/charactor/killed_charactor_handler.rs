use bevy::prelude::*;

use crate::components::charactor_component::{CharactorComponent, NPCComponent, PlayerComponent, MonsterComponent};
use crate::resources::scene_data::objects::body_part::HealthPoints;
use crate::resources::scene_manager::SceneManager;
use crate::resources::scene_data::objects::body_part::BodyPartType;

pub fn killed_charactor_monster_handler(
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
    mut charactor_query: Query<(Entity, &CharactorComponent), With<MonsterComponent>>
){
    //TODO: change sprite to dead, run timer to despawn creature;
    for (entity, component) in charactor_query.iter_mut(){

        match component.body_structure.get(&BodyPartType::Brain){
            Some(v) => {
                match v.health_points.get(&HealthPoints::Current) {
                    Some(t) => {
                        if *t <= 0 {
                            //do death function;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        };

        match component.body_structure.get(&BodyPartType::Heart){
            Some(v) => {
                match v.health_points.get(&HealthPoints::Current) {
                    Some(t) => {
                        if *t <= 0 {
                            //do death function;
                        }
                    },
                    None => {},
                }
            },
            None => {},
        };

    }
}

pub fn killed_charactor_npc_handler(
    mut commands: Commands,
    mut scene_manager: ResMut<SceneManager>,
    mut charactor_query: Query<(Entity, &CharactorComponent), With<NPCComponent>>
){

}

pub fn killed_charactor_player_handler(
    mut charactor_query: Query<(Entity, &CharactorComponent), With<PlayerComponent>>
){

}