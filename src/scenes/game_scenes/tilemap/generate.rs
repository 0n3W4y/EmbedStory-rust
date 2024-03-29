use rand::Rng;
use std::collections::HashMap;
use std::vec;


use crate::resources::deploy::game_scene_biome_deploy::{
    Biome, BiomeType, RiverSetting, RiverType, Rivers, SpotSetting, Spots,
};
use crate::resources::deploy::Deploy;
use crate::scenes::game_scenes::tilemap::Tilemap;
use crate::scenes::game_scenes::tilemap::tile::{GroundType, CoverType, Tile, Position, TileDeploy};

use super::tile::TilePermissions;


pub fn generate_tilemap(
    tilemap: &mut Tilemap, 
    deploy: &Deploy, 
    biome_type: &BiomeType
) {
    if tilemap.tile_size == 0 || tilemap.total_tiles == 0 {
        panic!("ground_tilemap::generate_tilemap. Tilemap not setted yet!");
    }

    let biome_setting: &Biome = deploy.game_scene_biome.get_biome_setting(&biome_type);

    generate_ground(tilemap, &biome_setting.main_ground, deploy);
    generate_additional_ground(tilemap, &biome_setting.additional_ground, deploy);

    generate_cover(
        tilemap,
        &biome_setting.main_cover,
        biome_setting.main_cover_filling,
        deploy,
    );
    generate_additional_cover(tilemap, &biome_setting.additional_cover, deploy);

    generate_solids_liquids(tilemap, &biome_setting.spots, &biome_setting.rivers, deploy);

    generate_environment_for_water_cover(tilemap, deploy, 2);
    generate_environment_for_rock_ground(tilemap, deploy, 2);
    spread_indexes_for_cover_tiles(tilemap);
}

fn generate_ground(
    tilemap: &mut Tilemap, 
    ground_type: &GroundType, 
    deploy: &Deploy
) {
    let tile_setting = deploy.tile.get_ground_tile_deploy(&ground_type);
    let tilemap_height = tilemap.tilemap_height as i32;
    let tilemap_width = tilemap.tilemap_width as i32;
    let half_tilmap_width = (tilemap_width / 2) as i32;
    let half_tilmap_height = (tilemap_height / 2) as i32;

    for i in 0..tilemap.tilemap_height {
        for j in 0..tilemap.tilemap_width {
            let x: i32 = -half_tilmap_width + j as i32;
            let y: i32 = -half_tilmap_height + i as i32;
            let id = i as usize * tilemap_height as usize + j as usize;

            let mut tile = Tile {
                id,
                ground_type: tile_setting.ground_type.clone(),
                cover_type: tile_setting.cover_type.clone(),
                position: Position { x, y },
                movement_ratio: tile_setting.movement_ratio,
                permissions: tile_setting.permissions.to_vec(),
                ..Default::default()
            };

            //added walk permission for "rock environment" not for thing ROCK;
            if *ground_type == GroundType::Rock {
                tile.permissions.push(TilePermissions::Walk);
            }

            tilemap.tilemap_tile_storage.push(tile);
        }
    }
}

fn generate_additional_ground(
    tilemap: &mut Tilemap,
    additional_ground: &HashMap<GroundType, f32>,
    deploy: &Deploy,
) {
    let mut rng = rand::thread_rng();
    for (key, value) in additional_ground.iter() {
        let percent: f32 = *value;
        let ground_type = key.clone();
        let mut remain_tiles: usize = (tilemap.total_tiles as f32 * percent / 100.0) as usize; //how manu tiles need to be created;
        let mut max_width = (tilemap.tilemap_width * 5 / 100) as u16; // 5% of tilemap width;
        if max_width < 5 {
            max_width = 5;
        }; // min value;
        let mut max_height: u16 = (tilemap.tilemap_height * 5 / 100) as u16; // 5% of tilemap height;
        if max_height < 5 {
            max_height = 5;
        }; // min value;

        //guard for infinity loop;
        while remain_tiles > 10 {
            let current_max_width = rng.gen_range(4..(1 + max_width));
            let current_max_height = rng.gen_range(4..(1 + max_height));

            let mut current_max_width_for_min_width_range: u16 = (current_max_width / 4) as u16;
            if current_max_width_for_min_width_range < 2 {
                current_max_width_for_min_width_range = 2;
            };

            let mut current_max_height_for_min_height_range: u16 =
                (current_max_height / 4) as u16;
            if current_max_height_for_min_height_range < 2 {
                current_max_height_for_min_height_range = 2;
            };

            let current_min_width =
                rng.gen_range(1..(1 + current_max_width_for_min_width_range)); // 25% of maximum value
            let current_min_height =
                rng.gen_range(1..(1 + current_max_height_for_min_height_range)); // 25% of maximum value

            let spot_setting: SpotSetting = SpotSetting {
                amount: 1,
                emerging: 100, //100%
                ground_type: ground_type.clone(),
                cover_type: CoverType::None,
                max_width: current_max_width,
                max_height: current_max_height,
                min_width: current_min_width,
                min_height: current_min_height,
                x_offset: 1,
                y_offset: 1,
                height_offset: 1,
                width_offset: 1,
            };

            generate_spot(tilemap, deploy, &spot_setting);
            let tiles_used = ((current_max_width + current_min_width) / 2)
                * ((current_max_height + current_min_height) / 2);
            remain_tiles -= tiles_used as usize;
        }
    }
}

fn generate_cover(
    tilemap: &mut Tilemap, 
    cover_type: &CoverType, 
    percent: u8, 
    deploy: &Deploy
) {
    let mut rng = rand::thread_rng();
    let tile_setting = deploy.tile.get_cover_tile_deploy(&cover_type);
    for tile in tilemap.tilemap_tile_storage.iter_mut() {
        let random_num = rng.gen_range(0..100); // 100%
        if percent > random_num {
            let set_cover_to_tile: bool = match cover_type {
                CoverType::Grass | CoverType::Flowers => match tile.ground_type {
                    GroundType::Earth => true,
                    _ => false,
                },
                _ => true, // all other can generate wherever except rock ground type; but in this fucntion rock's r not generated yet;
            };
            if set_cover_to_tile {
                tile.cover_type = tile_setting.cover_type.clone();
            }
        }
    }
}

