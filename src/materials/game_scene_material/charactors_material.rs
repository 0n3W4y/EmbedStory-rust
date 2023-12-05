use bevy::prelude::*;
use crate::{config::TILE_SIZE, resources::scene_data::charactor::{RaceType, MonsterType, GenderType, CompanionType, NPCType}};

#[derive(Debug, Clone)]
pub struct CharactorsMaterial {
    pub player_material: RaceTypeMaterial,
    pub companion_material: CompanionMaterial,
    pub npc_material: NPCMAterial,
    pub monster_material: MonsterMaterial,
}

#[derive(Debug, Clone)]
pub struct RaceTypeMaterial {
    elf: RaceGenderMaterial,
    human: RaceGenderMaterial,
    skelton: RaceGenderMaterial,
    wolf: RaceGenderMaterial,
}

#[derive(Debug, Clone)]
pub struct RaceGenderMaterial {
    female: Handle<TextureAtlas>,
    male: Handle<TextureAtlas>,
}

#[derive(Debug, Clone)]
pub struct CompanionMaterial {
    knight: RaceTypeMaterial,
    berserk: RaceTypeMaterial,
    rouge: RaceTypeMaterial,
    bowman: RaceTypeMaterial,
    crossbowman: RaceTypeMaterial,
    fire_mage: RaceTypeMaterial,
    water_mage: RaceTypeMaterial,

}

#[derive(Debug, Clone)]
pub struct NPCMAterial {
    pub trader: RaceTypeMaterial,
}

#[derive(Debug, Clone)]
pub struct MonsterMaterial {
    melee: RaceTypeMaterial,
    range: RaceTypeMaterial,
    magic: RaceTypeMaterial,
}

impl CharactorsMaterial {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> CharactorsMaterial {
        load_charactors_material(asset_server, texture_atlases)
    }

    pub fn get_monster_atlas(
        &self,
        monster_race: &RaceType,
        monster_type: &MonsterType,
        gender_type: &GenderType,
    ) -> Handle<TextureAtlas> {
        match *monster_race {
            RaceType::Human => {
                match *gender_type {
                    GenderType::Female => {
                        match *monster_type {
                            MonsterType::Melee => return self.monster_material.melee.human.female.clone_weak(),
                            MonsterType::Ranged => return self.monster_material.range.human.female.clone_weak(),
                            MonsterType::Magic => return self.monster_material.magic.human.female.clone_weak(),
                        };
                    },
                    GenderType::Male => {
                        match *monster_type {
                            MonsterType::Melee => return self.monster_material.melee.human.male.clone_weak(),
                            MonsterType::Ranged => return self.monster_material.range.human.male.clone_weak(),
                            MonsterType::Magic => return self.monster_material.magic.human.male.clone_weak(),
                        };
                    }
                }
            },
            RaceType::Elf => {
                
            },
            RaceType::Orc => {
                
            },
            RaceType::Dwarf => {
                
            },
            RaceType::Halfling => {
                
            },
            RaceType::Lizardfolk => {
                
            },
            RaceType::Naga => {
                
            },
            RaceType::Gnome => {
                
            },
            RaceType::Goblin => {
                
            },
            RaceType::Minotaur => {
                
            },
            RaceType::Harpia => {
                
            },
            RaceType::Dryada => {
                
            },
            RaceType::Fairy => {
                
            },
            RaceType::Celestial => {
                
            },
            RaceType::Elemental => {
                
            },
            RaceType::Skeleton => {
                
            },
            RaceType::Zombie => {
                
            },
            RaceType::Ogre => {
                
            },
            RaceType::Demon => {
                
            },
            RaceType::Wolf => {
                
            },
            RaceType::Bear => {
                
            },
            RaceType::Crocodile => {
                
            },
            RaceType::Scorpion => {
                
            },
            RaceType::Eagle => {
                
            },
            RaceType::Spider => {
                
            },
            RaceType::KomodoDragon => {
                
            },
            RaceType::Rhinocerops => {
                
            },
            RaceType::Snake => {
                
            },
            RaceType::Ghost => {
                
            },
        }
        
        println!("Check config. Try to get texture with Race: {:?}, monster: {:?}, gender: {:?}. Returning human_male_atlas", monster_race, monster_type, gender_type);
        return self.monster_material.melee.human.male.clone_weak();

    }

