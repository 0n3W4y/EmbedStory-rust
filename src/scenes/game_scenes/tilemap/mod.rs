use serde::{Deserialize, Serialize};

pub mod tile;
pub mod draw;
pub mod cleanup;
pub mod generate;

use self::tile::Tile;


#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Tilemap {
    tile_size: u16,
    tilemap_width: u16,
    tilemap_height: u16,
    tilemap_tile_storage: Vec<Tile>,
    total_tiles: usize,
}

impl Tilemap {
    pub fn set(&mut self, tile_size: u16, width: u16, height: u16) {
        self.tile_size = tile_size;
        self.tilemap_height = height;
        self.tilemap_width = width;
        self.total_tiles = width as usize * height as usize;
    }

    pub fn get_tile_size(&self) -> u16 {
        return self.tile_size;
    }

    pub fn get_tilemap_width(&self) -> u16 {
        return self.tilemap_width;
    }

    pub fn get_tilemap_height(&self) -> u16 {
        return self.tilemap_height;
    }

    pub fn get_total_tiles(&self) -> usize {
        return self.total_tiles;
    }

    pub fn get_tilemap_tile_storage_mut(&mut self) -> &mut Vec<Tile> {
        return &mut self.tilemap_tile_storage;
    }

    pub fn get_tilemap_tile_storage(&self) -> &Vec<Tile> {
        return &&self.tilemap_tile_storage;
    }

    pub fn get_tile_by_index(&self, value: usize) -> &Tile {
        let vector_length = self.tilemap_tile_storage.len();
        if value >= vector_length {
            panic!(
                "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}",
                value, vector_length
            );
        }

        return &self.tilemap_tile_storage[value];
    }

    pub fn get_tile_by_index_mut(&mut self, value: usize) -> &mut Tile {
        let vector_length = self.tilemap_tile_storage.len();
        if value >= vector_length {
            panic!(
                "ground_tilemap::get_tile_by_index. Value > vec.len(); Value:{}, vec.len():{}",
                value, vector_length
            );
        }

        return &mut self.tilemap_tile_storage[value];
    }
}