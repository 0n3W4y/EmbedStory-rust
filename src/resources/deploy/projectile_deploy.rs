use std::{fs::File, io::Read};

use serde::Deserialize;

use crate::resources::scene_data::projectiles::{ProjectileConfig, ProjectileType};

use super::DEPLOY_PROJECTILE_PATH;

#[derive(Deserialize, Debug, Clone)]
pub struct ProjectileDeploy {
    pub arrows: ArrowsDeploy,
    pub bullets: BulletsDeploy,
    pub spheres: SpheresDeploy,
}

impl ProjectileDeploy {
    pub fn new() -> Self {
        let projectile_data: ProjectileDeploy  = match File::open( DEPLOY_PROJECTILE_PATH ){
            Ok( mut file ) => {
                let mut contents = String::new();
                file.read_to_string( &mut contents ).unwrap();
                serde_json::from_str( &contents ).expect( "JSON was not well-formatted" )
            }
            Err( err ) => panic!( "Can not open cover data file: {}, {}", err, DEPLOY_PROJECTILE_PATH ),
        };

        return projectile_data;
    }

    pub fn get_config(&self, projectile_type: &ProjectileType) -> &ProjectileConfig {
        match *projectile_type {
            ProjectileType::Arrow => &self.arrows.arrow,
            ProjectileType::Bullet => &self.bullets.bullet,
            ProjectileType::FireSphere => &self.spheres.fire_sphere,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ArrowsDeploy {
    pub arrow: ProjectileConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BulletsDeploy {
    pub bullet: ProjectileConfig,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SpheresDeploy {
    pub fire_sphere: ProjectileConfig,
}