fn generate_additional_cover(
    tilemap: &mut Tilemap,
    additional_cover: &HashMap<CoverType, f32>,
    deploy: &Deploy,
) {
    let mut rng = rand::thread_rng();
    for (key, value) in additional_cover.iter() {
        let percent: f32 = *value;
        let cover_type: CoverType = key.clone();
        let mut remain_tiles: usize = (tilemap.total_tiles as f32 * percent / 100.0) as usize;

        let mut max_width = (tilemap.tilemap_width * 5 / 100) as u16; // 5% of tilemap width;
        if max_width < 5 {
            max_width = 5;
        };

        let mut max_height: u16 = (tilemap.tilemap_height * 5 / 100) as u16; // 5% of tilemap height;
        if max_height < 5 {
            max_height = 5;
        };

        while remain_tiles > 10 {
            let current_max_width = rng.gen_range(4..(max_width + 1));
            let current_max_height = rng.gen_range(4..(max_height + 1));

            let mut current_max_width_for_min_width_range: u16 = (current_max_width / 4) as u16;
            if current_max_width_for_min_width_range < 2 {
                current_max_width_for_min_width_range = 2;
            };

            let mut current_max_height_for_min_height_range: u16 =
                (current_max_height / 4) as u16;
            if current_max_height_for_min_height_range < 2 {
                current_max_height_for_min_height_range = 2;
            };

            let current_min_width =
                rng.gen_range(1..(current_max_width_for_min_width_range + 1)); // 25% of maximum value
            let current_min_height =
                rng.gen_range(1..(current_max_height_for_min_height_range + 1)); // 25% of maximum value

            let spot_setting: SpotSetting = SpotSetting {
                amount: 1,
                emerging: 100, //100%
                ground_type: GroundType::Earth,
                cover_type: cover_type.clone(),
                max_width: current_max_width,
                max_height: current_max_height,
                min_width: current_min_width,
                min_height: current_min_height,
                x_offset: 1,
                y_offset: 1,
                height_offset: 1,
                width_offset: 1,
            };

            generate_spot(tilemap, deploy, &spot_setting);
            let tiles_used = ((current_max_width + current_min_width) / 2)
                * ((current_max_height + current_min_height) / 2);
            remain_tiles -= tiles_used as usize;
        }
    }
}

fn generate_spots(
    tilemap: &mut Tilemap, 
    spot_vector: &Vec<SpotSetting>, 
    deploy: &Deploy
) {
    let vec_len = spot_vector.len();
    if vec_len == 0 {
        return;
    };

    for i in 0..vec_len {
        let spot_setting = &spot_vector[i];
        generate_spot(tilemap, deploy, spot_setting);
    }
}

fn generate_spot(
    tilemap: &mut Tilemap, 
    deploy: &Deploy, 
    spot_setting: &SpotSetting
) {
    let mut rng = rand::thread_rng();
    let tilemap_width = tilemap.tilemap_width;
    let tilemap_height = tilemap.tilemap_height;
    let half_tilemap_width = (tilemap_width / 2) as u16;
    let half_tilemap_height = (tilemap_height / 2) as u16;
    let total_tiles = tilemap.total_tiles;

    for _ in 0..spot_setting.amount {
        let random_num = rng.gen_range(0..100); //100%
        if random_num >= spot_setting.emerging {
            continue;
        };

        let ground_type = spot_setting.ground_type.clone();
        let cover_type: CoverType = spot_setting.cover_type.clone();
        // if cover_type == CoverType::None - function generation ground tiles; else generating only cover;

        let ground_data = deploy.tile.get_ground_tile_deploy(&ground_type);
        let cover_data = deploy.tile.get_cover_tile_deploy(&cover_type);
        let min_width: u16 = spot_setting.min_width;
        let min_height: u16 = spot_setting.min_height;
        let x_offset: i8 = spot_setting.x_offset;
        let y_offset: i8 = spot_setting.y_offset;
        let height_offset: i8 = spot_setting.height_offset;
        let width_offset: i8 = spot_setting.width_offset;

        let max_height = if tilemap_height <= spot_setting.max_height {
            println!(
                "ground_tilemap.generate_spot; Map Height: {}, Max Height: {} ",
                tilemap_height, spot_setting.max_height
            );
            (tilemap_height / 20) as u16 // by default do 20% of max height;
        } else {
            spot_setting.max_height
        };

        let max_width = if tilemap_width <= spot_setting.max_width {
            println!(
                "ground_tilemap.generate_spot; Map Width: {}, Max Width: {} ",
                tilemap_width, spot_setting.max_width
            );
            (tilemap_width / 20) as u16 // by default do 20% of max height;
        } else {
            spot_setting.max_width
        };

        let start_range_x: i32 = -(half_tilemap_width as i32);
        let end_range_x: i32 = half_tilemap_width as i32 - max_width as i32;
        let starting_point_x = rng.gen_range(start_range_x..end_range_x);

        let start_range_y: i32 = -(half_tilemap_height as i32);
        let end_range_y: i32 = half_tilemap_height as i32 - max_height as i32;
        let starting_point_y = rng.gen_range(start_range_y..end_range_y);

        let mut current_width = rng.gen_range(min_width..(max_width + 1));
        let mut current_height = rng.gen_range(min_height..(max_height + 1));

        let average_width = ((min_width + max_width) / 2) as u16;
        let average_height: u16 = ((min_height + max_height) / 2) as u16;

        let mut left_top_point_x = starting_point_x;
        let mut left_top_point_y: i32 = starting_point_y;

        // do horizontal lines
        for i in 0..average_height {
            left_top_point_x += rng.gen_range(-x_offset..(x_offset + 1)) as i32;
            if left_top_point_x < -(half_tilemap_width as i32) {
                left_top_point_x = -(half_tilemap_width as i32);
            }

            current_width = (current_width as i32
                + rng.gen_range(-width_offset..(width_offset + 1)) as i32)
                as u16;
            if current_width > max_width {
                current_width = max_width
            };
            if current_width < min_width {
                current_width = min_width
            };

            let y = starting_point_y + i as i32 + half_tilemap_height as i32;
            for j in 0..current_width {
                let x = left_top_point_x + j as i32 + half_tilemap_width as i32;
                let index: usize = y as usize * tilemap_height as usize + x as usize;
                if index > total_tiles {
                    continue;
                };

                let tile = tilemap.get_tile_by_index_mut(index);
                match cover_type {
                    CoverType::None => {
                        tile.ground_type = ground_type.clone();
                        tile.cover_type = cover_type.clone();
                        tile.movement_ratio = ground_data.movement_ratio;
                        tile.permissions = ground_data.permissions.to_vec();

                        match ground_type {
                            GroundType::Rock => {
                                tile.permissions.clear();
                                tile.permissions.push(TilePermissions::Fog);
                            },
                            _ => {},
                        }
                        
                    }
                    CoverType::Flowers | CoverType::Grass => match tile.ground_type {
                        GroundType::Earth => {
                            tile.cover_type = cover_type.clone();
                            tile.movement_ratio = cover_data.movement_ratio;
                            tile.permissions = cover_data.permissions.to_vec();
                        }
                        _ => {
                            continue;
                        }
                    },
                    _ => match tile.ground_type {
                        GroundType::Rock => {
                            continue;
                        }
                        _ => {
                            tile.cover_type = cover_type.clone();
                            tile.movement_ratio = cover_data.movement_ratio;
                            tile.permissions = cover_data.permissions.to_vec();
                        }
                    },
                };
            }
        }

        for k in 0..average_width {
            left_top_point_y += (rng.gen_range(-y_offset..(y_offset + 1))) as i32;
            if left_top_point_y < -(half_tilemap_height as i32) {
                left_top_point_y = -(half_tilemap_height as i32);
            }

            let current_height_i32 = current_height as i32
                + (rng.gen_range(-height_offset..(height_offset + 1))) as i32;
            if current_height_i32 < 1 {
                current_height = 1;
            } else {
                current_height = current_height_i32 as u16;
            }

            if current_height > max_height {
                current_height = max_height
            };
            if current_height < min_height {
                current_height = min_height
            };

            let x = starting_point_x + k as i32 + half_tilemap_width as i32;
            for l in 0..current_height {
                let y = left_top_point_y + l as i32 + half_tilemap_height as i32;
                let index = y as usize * tilemap.tilemap_height as usize + x as usize;
                if index > tilemap.total_tiles {
                    continue;
                };

                let tile = tilemap.get_tile_by_index_mut(index);
                match cover_type {
                    CoverType::None => {
                        tile.ground_type = ground_type.clone();
                        tile.cover_type = cover_type.clone();
                        tile.movement_ratio = ground_data.movement_ratio;
                        tile.permissions = ground_data.permissions.to_vec();

                        match ground_type {
                            GroundType::Rock => {
                                tile.permissions.clear();
                                tile.permissions.push(TilePermissions::Fog);
                            },
                            _ => {},
                        }
                    }
                    CoverType::Flowers | CoverType::Grass => match tile.ground_type {
                        GroundType::Earth => {
                            tile.cover_type = cover_type.clone();
                            tile.movement_ratio = cover_data.movement_ratio;
                            tile.permissions = cover_data.permissions.to_vec();
                        }
                        _ => {
                            continue;
                        }
                    },
                    _ => match tile.ground_type {
                        GroundType::Rock => {
                            continue;
                        }
                        _ => {
                            tile.cover_type = cover_type.clone();
                            tile.movement_ratio = cover_data.movement_ratio;
                            tile.permissions = cover_data.permissions.to_vec();
                        }
                    },
                };
            }
        }
    }
}

