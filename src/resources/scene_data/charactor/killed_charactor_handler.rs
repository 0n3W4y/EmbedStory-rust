use bevy::prelude::*;

use crate::components::charactor_component::CharactorComponent;

use super::CharactorStatus;

pub fn killed_charactor_handler(
    mut charactor_query: Query<(
        &mut CharactorComponent, 
        &mut TextureAtlasSprite,
    ), With<CharactorComponent>>,
){
    //TODO: change sprite to dead, run timer to despawn creature;
    for (mut component, mut sprite) in charactor_query.iter_mut(){
        if component.status == CharactorStatus::Dead {
            continue;
        };

        match component.body_structure.get(&BodyPartType::Brain){
            Some(v) => {
                match v.health_points.get(&HealthPoints::Current) {
                    Some(t) => {
                        if *t <= 0 {
                            sprite.index = 9;
                            component.status = CharactorStatus::Dead;
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
                            sprite.index = 9;
                            component.status = CharactorStatus::Dead;
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