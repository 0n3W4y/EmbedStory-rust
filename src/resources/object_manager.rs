use crate::scenes::game_scenes::game_ground_scene::GameGroundScene;

pub struct ObjectManager{
    id: usize,
}

impl ObjectManager{
    pub fn new() -> Self{
        return ObjectManager{
            id: 0,
        };
    }

    pub fn generate_things_for_scene( scene: &mut GameGroundScene ){
        
    }

    pub fn set_id( &mut self, id:usize ){
        self.id = id;
    }

    fn create_id( &mut self ) -> usize{
        let id = self.id;
        self.id += 1;
        return id;
    }
}