fn generate_rivers(
    tilemap: &mut Tilemap, 
    river_vector: &Vec<RiverSetting>, 
    deploy: &Deploy
) {
    let vec_len = river_vector.len();
    if vec_len == 0 {
        return;
    };

    for i in 0..vec_len {
        let river_setting = &river_vector[i];
        generate_river(tilemap, &river_setting, &deploy);
    }
}

fn generate_river(
    tilemap: &mut Tilemap, 
    river_setting: &RiverSetting, 
    deploy: &Deploy
) {
    let mut rng = rand::thread_rng();
    let tilemap_width = tilemap.tilemap_width;
    let tilemap_height = tilemap.tilemap_height;
    let half_tilemap_width = (tilemap_width / 2) as u16;
    let half_tilemap_height = (tilemap_height / 2) as u16;

    let mut random_num: u8 = rng.gen_range(0..100); // 100%
    if random_num >= river_setting.emerging {
        return;
    };

    let cover_type = river_setting.cover_type.clone();
    let ground_type = river_setting.ground_type.clone();
    let max_width = river_setting.max_width;
    let min_width = river_setting.min_width;
    let offset = river_setting.offset;
    let offset_width = river_setting.offset_width;

    let mut current_width = rng.gen_range(min_width..max_width);
    let mut river_type = river_setting.river_type.clone();

    if river_type == RiverType::Random {
        random_num = rng.gen_range(0..2);
        if random_num == 0 {
            river_type = RiverType::Vertical;
        } else {
            river_type = RiverType::Horizontal;
        }
    }

    match river_type {
        RiverType::Horizontal => {
            let start_range_y: i32 =
                -(half_tilemap_height as i32) + current_width as i32 + offset as i32;
            let end_range_y: i32 =
                half_tilemap_height as i32 - current_width as i32 - offset as i32;
            let mut river_point_y: i32 = rng.gen_range(start_range_y..end_range_y);

            for i in 0..tilemap_width {
                river_point_y += rng.gen_range(-offset as i32..(offset + 1) as i32);
                if river_point_y < -(half_tilemap_height as i32)
                    || river_point_y > half_tilemap_height as i32
                {
                    continue; // do not generate out of tilemap points;
                };

                let current_width_i32: i32 = current_width as i32
                    + rng.gen_range(-offset_width as i32..(offset_width + 1) as i32);
                current_width = if current_width_i32 < min_width as i32 {
                    min_width
                } else if current_width_i32 > max_width as i32 {
                    max_width
                } else {
                    current_width_i32 as u16
                };

                for j in 0..current_width {
                    let y = river_point_y + j as i32 + half_tilemap_height as i32;
                    let index = y as usize * tilemap_height as usize + i as usize;
                    if index >= tilemap.total_tiles {
                        continue;
                    };

                    let tile = tilemap.get_tile_by_index_mut(index as usize);
                    let mut tile_data: &TileDeploy = deploy
                        .tile
                        .get_ground_tile_deploy(&river_setting.ground_type);

                    if cover_type == CoverType::None {
                        tile.ground_type = ground_type.clone();
                        if ground_type == GroundType::Rock {
                            tile.cover_type = CoverType::None;
                        };
                    } else {
                        tile_data =
                            deploy.tile.get_cover_tile_deploy(&river_setting.cover_type);
                        tile.cover_type = cover_type.clone();
                    };

                    tile.permissions = tile_data.permissions.to_vec();
                    tile.movement_ratio = tile_data.movement_ratio;
                }
            }
        }
        RiverType::Vertical => {
            let start_range_x = -(half_tilemap_width as i32) + offset as i32;
            let end_range_x = half_tilemap_width as i32 - current_width as i32 - offset as i32;
            let mut river_point_x = rng.gen_range(start_range_x..end_range_x);

            for i in 0..tilemap_height {
                river_point_x += rng.gen_range(-offset as i32..(offset + 1) as i32);
                if river_point_x < -(half_tilemap_width as i32)
                    || river_point_x > half_tilemap_width as i32
                {
                    continue;
                };

                let current_width_i32: i32 = current_width as i32
                    + rng.gen_range(-offset_width as i32..(offset_width + 1) as i32);
                current_width = if current_width_i32 < min_width as i32 {
                    min_width
                } else if current_width_i32 > max_width as i32 {
                    max_width
                } else {
                    current_width_i32 as u16
                };
                for j in 0..current_width {
                    let x = river_point_x + j as i32 + half_tilemap_width as i32;
                    let index = x as usize + tilemap_height as usize * i as usize;
                    if index >= tilemap.total_tiles {
                        continue;
                    };

                    let tile = tilemap.get_tile_by_index_mut(index);
                    let mut tile_data: &TileDeploy = deploy
                        .tile
                        .get_ground_tile_deploy(&river_setting.ground_type);

                    if cover_type == CoverType::None {
                        tile.ground_type = ground_type.clone();
                        if ground_type == GroundType::Rock {
                            tile.cover_type = CoverType::None;
                        };
                    } else {
                        tile_data =
                            deploy.tile.get_cover_tile_deploy(&river_setting.cover_type);
                        tile.cover_type = cover_type.clone();
                    };

                    tile.permissions = tile_data.permissions.to_vec();
                    tile.movement_ratio = tile_data.movement_ratio;
                }
            }
        }
        _ => panic!(" Unknown river type: {:?}", river_type),
    }
}

