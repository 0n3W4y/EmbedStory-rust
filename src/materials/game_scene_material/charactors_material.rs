use bevy::prelude::*;
use crate::{config::TILE_SIZE, resources::scene_data::charactor::{RaceType, GenderType}};

#[derive(Debug, Clone)]
pub struct CharactorsMaterial {
   pub human: RaceGenderMaterial,
   pub skeleton: RaceGenderMaterial,
   pub wolf: RaceGenderMaterial,
}

#[derive(Debug, Clone)]
pub struct RaceGenderMaterial {
    female: Handle<TextureAtlas>,
    male: Handle<TextureAtlas>,
}

impl CharactorsMaterial {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> CharactorsMaterial {
        load_charactors_material(asset_server, texture_atlases)
    }

    pub fn get_atlas(
        &self,
        race: &RaceType,
        gender: &GenderType,
    ) -> Handle<TextureAtlas> {
        match *gender {
            GenderType::Female => {
                match *race {
                    RaceType::Human => self.human.female.clone_weak(),
                    RaceType::Elf => todo!(),
                    RaceType::Orc => todo!(),
                    RaceType::Dwarf => todo!(),
                    RaceType::Halfling => todo!(),
                    RaceType::Lizardfolk => todo!(),
                    RaceType::Naga => todo!(),
                    RaceType::Gnome => todo!(),
                    RaceType::Goblin => todo!(),
                    RaceType::Minotaur => todo!(),
                    RaceType::Skeleton => self.skeleton.female.clone_weak(),
                    RaceType::Zombie => todo!(),
                    RaceType::Ogre => todo!(),
                    RaceType::Harpia => todo!(),
                    RaceType::Dryada => todo!(),
                    RaceType::Fairy => todo!(),
                    RaceType::Celestial => todo!(),
                    RaceType::Elemental => todo!(),
                    RaceType::Ghost => todo!(),
                    RaceType::Demon => todo!(),
                    RaceType::Wolf => self.wolf.female.clone_weak(),
                    RaceType::Bear => todo!(),
                    RaceType::Crocodile => todo!(),
                    RaceType::Scorpion => todo!(),
                    RaceType::Eagle => todo!(),
                    RaceType::Spider => todo!(),
                    RaceType::KomodoDragon => todo!(),
                    RaceType::Rhinocerops => todo!(),
                    RaceType::Snake => todo!(),
                }
            },
            GenderType::Male => {
                match *race {
                    RaceType::Human => self.human.male.clone_weak(),
                    RaceType::Elf => todo!(),
                    RaceType::Orc => todo!(),
                    RaceType::Dwarf => todo!(),
                    RaceType::Halfling => todo!(),
                    RaceType::Lizardfolk => todo!(),
                    RaceType::Naga => todo!(),
                    RaceType::Gnome => todo!(),
                    RaceType::Goblin => todo!(),
                    RaceType::Minotaur => todo!(),
                    RaceType::Skeleton => self.skeleton.male.clone_weak(),
                    RaceType::Zombie => todo!(),
                    RaceType::Ogre => todo!(),
                    RaceType::Harpia => todo!(),
                    RaceType::Dryada => todo!(),
                    RaceType::Fairy => todo!(),
                    RaceType::Celestial => todo!(),
                    RaceType::Elemental => todo!(),
                    RaceType::Ghost => todo!(),
                    RaceType::Demon => todo!(),
                    RaceType::Wolf => self.wolf.male.clone_weak(),
                    RaceType::Bear => todo!(),
                    RaceType::Crocodile => todo!(),
                    RaceType::Scorpion => todo!(),
                    RaceType::Eagle => todo!(),
                    RaceType::Spider => todo!(),
                    RaceType::KomodoDragon => todo!(),
                    RaceType::Rhinocerops => todo!(),
                    RaceType::Snake => todo!(),
                }
            },
        }
    }
}

fn load_charactors_material(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> CharactorsMaterial {

    let human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/male.png"); 
    let human_female_texture_handle: Handle<Image> = asset_server.load("textures/charactor/human/female.png");
    let skeleton_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/skeleton/male.png"); 
    let skeleton_female_texture_handle: Handle<Image> = asset_server.load("textures/charactor/skeleton/female.png");
    let wolf_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/wolf/male.png"); 
    let wolf_female_texture_handle: Handle<Image> = asset_server.load("textures/charactor/wolf/female.png");

    let human_male_texture_atlas = TextureAtlas::from_grid(
        human_male_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );
    let human_female_texture_atlas = TextureAtlas::from_grid(
        human_female_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );
    let skeleton_male_texture_atlas = TextureAtlas::from_grid(
        skeleton_male_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );
    let skeleton_female_texture_atlas = TextureAtlas::from_grid(
        skeleton_female_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );
    let wolf_male_texture_atlas = TextureAtlas::from_grid(
        wolf_male_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );
    let wolf_female_texture_atlas = TextureAtlas::from_grid(
        wolf_female_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
        None,
        None,
    );

    let human_male_atlas = texture_atlases.add(human_male_texture_atlas);
    let human_female_atlas = texture_atlases.add(human_female_texture_atlas);
    let skeleton_male_atlas = texture_atlases.add(skeleton_male_texture_atlas);
    let skeleton_female_atlas = texture_atlases.add(skeleton_female_texture_atlas);
    let wolf_male_atlas = texture_atlases.add(wolf_male_texture_atlas);
    let wolf_female_atlas = texture_atlases.add(wolf_female_texture_atlas);

    let human_material = RaceGenderMaterial {
        male: human_male_atlas,
        female: human_female_atlas,
    };

    let skeleton_material = RaceGenderMaterial {
        male: skeleton_male_atlas,
        female: skeleton_female_atlas,
    };

    let wolf_material = RaceGenderMaterial {
        male: wolf_male_atlas,
        female: wolf_female_atlas,
    };

    CharactorsMaterial {
        human: human_material,
        skeleton: skeleton_material,
        wolf: wolf_material,
    }
}