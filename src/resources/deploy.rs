use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

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