pub fn generate_environment_for_rock_ground(tilemap: &mut Tilemap, deploy: &Deploy, environment_value: u8) {
    let tilemap_height: u16 = tilemap.tilemap_height;
    let tilemap_width: u16 = tilemap.tilemap_width;
    let half_tilemap_height = (tilemap_height / 2) as u16;
    let half_tilemap_width = (tilemap_width / 2) as u16;
    let mut rng = rand::thread_rng();

    let max_envirounment: u8 = environment_value;

    let data_tile = deploy.tile.get_ground_tile_deploy(&GroundType::Rock);

    for i in 0..tilemap.tilemap_tile_storage.len() {
        let x = tilemap.tilemap_tile_storage[i].position.x;
        let y = tilemap.tilemap_tile_storage[i].position.y;

        if tilemap.tilemap_tile_storage[i].ground_type != GroundType::Rock {
            continue;                                                           //skip non rock ground tile;
        }

        match tilemap.tilemap_tile_storage[i].permissions.iter().find(|&x| x == &TilePermissions::Walk) {
            Some(_) => {},
            None => continue,                                                   //skip non rock things;
        }

        let current_envirounment = rng.gen_range(0..=max_envirounment);          //рандомно выбираем "подложку"
        if current_envirounment == 0 {
            continue;
        };

        let grid_multiplier = current_envirounment * 2 + 1;                         // окружность вокруг тайла ( CurEnv = 1; x = 3, y = 3 ( 3 x 3 ) );

        for i in 0..grid_multiplier {
            for j in 0..grid_multiplier {
                let new_x = x - current_envirounment as i32 + j as i32 + half_tilemap_width as i32;
                let new_y = y - current_envirounment as i32 + i as i32 + half_tilemap_height as i32;
                if new_x >= tilemap_width as i32 || new_x < 0 || new_y < 0 || new_y >= tilemap_height as i32 {
                    continue;                                                           //skip values not in range of tilemap;
                };

                let index: usize = new_y as usize * tilemap_height as usize + new_x as usize;
                let environment_tile: &mut Tile = tilemap.get_tile_by_index_mut(index);

                if environment_tile.ground_type == GroundType::Rock {
                    continue;                                                           //skip already rock environment or rock thing;
                }

                environment_tile.ground_type = data_tile.ground_type.clone();
                environment_tile.cover_type = data_tile.cover_type.clone();
                environment_tile.permissions = data_tile.permissions.to_vec();
                environment_tile.movement_ratio = data_tile.movement_ratio;
            }
        }
    }
}

fn generate_environment_for_water_cover(
    tilemap: &mut Tilemap, 
    deploy: &Deploy, 
    environment_value: u8
) {
    let tilemap_height: u16 = tilemap.tilemap_height;
    let tilemap_width: u16 = tilemap.tilemap_width;
    let half_tilemap_height = (tilemap_height / 2) as u16;
    let half_tilemap_width = (tilemap_width / 2) as u16;
    let mut rng = rand::thread_rng();

    let max_envirounment: u8 = environment_value;
    let data_tile = deploy.tile.get_cover_tile_deploy(&CoverType::Shallow);

    for i in 0..tilemap.tilemap_tile_storage.len() {
        let x = tilemap.tilemap_tile_storage[i].position.x;
        let y = tilemap.tilemap_tile_storage[i].position.y;
        if tilemap.tilemap_tile_storage[i].cover_type != CoverType::Water {
            continue;                                                       //skip non water cover tiles;
        }
        
        let current_envirounment = rng.gen_range(0..=max_envirounment);      //рандомно выбираем "подложку"
        if current_envirounment == 0 {
            continue;
        };

        let grid_multiplier = current_envirounment * 2 + 1; // окружность вокруг тайла ( CurEnv = 1; x = 3, y = 3 ( 3 x 3 ) );

        for i in 0..grid_multiplier {
            for j in 0..grid_multiplier {
                let new_x = x - current_envirounment as i32 + j as i32 + half_tilemap_width as i32;
                let new_y = y - current_envirounment as i32 + i as i32 + half_tilemap_height as i32;
                if new_x >= tilemap_width as i32 || new_x < 0 || new_y < 0 || new_y >= tilemap_height as i32 {
                    continue;
                };

                let index: usize = new_y as usize * tilemap_height as usize + new_x as usize;
                let environment_tile: &mut Tile = tilemap.get_tile_by_index_mut(index);

                match environment_tile.ground_type {
                    GroundType::Rock => {
                        match environment_tile.permissions.iter().find(|&&x| x == TilePermissions::Walk) {
                            Some(_) => {},
                            None => continue,                                               //skip rock thing;
                        }
                    },
                    _ => {},
                }

                if environment_tile.cover_type == CoverType::Water || environment_tile.cover_type == CoverType::Shallow {
                    continue;                                                               //skip water or shallow cover type tile;
                }

                environment_tile.cover_type = data_tile.cover_type.clone();
                environment_tile.permissions = data_tile.permissions.to_vec();
                environment_tile.movement_ratio = data_tile.movement_ratio;
            }
        }
    }
}

fn spread_indexes_for_cover_tiles(tilemap: &mut Tilemap) {
    for i in 0..tilemap.tilemap_tile_storage.len() {
        let x = tilemap.tilemap_tile_storage[i].position.x;
        let y: i32 = tilemap.tilemap_tile_storage[i].position.y;
        let tile_cover: &CoverType = &tilemap.tilemap_tile_storage[i].cover_type;

        let cover_graphic_index: u8 = match tile_cover {
            CoverType::Water | CoverType::Shallow | CoverType::Ice => {
                find_cover_graphic_index_for_cover_tiles(tilemap, x, y)
            }
            _ => continue,
        };

        tilemap.tilemap_tile_storage[i].cover_graphic_index = cover_graphic_index;
    }
}

fn generate_solids_liquids(
    tilemap: &mut Tilemap, 
    spots: &Spots, 
    rivers: &Rivers, 
    deploy: &Deploy
) {
    let solid_rivers = &rivers.solid_river;
    let solid_spots = &spots.solid_spot;
    let liquid_rivers = &rivers.liquid_river;
    let liquid_spots = &spots.liquid_spot;

    //first generate solids, then liquids;
    generate_spots(tilemap, &solid_spots, &deploy);
    generate_rivers(tilemap, &solid_rivers, &deploy);
    generate_spots(tilemap, &liquid_spots, &deploy);
    generate_rivers(tilemap, &liquid_rivers, &deploy);
}

