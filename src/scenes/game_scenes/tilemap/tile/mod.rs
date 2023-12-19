use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default, Copy)]
pub enum TilePermissions{
    RemoveFloor,
    PlaceFloor,
    PlaceThing,
    RemoveThing,
    #[default]
    Walk,
    PlaceStuff,
    Roof,
    Fog,
    PlaceEffect,
    RemoveEffect,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default, Hash)]
pub enum GroundType {
    #[default]
    Earth,
    DryEarth,
    Dirt,
    Rock,
    Clay,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Default, Hash)]
pub enum CoverType {
    #[default]
    None,
    Grass,
    Flowers,
    Water,
    Sand,
    Snow,
    Ice,
    Shallow,
    WoodenFloor,
    RockyRoad,
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug, Copy, Default)]
pub struct Position<T>{
    pub x: T, 
    pub y: T
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Tile{
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub id: usize, // in vec;

    pub ground_graphic_index: u8,
    pub cover_graphic_index: u8,
    pub movement_ratio: u16,

    pub position: Position<i32>,

    pub permissions: Vec<TilePermissions>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TileDeploy {
    pub ground_type: GroundType,
    pub cover_type: CoverType,
    pub movement_ratio: u16,
    pub permissions: Vec<TilePermissions>
}