use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::scene_data::projectiles::{ProjectileConfig, ProjectileType};

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectileDeploy {
    pub arrow: ProjectileConfig,
    pub bullet: ProjectileConfig,
    pub fire_sphere: ProjectileConfig,
}

impl ProjectileDeploy {
    pub fn new() -> Self {
        let projectile_path = "deploy/projectiles.json";
        let projectile_data: ProjectileDeploy  = match File::open( projectile_path ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, projectile_path ),
        };

        return projectile_data;
    }

    pub fn get_config(&self, projectile_type: &ProjectileType) -> &ProjectileConfig {
        match *projectile_type {
            ProjectileType::Arrow => &self.arrow,
            ProjectileType::Bullet => &self.bullet,
            ProjectileType::FireSphere => &self.fire_sphere,
            ProjectileType::None => panic!("Try to get empty projectileconfig!!!"),
        }
    }
}