fn find_cover_graphic_index_for_cover_tiles(
    tilemap: &mut Tilemap, 
    x: i32,
     y: i32
    ) -> u8 {
    // actually do for water, ice and shallow;
    let tile_storage: &Vec<Tile> = &tilemap.tilemap_tile_storage;
    let tilemap_height = tilemap.tilemap_height;
    let tilemap_width = tilemap.tilemap_width;
    let half_tilemap_height = tilemap_height / 2;
    let half_tilemap_width = tilemap_width / 2;
    let tilemap_total_tiles = tilemap.total_tiles;

    let top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32
        + x
        + half_tilemap_width as i32;
    let left_top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32 + x
        - 1
        + half_tilemap_width as i32;
    let left_index: i32 = (y + half_tilemap_height as i32) * tilemap_height as i32 + x - 1
        + half_tilemap_width as i32;
    let right_top_index: i32 = (y + 1 + half_tilemap_height as i32) * tilemap_height as i32
        + 1
        + x
        + half_tilemap_width as i32;
    let right_index: i32 = (y + half_tilemap_height as i32) * tilemap_height as i32
        + x
        + 1
        + half_tilemap_width as i32;
    let bottom_index: i32 = (y - 1 + half_tilemap_height as i32) * tilemap_height as i32
        + x
        + half_tilemap_width as i32;
    let left_bottom_index: i32 =
        (y - 1 + half_tilemap_height as i32) * tilemap_height as i32 + x - 1
            + half_tilemap_width as i32;
    let right_bottom_index: i32 = (y - 1 + half_tilemap_height as i32) * tilemap_height as i32
        + x
        + 1
        + half_tilemap_width as i32;

    let vec_of_indexes = vec![
        top_index,
        left_top_index,
        left_index,
        right_top_index,
        right_index,
        bottom_index,
        left_bottom_index,
        right_bottom_index,
    ];

    let mut vec_of_bool: Vec<bool> = vec![];

    for index in vec_of_indexes.iter() {
        let new_bool = if *index < 0 || *index as usize >= tilemap_total_tiles {
            false
        } else {
            match tile_storage[*index as usize].cover_type {
                CoverType::Shallow | CoverType::Water | CoverType::Ice => true,
                _ => false,
            }
        };

        vec_of_bool.push(new_bool);
    }

    return get_index_for_graphic_placement(vec_of_bool);
}

