use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileDeployData{
    ground_tile_data: GroundTilemapTileGroundDeployData,
    cover_tile_data: GroundTilemapTileCoverDeployData,
}

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileCoverDeployData{

}

#[derive( Deserialize, Debug )]
pub struct GroundTilemapTileGroundDeployData{
    earth:
}

#[derive( Serialize, Deserialize, Debug )]
pub struct Deploy{

}

impl Deploy{

}

impl FromWorld for Deploy{
    fn from_world( world: &mut World ) -> Self {
        return Deploy{};
    }
}