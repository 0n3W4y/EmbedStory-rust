pub struct ObjectManager{
    id: usize,
}

impl ObjectManager{
    pub fn new() -> Self{
        return ObjectManager{
            id: 0,
        };
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