pub fn get_index_for_graphic_placement(vec_of_bool: Vec<bool>) -> u8 {
    let top: bool = vec_of_bool[0];
    let left: bool = vec_of_bool[1];
    let right: bool = vec_of_bool[2];
    let bottom: bool = vec_of_bool[3];
    let left_top: bool = vec_of_bool[4];
    let right_top: bool = vec_of_bool[5];
    let left_bottom: bool = vec_of_bool[6];
    let right_bottom: bool = vec_of_bool[7];

    if top && left && right && bottom && left_top && right_top && left_bottom && right_bottom {
        // all
        return 0;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && right_top
        && left_bottom
        && !right_bottom
    {
        // excluded right_bottom
        return 1;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && right_top
        && !left_bottom
        && right_bottom
    {
        // excluded  left_bottom
        return 2;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && !right_top
        && left_bottom
        && right_bottom
    {
        // excluded  right_top
        return 3;
    } else if top
        && left
        && right
        && bottom
        && !left_top
        && right_top
        && left_bottom
        && right_bottom
    {
        // excluded  left_top
        return 4;
    } else if top && left && right && !bottom && left_top && right_top && left_bottom && right_bottom //excluded  bottom
        || top && left && right && !bottom && left_top && right_top && left_bottom && !right_bottom //excluded right bottom and bottom
        || top && left && right && !bottom && left_top && right_top && !left_bottom && right_bottom //excluded left bottom and bottom 
        || top && left && right && !bottom && left_top && right_top && !left_bottom && !right_bottom //excluded left bottom and left and right bottom
    {
        return 5;
    } else if top && left && !right && bottom && left_top && right_top && left_bottom && right_bottom //excluded  right 
        || top && left && !right && bottom && left_top && right_top && left_bottom && !right_bottom  //excluded right bottom and right
        || top && left && !right && bottom && left_top && !right_top && left_bottom && right_bottom  //excluded right top and right
        || top && left && !right && bottom && left_top && !right_top && left_bottom && !right_bottom //excluded right top and right and right bottom
    {
        return 6;
    } else if top && !left && right && bottom && left_top && right_top && left_bottom && right_bottom //excluded  left
        || top && !left && right && bottom && !left_top && right_top && left_bottom && right_bottom //excluded left top and left
        || top && !left && right && bottom && left_top && right_top && !left_bottom && right_bottom //excluded left bottom and left
        || top && !left && right && bottom && !left_top && right_top && !left_bottom && right_bottom //left top and left and left bottom
    {
        return 7;
    } else if !top && left && right && bottom && left_top && right_top && left_bottom && right_bottom //excluded  top
        || !top && left && right && bottom && left_top && !right_top && left_bottom && right_bottom //excluded right top and top
        || !top && left && right && bottom && !left_top && right_top && left_bottom && right_bottom //excluded left top and top
        || !top && left && right && bottom && !left_top && !right_top && left_bottom && right_bottom //left top and top and right top
    {
        return 8;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && right_top
        && !left_bottom
        && !right_bottom
    {
        // excluded left bottom and right bottom
        return 9;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && !right_top
        && left_bottom
        && !right_bottom
    {
        // excluded right bottom and right top
        return 10;
    } else if top
        && left
        && right
        && bottom
        && !left_top
        && right_top
        && left_bottom
        && !right_bottom
    {
        // exluded right bottom and left top
        return 11;
    } else if top
        && left
        && right
        && bottom
        && !left_top
        && right_top
        && !left_bottom
        && right_bottom
    {
        // excluded left bottom and left top
        return 12;
    } else if top
        && left
        && right
        && bottom
        && !left_top
        && !right_top
        && left_bottom
        && right_bottom
    {
        // excluded right top and left top
        return 13;
    } else if top
        && left
        && right
        && bottom
        && left_top
        && !right_top
        && !left_bottom
        && right_bottom
    {
        // excluded left bottom and right top
        return 14;
    } else if top && !left && right && bottom && left_top && right_top && left_bottom && !right_bottom  // excluded right bottom and left
        || top && !left && right && bottom && left_top && right_top && !left_bottom && !right_bottom //excluded right bottom and left and left bottom
        || top && !left && right && bottom && !left_top && right_top && left_bottom && !right_bottom // excluded right bottom and left and left top
        || top && !left && right && bottom && !left_top && right_top && !left_bottom && !right_bottom // exluded right bottom and left and left top and left bottom
    {
        return 15;
    } else if !top && left && right && bottom && left_top && right_top && left_bottom && !right_bottom // excluded right bottom and top
        || !top && left && right && bottom && left_top && !right_top && left_bottom && !right_bottom // excluded right bottom and top and right top
        || !top && left && right && bottom && !left_top && right_top && left_bottom && !right_bottom // exluded right bottom and top and left top
        || !top && left && right && bottom && !left_top && !right_top && left_bottom && !right_bottom // exluded right bottom and top and left top and right top
    {     
        return 16;
    } else if top && left && !right && bottom && left_top && right_top && !left_bottom && right_bottom // excluded left bottom and right
        || top && left && !right && bottom && left_top && right_top && !left_bottom && !right_bottom // excluded left bottom and right and right bottom
        || top && left && !right && bottom && left_top && !right_top && !left_bottom && right_bottom // exluded left bottom and right and right top
        || top && left && !right && bottom && left_top && !right_top && !left_bottom && !right_bottom // exluded left bottom and right and right bottom and right top
    {    
        return 17;
    } else if !top && left && right && bottom && left_top && right_top && !left_bottom && right_bottom // excluded left bottom and top
        || !top && left && right && bottom && !left_top && right_top && !left_bottom && right_bottom // excluded left bottom and top and left top
        || !top && left && right && bottom && left_top && !right_top && !left_bottom && right_bottom // excluded left bottom and top and right top
        || !top && left && right && bottom && !left_top && !right_top && !left_bottom && right_bottom // ecluded left bottom and top and left top and right top
    {
        return 18;
    } else if top && left && right && !bottom && left_top && !right_top && left_bottom && right_bottom // excluded right top and bottom
        || top && left && right && !bottom && left_top && !right_top && !left_bottom && right_bottom // exlcluded right top and bottom and left bottom
        || top && left && right && !bottom && left_top && !right_top && left_bottom && !right_bottom // excluded right top and bottom and right bottom
        || top && left && right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // exluded right top and bottom and left bottom and right bottom
    {
        return 19;
    } else if top && !left && right && bottom && left_top && !right_top && left_bottom && right_bottom // excluded right top and left
        || top && !left && right && bottom && !left_top && !right_top && left_bottom && right_bottom // excluded right top and left and left top
        || top && !left && right && bottom && left_top && !right_top && !left_bottom && right_bottom // exccluded right top and left and left bottom
        || top && !left && right && bottom && !left_top && !right_top && !left_bottom && right_bottom // excluded right top and left and left bottom and left top
    {
        return 20;
    } else if top && left && right && !bottom && !left_top && right_top && left_bottom && right_bottom // excluded left top and bottom
        || top && left && right && !bottom && !left_top && right_top && !left_bottom && right_bottom // excluded left top and bottom and left bottom
        || top && left && right && !bottom && !left_top && right_top && left_bottom && !right_bottom // excluded left top and bottom and right bottom
        || top && left && right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // excluded left top and bottom and right bottom and left bottom
    {
        return 21;
    } else if top && left && !right && bottom && !left_top && right_top && left_bottom && right_bottom  // excluded left top and right
        || top && left && !right && bottom && !left_top && !right_top && left_bottom && right_bottom // excluded left top and right and right top
        || top && left && !right && bottom && !left_top && right_top && left_bottom && !right_bottom // excluded left top and right and right bottom
        || top && left && !right && bottom && !left_top && !right_top && left_bottom && !right_bottom // excluded left top and right and right top and right bottom
    {
        return 22;
    } else if top && left && !right && !bottom && left_top && right_top && left_bottom && right_bottom // excluded bottom and right
        || top && left && !right && !bottom && left_top && right_top && left_bottom && !right_bottom // excluded bottom and right and right bottom
        || top && left && !right && !bottom && left_top && right_top && left_bottom && right_bottom // excluded bottom and right and right top
        || top && left && !right && !bottom && left_top && right_top && !left_bottom && right_bottom // excluded bottom and right and left bottom
        || top && left && !right && !bottom && left_top && !right_top && left_bottom && !right_bottom // excluded bottom and right and right bottom and right top
        || top && left && !right && !bottom && left_top && right_top && !left_bottom && !right_bottom // excluded bottom and right and right bottom and left bottom
        || top && left && !right && !bottom && left_top && !right_top && !left_bottom && right_bottom // excluded bottom and right and left bottom and right top
        || top && left && !right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // excluded bottom and right and right top and right bottom and left bottom
    {
        return 23;
    } else if top && !left && right && !bottom && left_top && right_top && left_bottom && right_bottom // excluded bottom and left
        || top && !left && right && !bottom && left_top && right_top && !left_bottom && right_bottom // excluded bottom and left and left bottom
        || top && !left && right && !bottom && !left_top && right_top && left_bottom && right_bottom // excluded bottom and left and left top
        || top && !left && right && !bottom && left_top && right_top && left_bottom && !right_bottom // excluded bottom and left and right bottom
        || top && !left && right && !bottom && !left_top && right_top && !left_bottom && right_bottom // excluded bottom and left and left bottom and left top
        || top && !left && right && !bottom && left_top && right_top && !left_bottom && !right_bottom // excluded bottom and left and left bottom and right bottom
        || top && !left && right && !bottom && !left_top && right_top && left_bottom && !right_bottom // excluded bottom and left and left top and right bottom
        || top && !left && right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // excluded bottom and left and left top and right bottom and left bottom
    {
        return 24;
    } else if !top && left && right && !bottom && left_top && right_top && left_bottom && right_bottom // excluded bottom and top
        || !top && left && right && !bottom && left_top && right_top && left_bottom && !right_bottom // excluded bottom and top and right bottom
        || !top && left && right && !bottom && left_top && right_top && !left_bottom && right_bottom // excluded bottom and top and left bottom
        || !top && left && right && !bottom && left_top && !right_top && left_bottom && right_bottom // excluded bottom and top and right top
        || !top && left && right && !bottom && !left_top && right_top && left_bottom && right_bottom // excluded bottom and top amd left top
        || !top && left && right && !bottom && left_top && right_top && !left_bottom && !right_bottom // excluded bottom and top and right bottom and left bottom
        || !top && left && right && !bottom && left_top && !right_top && left_bottom && !right_bottom // excluded bottom and top and right bottom and right top
        || !top && left && right && !bottom && !left_top && right_top && left_bottom && !right_bottom // excluded bottom and top and right bottom and left top
        || !top && left && right && !bottom && !left_top && right_top && !left_bottom && right_bottom // excluded bottom and top and left bottom and left top
        || !top && left && right && !bottom && left_top && !right_top && !left_bottom && right_bottom // excluded bottom and top and left bottom and right top
        || !top && left && right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // included left top and right and left
        || !top && left && right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // included left bottom and right and left
        || !top && left && right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // included right top and right and left
        || !top && left && right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // included rigth bottom and right and left
        || !top && left && right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // included right and left
    {
        return 25;
    } else if top && !left && !right && bottom && left_top && right_top && left_bottom && right_bottom // excluded left and right
        || top && !left && !right && bottom && left_top && right_top && left_bottom && !right_bottom // excluded left and right and right bottom
        || top && !left && !right && bottom && left_top && right_top && !left_bottom && right_bottom // excluded left and right and left bottom
        || top && !left && !right && bottom && left_top && !right_top && left_bottom && right_bottom // excluded left and right and right top
        || top && !left && !right && bottom && !left_top && right_top && left_bottom && right_bottom // excluded left and right and left top
        || top && !left && !right && bottom && left_top && right_top && !left_bottom && !right_bottom // excluded left and right and right bottom and left bottom
        || top && !left && !right && bottom && left_top && !right_top && left_bottom && !right_bottom // excluded left and right and right bottom and right top
        || top && !left && !right && bottom && !left_top && right_top && left_bottom && !right_bottom // excluded left and right and right bottom and left top
        || top && !left && !right && bottom && !left_top && right_top && !left_bottom && right_bottom // excluded left and right and left bottom and left top
        || top && !left && !right && bottom && left_top && !right_top && !left_bottom && right_bottom // excluded left and right and left bottom and right top
        || top && !left && !right && bottom && left_top && !right_top && !left_bottom && !right_bottom // include left top and top and bottom
        || top && !left && !right && bottom && !left_top && !right_top && left_bottom && !right_bottom // include left bottom and top and bottom
        || top && !left && !right && bottom && !left_top && right_top && !left_bottom && !right_bottom // include right top and top and bottom
        || top && !left && !right && bottom && !left_top && !right_top && !left_bottom && right_bottom // include right bottom and top and bottom
        || top && !left && !right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // include top and bottom
    {
        return 26;
    } else if !top && left && !right && bottom && left_top && right_top && left_bottom && right_bottom // excluded top and right
        || !top && left && !right && bottom && left_top && !right_top && left_bottom && right_bottom // excluded top and right and right top
        || !top && left && !right && bottom && !left_top && right_top && left_bottom && right_bottom // excluded top and right and left top
        || !top && left && !right && bottom && left_top && right_top && left_bottom && !right_bottom // excluded top and right and right bottom
        || !top && left && !right && bottom && !left_top && !right_top && left_bottom && right_bottom // exluded top and right and right top and left top
        || !top && left && !right && bottom && left_top &&! right_top && left_bottom && !right_bottom // excluded top and right and right top and right bottom
        || !top && left && !right && bottom && !left_top && right_top && left_bottom && !right_bottom // excluded top and right and left top and right bottom
        || !top && left && !right && bottom && !left_top && !right_top && left_bottom && !right_bottom // included left and bottom and left bottom
        || !top && left && !right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // included left and bottom
    { 
        return 27;
    } else if !top && !left && right && bottom && left_top && right_top && left_bottom && right_bottom // excluded top and left
        || !top && !left && right && bottom && !left_top && right_top && left_bottom && right_bottom // excluded top and left and left top
        || !top && !left && right && bottom && left_top && !right_top && left_bottom && right_bottom // excluded top and left and right top
        || !top && !left && right && bottom && left_top && right_top && !left_bottom && right_bottom // excluded top and left and left bottom
        || !top && !left && right && bottom && !left_top && right_top && !left_bottom && right_bottom // excluded top and left and left top and left bottom
        || !top && !left && right && bottom && !left_top && !right_top && left_bottom && right_bottom // excluded top and left and left top and right top
        || !top && !left && right && bottom && left_top && !right_top && !left_bottom && right_bottom // excluded top and left and left bottom and right top
        || !top && !left && right && bottom && !left_top && !right_top && !left_bottom && right_bottom // include right and bottom and right bottom
        || !top && !left && right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // include right and bottom
    { 
        return 28;
    } else if top && !left && !right && !bottom && left_top && right_top && left_bottom && right_bottom // exclude left and right and bottom
        || top && !left && !right && !bottom && left_top && right_top && left_bottom && !right_bottom // exclude left and right and bottom and right bottom
        || top && !left && !right && !bottom && left_top && right_top && !left_bottom && right_bottom // exclude left and right and bottom and left bottom
        || top && !left && !right && !bottom && left_top && !right_top && left_bottom && right_bottom // exclude left and right and bottom and right top
        || top && !left && !right && !bottom && !left_top && right_top && left_bottom && right_bottom // exclude left and right and bottom and left top
        || top && !left && !right && !bottom && left_top && right_top && !left_bottom && !right_bottom // include top and left top and right top
        || top && !left && !right && !bottom && !left_top && !right_top && left_bottom && right_bottom // include top and left bottom and right bottom
        || top && !left && !right && !bottom && left_top && !right_top && left_bottom && !right_bottom // include top and left top and left bottom
        || top && !left && !right && !bottom && !left_top && right_top && !left_bottom && right_bottom // include top and right top and right bottom
        || top && !left && !right && !bottom && !left_top && right_top && left_bottom && !right_bottom // include top and right top and left bottom
        || top && !left && !right && !bottom && left_top && !right_top && !left_bottom && right_bottom // include top and left top and right bottom
        || top && !left && !right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // include top and left top
        || top && !left && !right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // include top and right top
        || top && !left && !right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // include top and left bottom
        || top && !left && !right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // include top and right bottom
        || top && !left && !right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // include top
    {
        return 29;
    } else if !top && !left && !right && bottom && left_top && right_top && left_bottom && right_bottom // exclude left and right and top
        || !top && !left && !right && bottom && left_top && right_top && left_bottom && !right_bottom // exclude left and right and top and right bottom
        || !top && !left && !right && bottom && left_top && right_top && !left_bottom && right_bottom // exclude left and right and top and left bottom
        || !top && !left && !right && bottom && left_top && !right_top && left_bottom && right_bottom // exclude left and right and top and right top
        || !top && !left && !right && bottom && !left_top && right_top && left_bottom && right_bottom // exclude left and right and top and left top
        || !top && !left && !right && bottom && left_top && right_top && !left_bottom && !right_bottom // include bottom and left top and right top
        || !top && !left && !right && bottom && !left_top && !right_top && left_bottom && right_bottom // include bottom and left bottom and right bottom
        || !top && !left && !right && bottom && left_top && !right_top && left_bottom && !right_bottom // include bottom and left top and left bottom
        || !top && !left && !right && bottom && !left_top && right_top && !left_bottom && right_bottom // include bottom and right top and right bottom
        || !top && !left && !right && bottom && !left_top && right_top && left_bottom && !right_bottom // include bottom and right top and left bottom
        || !top && !left && !right && bottom && left_top && !right_top && !left_bottom && right_bottom // include bottom and left top and right bottom
        || !top && !left && !right && bottom && left_top && !right_top && !left_bottom && !right_bottom // include bottom and left top
        || !top && !left && !right && bottom && !left_top && right_top && !left_bottom && !right_bottom // include bottom and right top
        || !top && !left && !right && bottom && !left_top && !right_top && left_bottom && !right_bottom // include bottom and left bottom
        || !top && !left && !right && bottom && !left_top && !right_top && !left_bottom && right_bottom // include bottom and right bottom
        || !top && !left && !right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // include bottom
    {
        return 30;
    } else if !top && !left && right && !bottom && left_top && right_top && left_bottom && right_bottom // exclude left and bottom and top
        || !top && !left && right && !bottom && left_top && right_top && left_bottom && !right_bottom // exclude left and bottom and top and right bottom
        || !top && !left && right && !bottom && left_top && right_top && !left_bottom && right_bottom // exclude left and bottom and top and left bottom
        || !top && !left && right && !bottom && left_top && !right_top && left_bottom && right_bottom // exclude left and bottom and top and right top
        || !top && !left && right && !bottom && !left_top && right_top && left_bottom && right_bottom // exclude left and bottom and top and left top
        || !top && !left && right && !bottom && left_top && right_top && !left_bottom && !right_bottom // include right and left top and right top
        || !top && !left && right && !bottom && !left_top && !right_top && left_bottom && right_bottom // include right and left bottom and right bottom
        || !top && !left && right && !bottom && left_top && !right_top && left_bottom && !right_bottom // include right and left top and left bottom
        || !top && !left && right && !bottom && !left_top && right_top && !left_bottom && right_bottom // include right and right top and right bottom
        || !top && !left && right && !bottom && !left_top && right_top && left_bottom && !right_bottom // include right and right top and left bottom
        || !top && !left && right && !bottom && left_top && !right_top && !left_bottom && right_bottom // include right and left top and right bottom
        || !top && !left && right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // include right and left top
        || !top && !left && right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // include right and right top
        || !top && !left && right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // include right and left bottom
        || !top && !left && right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // include right and right bottom
        || !top && !left && right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // include right
    {
        return 31;
    } else if !top && left && !right && !bottom && left_top && right_top && left_bottom && right_bottom // exclude right and bottom and top
        || !top && left && !right && !bottom && left_top && right_top && left_bottom && !right_bottom // exclude right and bottom and top and right bottom
        || !top && left && !right && !bottom && left_top && right_top && !left_bottom && right_bottom // exclude right and bottom and top and left bottom
        || !top && left && !right && !bottom && left_top && !right_top && left_bottom && right_bottom // exclude right and bottom and top and right top
        || !top && left && !right && !bottom && !left_top && right_top && left_bottom && right_bottom // exclude right and bottom and top and left top
        || !top && left && !right && !bottom && left_top && right_top && !left_bottom && !right_bottom // include left and left top and right top
        || !top && left && !right && !bottom && !left_top && !right_top && left_bottom && right_bottom // include left and left bottom and right bottom
        || !top && left && !right && !bottom && left_top && !right_top && left_bottom && !right_bottom // include left and left top and left bottom
        || !top && left && !right && !bottom && !left_top && right_top && !left_bottom && right_bottom // include left and right top and right bottom
        || !top && left && !right && !bottom && !left_top && right_top && left_bottom && !right_bottom // include left and right top and left bottom
        || !top && left && !right && !bottom && left_top && !right_top && !left_bottom && right_bottom // include left and left top and right bottom
        || !top && left && !right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // include left and left top
        || !top && left && !right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // include left and right top
        || !top && left && !right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // include left and left bottom
        || !top && left && !right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // include left and right bottom
        || !top && left && !right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // include left
    {
        return 32;
    } else if top && left && !right && !bottom && !left_top && right_top && left_bottom && right_bottom  // exclude right and bottom and left top
        || top && left && !right && !bottom && !left_top && right_top && !left_bottom && right_bottom // exclude right and bottom and left top and left bottom
        || top && left && !right && !bottom && !left_top && right_top && left_bottom && !right_bottom // exclude right and bottom and left top and right bottom
        || top && left && !right && !bottom && !left_top && !right_top && left_bottom && right_bottom // exclude right and bottom and left top and right top
        || top && left && !right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // include left and top and left bottom
        || top && left && !right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // include left and top and right bottom
        || top && left && !right && !bottom && !left_top && right_top && !left_bottom && !right_bottom // include left and top and right top
        || top && left && !right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // include left and top
    {
        return 33;
    } else if !top && left && !right && bottom && left_top && right_top && !left_bottom && right_bottom // exclude right and top and left bottom
        || !top && left && !right && bottom && !left_top && right_top && !left_bottom && right_bottom // exclude right and top and left bottom and left top
        || !top && left && !right && bottom && left_top && !right_top && !left_bottom && right_bottom // exclude right and top and left bottom and right top
        || !top && left && !right && bottom && left_top && right_top && !left_bottom && !right_bottom // exclude right and top and left bottom and right bottom
        || !top && left && !right && bottom && left_top && !right_top && !left_bottom && !right_bottom // include left and bottom and left top
        || !top && left && !right && bottom && !left_top && right_top && !left_bottom && !right_bottom // include left and bottom and right top
        || !top && left && !right && bottom && !left_top && !right_top && !left_bottom && right_bottom // include left and bottom and right bottom
        || !top && left && !right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // include left and top
    { 
        return 34;
    } else if top && !left && right && !bottom && left_top && !right_top && left_bottom && right_bottom // exclude left and bottom and right top
        || top && !left && right && !bottom && !left_top && !right_top && left_bottom && right_bottom // exclude left and bottom and right top and left top
        || top && !left && right && !bottom && left_top && !right_top && !left_bottom && right_bottom // exclude left and bottom and right top and left bottom
        || top && !left && right && !bottom && left_top && !right_top && left_bottom && !right_bottom // exclude left and bottom and right top and right bottom
        || top && !left && right && !bottom && left_top && !right_top && !left_bottom && !right_bottom // include right and top and left top
        || top && !left && right && !bottom && !left_top && !right_top && left_bottom && !right_bottom // include right and top and left bottom
        || top && !left && right && !bottom && !left_top && !right_top && !left_bottom && right_bottom // include right and top and right bottom
        || top && !left && right && !bottom && !left_top && !right_top && !left_bottom && !right_bottom // include right and top
    {
        return 35;
    } else if !top && !left && right && bottom && left_top && right_top && left_bottom && !right_bottom // exclude left and top and right bottom
        || !top && !left && right && bottom && left_top && right_top && !left_bottom && !right_bottom // exclude left and top and right bottom and left bottom
        || !top && !left && right && bottom && !left_top && right_top && left_bottom && !right_bottom // exclude left and top and right bottom and left top
        || !top && !left && right && bottom && left_top && !right_top && left_bottom && !right_bottom // exclude left and top and right bottom and right top
        || !top && !left && right && bottom && !left_top && !right_top && left_bottom && !right_bottom // include right and bottom and left bottom
        || !top && !left && right && bottom && left_top && !right_top && !left_bottom && !right_bottom // include right and bottom and left top
        || !top && !left && right && bottom && !left_top && right_top && !left_bottom && !right_bottom // include right and bottom and right top
        || !top && !left && right && bottom && !left_top && !right_top && !left_bottom && !right_bottom // include right and bottom
    {
        return 36;
    } else {
        // exclude left and right and top and bottom and other
        return 37;
    }
}
