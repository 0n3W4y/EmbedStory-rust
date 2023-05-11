use bevy::prelude::*;

use crate::resources::scene_manager::SceneManager;
use crate::materials::material_manager::MaterialManager;
use crate::scenes::game_scenes::game_scene::GameScene;
use crate::resources::scene_data::objects::charactor::Charactor;
use crate::components::charactor_component::CharactorComponent;

pub fn draw(
    mut commands: Commands,
    scene_manager: Res<SceneManager>,
    material_manager: Res<MaterialManager>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
){
    let scene: &GameScene = scene_manager.get_current_game_scene();
    let total_tiles = scene.tilemap.get_total_tiles();
    for charactor in scene.charactors.iter(){
        let x: f32 = charactor.graphic_position.x;
        let y: f32 = charactor.graphic_position.y;
        let charactor_type = charactor.charactor_type;
        let charactor_subtype = charactor.charactor_subtype;
        let charactor_gender = charactor.gender_type;


        let texture_handle: Handle<Image> = material_manager.game_scene.charactors.get_image();
        let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
        let texture_atlas_handle = texture_atlases.add(texture_atlas);

        let mut charactor_component: CharactorComponent = Default::default();
        copy_from_charactor_to_component(charactor, &mut charactor_component);
    }

}

pub fn copy_from_charactor_to_component(
    charactor: &Charactor,
    charactor_component: &mut CharactorComponent,
){

}
pub fn copy_from_component_to_charactor(
    charactor: &mut Charactor,
    charactor_component: CharactorComponent,
){}