use serde::{Serialize, Deserialize};
use bevy::prelude::*;

#[derive( Clone, Default, Serialize, Deserialize, Resource)]
pub struct StuffManager;