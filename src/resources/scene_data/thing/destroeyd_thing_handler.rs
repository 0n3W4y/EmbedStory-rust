use bevy::prelude::*;

use crate::components::PositionComponent;
use crate::components::thing_component::{ThingComponent, ThingStatsComponent};
use crate::resources::scene_data::charactor::stats::Stat;
use crate::resources::scene_data::stuff::Stuff;
use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;

use super::ThingType;

pub fn destroeyd_thing_handler(
    mut commands: Commands,
    mut things_query: Query<(Entity, &ThingComponent, &ThingStatsComponent, &PositionComponent), (Changed<ThingComponent>, With<ThingComponent>)>,
    mut scene_manager: ResMut<SceneManager>,
    material_manager: Res<MaterialManager>,
){
    for (entity, thing_component, thing_stats, position_component) in things_query.iter_mut(){
        //TODO: Create animation timer, then despawn entity and create new;

        if *thing_stats.stats.get(&Stat::HealthPoints).unwrap() <= 0 { //check for destroy
            //despawn curent thing, and spawn stuff or something what should be spawn after death;
            let new_stuff: Option<Stuff> = match thing_component.thing_type {
                ThingType::Tree
                | ThingType::FertileTree
                | ThingType::Bush
                | ThingType::FertileBush
                | ThingType::Log => {
                    //create new thing, remove old thing from scene; remove destroyed thing from tile; add new thing on tile and permissions if needed;
                    None
                },

                ThingType::Rock => {
                    //create new thing, remove old thing from scene; remove destroyed thing from tile; add new thing on tile and permissions!!!!! 
                    //change cover type to rock_envirounment;
                    None
                },
                ThingType::Boulder => {
                    None
                },

                ThingType::IronOre => {
                    None
                },
                ThingType::CopperOre => {
                    None
                },

                _ => None,
            };
            let thing_position = position_component.position.clone();
            commands.entity(entity).despawn_recursive();

            match new_stuff {
                Option::Some(v) => {
                    //spawn new stuff;
                },
                None => { 
                    //do nothing;
                },
            };
        }
    }
}