    pub fn get_companion_atlas(&self, race_type: &RaceType, companion_type: &CompanionType, gender_type: &GenderType) -> Handle<TextureAtlas> {
        match *race_type {
            RaceType::Human => {
                match *gender_type {
                    GenderType::Female => {
                        match *companion_type {
                            CompanionType::Knight => todo!(),
                            CompanionType::Berserk => todo!(),
                            CompanionType::Rouge => todo!(),
                            CompanionType::Bowman => todo!(),
                            CompanionType::Crossbowman => todo!(),
                            CompanionType::FireMage => todo!(),
                            CompanionType::WaterMage => todo!(),
                        }
                    },
                    GenderType::Male => {
                        match *companion_type {
                            CompanionType::Knight => todo!(),
                            CompanionType::Berserk => todo!(),
                            CompanionType::Rouge => todo!(),
                            CompanionType::Bowman => todo!(),
                            CompanionType::Crossbowman => todo!(),
                            CompanionType::FireMage => todo!(),
                            CompanionType::WaterMage => todo!(),
                        }
                    },
                }
            },
            RaceType::Elf => {
                
            },
            RaceType::Orc => {
                
            },
            RaceType::Dwarf => {
                
            },
            RaceType::Halfling => {
                
            },
            RaceType::Lizardfolk => {
                
            },
            RaceType::Naga => {
                
            },
            RaceType::Gnome => {
                
            },
            RaceType::Goblin => {
                
            },
            RaceType::Minotaur => {
                
            },
            RaceType::Harpia => {
                
            },
            RaceType::Dryada => {
                
            },
            RaceType::Fairy => {
                
            },
            RaceType::Celestial => {
                
            },
            RaceType::Elemental => {
                
            },
            RaceType::Skeleton => {
                
            },
            RaceType::Zombie => {
                
            },
            RaceType::Ogre => {
                
            },
            RaceType::Demon => {
                
            },
            RaceType::Wolf => {
                
            },
            RaceType::Bear => {
                
            },
            RaceType::Crocodile => {
                
            },
            RaceType::Scorpion => {
                
            },
            RaceType::Eagle => {
                
            },
            RaceType::Spider => {
                
            },
            RaceType::KomodoDragon => {
                
            },
            RaceType::Rhinocerops => {
                
            },
            RaceType::Snake => {
                
            },
            RaceType::Ghost => {
                
            },
        }
        println!("Check config. Try to get texture with race: {:?}, companion: {:?}, gender: {:?}. Returning human_knight", race_type, companion_type, gender_type);
        return self.companion_material.knight.human.male.clone_weak();
    }

    pub fn get_player_atlas(&self, race_type: &RaceType, gender_type: &GenderType) -> Handle<TextureAtlas> {
        match *race_type {
            RaceType::Human => {
                match *gender_type {
                    GenderType::Female => self.player_material.human.female.clone_weak(),
                    GenderType::Male => self.player_material.human.male.clone_weak(),
                }
            },
            RaceType::Elf => todo!(),
            RaceType::Orc => todo!(),
            RaceType::Dwarf => todo!(),
            RaceType::Halfling => todo!(),
            RaceType::Lizardfolk => todo!(),
            RaceType::Naga => todo!(),
            RaceType::Gnome => todo!(),
            RaceType::Goblin => todo!(),
            RaceType::Minotaur => todo!(),
            RaceType::Harpia => todo!(),
            RaceType::Dryada => todo!(),
            RaceType::Fairy => todo!(),
            RaceType::Celestial => todo!(),
            RaceType::Elemental => todo!(),
            RaceType::Skeleton => todo!(),
            RaceType::Zombie => todo!(),
            RaceType::Ogre => todo!(),
            RaceType::Demon => todo!(),
            RaceType::Wolf => todo!(),
            RaceType::Bear => todo!(),
            RaceType::Crocodile => todo!(),
            RaceType::Scorpion => todo!(),
            RaceType::Eagle => todo!(),
            RaceType::Spider => todo!(),
            RaceType::KomodoDragon => todo!(),
            RaceType::Rhinocerops => todo!(),
            RaceType::Snake => todo!(),
            RaceType::Ghost => todo!(),
        }
    }

    pub fn get_npc_atlas(&self, race_type: &RaceType, npc_type: &NPCType, gender_type: &GenderType) -> Handle<TextureAtlas> {
        match *race_type {
            RaceType::Human => {
                match *gender_type {
                    GenderType::Female => {
                        match *npc_type {
                            NPCType::PotionTrader => todo!(),
                            NPCType::BlackSmith => todo!(),
                            NPCType::TrinketTrader => todo!(),
                        }
                    },
                    GenderType::Male => todo!(),
                }
            },
            RaceType::Elf => todo!(),
            RaceType::Orc => todo!(),
            RaceType::Dwarf => todo!(),
            RaceType::Halfling => todo!(),
            RaceType::Lizardfolk => todo!(),
            RaceType::Naga => todo!(),
            RaceType::Gnome => todo!(),
            RaceType::Goblin => todo!(),
            RaceType::Minotaur => todo!(),
            RaceType::Harpia => todo!(),
            RaceType::Dryada => todo!(),
            RaceType::Fairy => todo!(),
            RaceType::Celestial => todo!(),
            RaceType::Elemental => todo!(),
            RaceType::Skeleton => todo!(),
            RaceType::Zombie => todo!(),
            RaceType::Ogre => todo!(),
            RaceType::Demon => todo!(),
            RaceType::Ghost => todo!(),
            RaceType::Wolf => todo!(),
            RaceType::Bear => todo!(),
            RaceType::Crocodile => todo!(),
            RaceType::Scorpion => todo!(),
            RaceType::Eagle => todo!(),
            RaceType::Spider => todo!(),
            RaceType::KomodoDragon => todo!(),
            RaceType::Rhinocerops => todo!(),
            RaceType::Snake => todo!(),
        }
    }
}

fn load_charactors_material(
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> CharactorsMaterial {

    //players material:
    let player_human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/male/player.png"); 
    let player_human_female_texture_handle: Handle<Image> = asset_server.load("textures/charactor/human/female/player.png");

    let player_human_male_texture_atlas = TextureAtlas::from_grid(
        player_human_male_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
    );
    let player_human_female_texture_atlas = TextureAtlas::from_grid(
        player_human_female_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
    );

    let player_human_male_atlas = texture_atlases.add(player_human_male_texture_atlas);
    let player_human_female_atlas = texture_atlases.add(player_human_female_texture_atlas);
    let player_human_material = RaceGenderMaterial {
        male: player_human_male_atlas,
        female: player_human_female_atlas,
    };

    let player_material = PlayerMaterial {
        human: player_human_material,
    };


    //companion materials: 

    let companion_human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/male/companion.png"); 
    let companion_human_female_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/female/companion.png"); 

    let compnaion_human_male_texture_atlas = TextureAtlas::from_grid(
        companion_human_male_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
    );
    let companion_human_female_texture_atlas = TextureAtlas::from_grid(
        companion_human_female_texture_handle,
        Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32),
        4,
        1,
    );

    let companion_human_male_atlas = texture_atlases.add(player_human_male_texture_atlas);
    let compnaion_human_female_atlas = texture_atlases.add(player_human_female_texture_atlas);
    let compnaion_human_material = RaceGenderMaterial {
        male: companion_human_male_atlas,
        female: compnaion_human_female_atlas,
    };

    let companion_material = CompanionMaterial {
        human: compnaion_human_material,
        knight: todo!(),
        berserk: todo!(),
        rouge: todo!(),
        bowman: todo!(),
        crossbowman: todo!(),
        fire_mage: todo!(),
        water_mage: todo!(),
    };

    //NPCs materials: 
    let npc_human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/male/npc.png"); 
    let npc_human_female_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/female/npc.png"); 

    let npc_material = NPCMAterial {
        trader: todo!(),
    };

    //monsters materials: 
    let monster_human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/male/monster.png"); 
    let monster_human_male_texture_handle: Handle<Image> = asset_server.load("textures/charactors/human/female/monster.png"); 

    let monster_material = MonsterMaterial {
        melee: todo!(),
        range: todo!(),
        magic: todo!(),
    };   

    CharactorsMaterial {
        player_material,
        companion_material,
        npc_material,
        monster_material,
    }
}