use rand::{ thread_rng, Rng};
use bevy::{
    input::mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    prelude::*,
    window::{CursorMoved, PresentMode,},
    diagnostic::{ Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin },
    core::FixedTimestep,
};

#[derive( Component )]
struct Character{
    fraction: Fraction,
}

enum Fraction{
    Player,
    Enemy,
    Ally,
}

#[derive(Component)]
struct MainCamera{
    move_detection:u8,
    cursor_position:Vec2,
}

#[derive(Component)]
struct Tile{
    position: Position,
}

struct TimerOneSecond( Timer );

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i8,
    y: i8,
}

#[derive( Component )]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Component)]
struct StatsText;

struct BevyCounter {
    pub count: usize,
    pub color: Color,
}

impl Size {
    pub fn square( x: f32 ) -> Self{
        Self{
            width: x,
            height: x,
        }
    }
}

#[derive( Component )]
struct Move{
    speed: u16,
    direction_x:i8,
    direction_y:i8,
    status:MovingStatus,
    point: Position,
}

enum MovingStatus{
    Standing,
    Moving,
}

impl Move {
    pub fn calculate_movement( &mut self, x:i8, y:i8 ){
        let mut dir_x = self.point.x - x;
        let mut dir_y = self.point.x - y;
        if dir_x < 0 {
            dir_x = -1;
        }else if dir_x > 0{
            dir_x = 1;
        }

        if dir_y < 0 {
            dir_y = -1;
        }else if dir_y > 0{
            dir_y = 1;
        }
        self.direction_x = dir_x as i8;
        self.direction_y = dir_y as i8;

        if dir_x == 0 && dir_y == 0 {
            self.status = MovingStatus::Standing;
        }
    }
}




const CHARACTER_PLAYER_COLOR:Color = Color::rgb( 0.0, 1.0, 0.0 );
const CHARACTER_ENEMY_COLOR:Color = Color::rgb( 1.0, 0.0, 0.0 );
const CHARACTER_ALLY_COLOR:Color = Color::rgb( 0.0, 0.0, 1.0 );
const SPRITE_SIZE:u8 = 128;
const GRID_WIDTH:u8 = 200;
const GRID_HEIGHT:u8 = 200;
const MAX_LR_GRID: i8 = ( GRID_WIDTH / 2 ) as i8;
const MAX_UD_GRID: i8 = ( GRID_HEIGHT / 2 ) as i8;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor { 
            title: "test".to_string(), 
            width: 1280.0,                 
            height: 768.0,
            present_mode: PresentMode::Immediate,
            resizable: true,             
            ..default()
        })
        .add_plugins( DefaultPlugins )
        .insert_resource( TimerOneSecond( Timer::from_seconds( 5.0, true )))
        //.add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system( spawn_grid )
        .add_startup_system( add_camera )
        .add_startup_system( spawn_player )
        .add_startup_system( spawn_enemy)
        .add_startup_system( spawn_ally )
        .add_startup_system( spawn_text_bundl )
        .insert_resource(BevyCounter {
            count: 0,
            color: Color::WHITE,
        })
        .add_system( print_mouse_events_system )
        .add_system( mouse_click_system_for_player )
        .add_system( generate_move_point_for_characters )
        .add_system( move_character )
        .add_system( camera_zoom )
        .add_system( camera_move_by_mouse )
        .add_system(counter_system)
        //.add_system( trace_info )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
               .with_system(size_scaling),
        )
        .run();
}

fn add_camera( mut commands: Commands ){
    commands.spawn()
            .insert_bundle( OrthographicCameraBundle::new_2d() )
            .insert( MainCamera{ move_detection: 0, cursor_position: Vec2::new( 0.0, 0.0 )});
    commands.spawn_bundle(UiCameraBundle::default());
}

fn spawn_grid( mut commands: Commands, asset_server: Res<AssetServer> ){
    let grid_texture = asset_server.load("images/grid_tile_128.png");
    let mut y:f32 = -MAX_UD_GRID as f32 * SPRITE_SIZE as f32;
    let mut pos_y:i8 = -MAX_UD_GRID;
    for _ in 0..GRID_HEIGHT {
        let mut x:f32 = -MAX_LR_GRID as f32 * SPRITE_SIZE as f32;
        let mut pos_x:i8 = -MAX_LR_GRID;
        for _ in 0..GRID_WIDTH {
            commands.spawn_bundle( SpriteBundle {
                texture: grid_texture.clone(),
                transform: Transform { translation: Vec3::new( x , y , 0.0 ),
                scale: Vec3::new(1.0, 1.0, 0.0),
                ..default() },
            ..default()
            })
            .insert( Tile{ position: Position{ x: pos_x ,y: pos_y }});
            x += SPRITE_SIZE as f32;
            pos_x += 1;
        }
        y += SPRITE_SIZE as f32;
        pos_y += 1;
    }    
}



fn spawn_player( mut commands: Commands ){
    //let mut rnd = thread_rng();
    //let pos_x:i32 = rnd.gen_range( 0..=10 );
    //let pos_y:i32 = rnd.gen_range( 0..=10 );
    commands
        .spawn_bundle( SpriteBundle { 
            sprite: Sprite{ color: CHARACTER_PLAYER_COLOR, ..default() },
            transform: Transform { translation: Vec3::new(0.0, 0.0, 3.0),..default() },
            ..default()
        })
        .insert( Character{ fraction: Fraction::Player })
        .insert( Position{ x: 0, y: 0 })
        .insert( Size::square( 1.0 ))
        .insert( Move{ speed: 1150, direction_x: 0, direction_y: 0, status: MovingStatus::Standing, point: Position{ x: 0, y: 0 }});
}

fn spawn_enemy( mut commands: Commands ){
    let mut rnd = thread_rng();
    let pos_x:i8 = rnd.gen_range( -MAX_LR_GRID..MAX_LR_GRID );
    let pos_y:i8 = rnd.gen_range( -MAX_UD_GRID..MAX_UD_GRID );
    let sprite_pos_x:f32 = pos_x as f32 * SPRITE_SIZE as f32;
    let sprite_pos_y:f32 = pos_y as f32 * SPRITE_SIZE as f32;
    commands
        .spawn_bundle( SpriteBundle { 
            sprite: Sprite{ color: CHARACTER_ENEMY_COLOR, ..default() },
            transform: Transform { translation: Vec3::new( sprite_pos_x, sprite_pos_y, 1.0),..default() },
            ..default()
        })
        .insert( Character{ fraction: Fraction::Enemy })
        .insert( Position{ x: pos_x, y: pos_y })
        .insert( Move{ speed: 1100, direction_x: 0, direction_y: 0, status: MovingStatus::Standing, point: Position{ x: pos_x, y: pos_y }})
        .insert( Size::square( 1.0 ));
}

fn spawn_ally( mut commands: Commands ){
    let mut rnd = thread_rng();
    let pos_x:i8 = rnd.gen_range( -MAX_LR_GRID..MAX_LR_GRID );
    let pos_y:i8 = rnd.gen_range( -MAX_UD_GRID..MAX_UD_GRID );
    let sprite_pos_x:f32 = pos_x as f32 * SPRITE_SIZE as f32;
    let sprite_pos_y:f32 = pos_y as f32 * SPRITE_SIZE as f32;
    commands
        .spawn_bundle( SpriteBundle { 
            sprite: Sprite{ color: CHARACTER_ALLY_COLOR, ..default() },
            transform: Transform { translation: Vec3::new( sprite_pos_x, sprite_pos_y, 2.0),..default() },
            ..default()
        })
        .insert( Character{ fraction: Fraction::Ally })
        .insert( Position{ x: pos_x, y: pos_y })
        .insert( Move{ speed: 1000, direction_x: 0, direction_y: 0, status: MovingStatus::Standing, point: Position{ x: pos_x, y: pos_y }})
        .insert( Size::square( 1.0 ));
}

fn spawn_text_bundl( mut commands: Commands, asset_server: Res<AssetServer> ){
    commands
        .spawn_bundle(TextBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Bird Count: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 1.0, 0.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 1.0, 1.0),
                        },
                    },
                    TextSection {
                        value: "\nAverage FPS: ".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 1.0, 0.0),
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.0, 1.0, 1.0),
                        },
                    },
                ],
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(5.0),
                    left: Val::Px(5.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .insert(StatsText);
}

fn generate_move_point_for_characters( time: Res<Time>, mut timer: ResMut<TimerOneSecond>, mut enemy: Query<(&mut Move, &Character ),  With<Character>>){
    if timer.0.tick(time.delta()).just_finished() {
        for ( mut move_direction, character ) in enemy.iter_mut() {
            match move_direction.status {
                MovingStatus::Standing => {
                    match character.fraction {
                        Fraction::Player => return,
                        Fraction::Enemy  => generate_move_point( &mut move_direction, "Enemy".to_string() ),
                        Fraction::Ally => generate_move_point( &mut move_direction, "Ally".to_string() ),
                    }
                },
                MovingStatus::Moving => {
                    return;
                    //let char_fraction = &character.fraction;
                    //info!( "Character: {} on move", char_fraction );
                }
            }
        }
    }
}

fn generate_move_point( pos: &mut Mut<Move>, string:String ) {
        let mut rnd = thread_rng();
        let x:i8 = rnd.gen_range( -MAX_LR_GRID..=MAX_LR_GRID );
        let y:i8 = rnd.gen_range( -MAX_UD_GRID..=MAX_UD_GRID );
        pos.point.x = x;
        pos.point.y = y;
        pos.status = MovingStatus::Moving;
        info!( "Character: {}; goto x:{}, y:{} ", string, x, y );
}
/*
fn set_destination_for_player_by_click( x:i8, y:i8, &mut player: ){
    for( mut move_direction, character ) in player.iter_mut(){
        if character.fraction != "player".to_string(){
            return;
        }else{
            move_direction.point.x = x;
            move_direction.point.y = y;
            move_direction.status = MovingStatus::Moving;
        }
    }
}
*/
/*
fn trace_info( character: Query<( &Position, &Transform, &Character ), With<Character>> ){
    for( pos, transform, character ) in character.iter(){
        if character.fraction == "enemy".to_string() {
            let pos_x = pos.x;
            let pos_y = pos.y;
            let sprite_x = transform.translation.x;
            let sprite_y = transform.translation.y;
            //info!( "Current Pos x:{}, y:{}; Current Sprite Pos x:{}, y:{}", pos_x, pos_y, sprite_x, sprite_y );
        }        
    }
}
*/
fn move_character( mut enemy: Query<( &mut Move, &mut Transform, &mut Position ), With<Character>> ){
    for ( mut move_direction, mut transform, mut position ) in enemy.iter_mut(){
        match move_direction.status {
            MovingStatus::Standing => {
                return;
            },
            MovingStatus::Moving => {
                move_direction.calculate_movement( position.x, position.y );
                if transform.translation.x < MAX_LR_GRID as f32 || transform.translation.x > -MAX_LR_GRID as f32 {
                    transform.translation.x += move_direction.direction_x as f32 * ( move_direction.speed / 1000 ) as f32;
                    if move_direction.direction_x > 0{
                        let x:i8 = ( transform.translation.x / SPRITE_SIZE as f32 ).floor() as i8;
                        if x > position.x {
                            position.x += 1;
                            //info!( "x: {}; Sprite_x: {} ",position.x, transform.translation.x );
                        }
                    }else if move_direction.direction_x < 0{
                        let x:i8 = ( transform.translation.x / SPRITE_SIZE as f32 ).ceil() as i8;
                        if x < position.x {
                            position.x -= 1;
                            //info!( "y: {}; Sprite_y: {} ",position.y, transform.translation.y );
                        }
                    }                    
                }
                
                if transform.translation.y < MAX_UD_GRID as f32 || transform.translation.y > -MAX_UD_GRID as f32 {
                    transform.translation.y += move_direction.direction_y as f32 * ( move_direction.speed / 1000 ) as f32;
                    if move_direction.direction_y > 0{
                        let y:i8 = ( transform.translation.y / SPRITE_SIZE as f32 ).floor() as i8;
                        if y > position.y {
                            position.y += 1;
                        }
                    }else if move_direction.direction_x < 0{
                        let y:i8 = ( transform.translation.y / SPRITE_SIZE as f32 ).ceil() as i8;
                        if y < position.y {
                            position.y -= 1;
                        }
                    } 
                }
            },
        }        
    }    
}


fn size_scaling( mut query: Query<(&Size, &mut Transform )>){
    for( sprite_size, mut transform ) in query.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width * SPRITE_SIZE as f32,
            sprite_size.height * SPRITE_SIZE as f32,
            1.0,
        );
    }
}

fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
) {
    for event in mouse_button_input_events.iter() {
        //info!("{:?}", event);
        //let button_type = event.button;
        //info!( "Button type = {:?}", button_type )
        //button: Left, state Pressed;
        //button Right, state Realesed;
    }

    for event in mouse_motion_events.iter() {
        //info!("{:?}", event);
    }

    for event in cursor_moved_events.iter() {

    }

    for event in mouse_wheel_events.iter() {
        //info!("{:?}", event);
    }
}

fn mouse_click_system_for_player( windows: Res<Windows>, mouse_button_input: Res<Input<MouseButton>>, tile: Query<&Position, With<Tile>>, camera: Query< (&Transform, &OrthographicProjection), With<MainCamera>>) {
    let window = windows.get_primary().unwrap();
    let mut x:f32 = 0.0;
    let mut y:f32 = 0.0;
    let mut cam_x:f32 = 0.0;
    let mut cam_y:f32 = 0.0;
    let mut camera_scale: f32 = 0.0;
    if mouse_button_input.pressed(MouseButton::Left) {
        //info!("left mouse currently pressed");
        if let Some(_position) = window.cursor_position() {
            x = _position.x;
            y = _position.y;
            info!( "x:{}, y:{}", x, y );
            for ( transform, projection ) in camera.iter(){
                cam_x = transform.translation.x;
                cam_y = transform.translation.y;
                camera_scale = projection.scale.ln();
            }
            info!( "cam x:{}, cam y:{}, cam scale:{}", cam_x, cam_y, camera_scale );
            //TODO: camera position, camera scale, 
        } else {
            // cursor is not inside the window
        }
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        //info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        //info!("left mouse just released");
    }
}

fn camera_zoom( mut wheel_input: EventReader<MouseWheel>, mut camera: Query< &mut OrthographicProjection , With<MainCamera>> ){
    let mut scaling = 0.0;
        for event in wheel_input.iter(){
            scaling = event.y;
        }
        if scaling != 0.0{
            scaling = scaling / ( -4.0 );
            for mut projection in camera.iter_mut(){
                let mut log_scale = projection.scale.ln();
                log_scale += scaling;
                if log_scale >= 2.0 || log_scale <= 0.2 {
                    return;
                }
                projection.scale = log_scale.exp();    
            }
        }else{
            return;
        } 
}

fn camera_move_by_mouse(mouse_button_input: Res<Input<MouseButton>>, mut cursor_moved_events: EventReader<CursorMoved>, mut camera: Query<( &mut Transform, &mut MainCamera, &OrthographicProjection ), With<MainCamera>>  ){
    if mouse_button_input.pressed( MouseButton::Left ) {
        for ( mut transform, mut cam, projection ) in camera.iter_mut(){
            if cam.move_detection >= 2{
                for event in cursor_moved_events.iter() {
                    if cam.cursor_position.x == 0.0 {
                        cam.cursor_position.x = event.position.x;
                        cam.cursor_position.y = event.position.y;
                    }
                    let dif_x = cam.cursor_position.x - event.position.x;
                    let dif_y = cam.cursor_position.y - event.position.y;
                    transform.translation.x += dif_x * projection.scale;
                    transform.translation.y += dif_y * projection.scale;

                    cam.cursor_position.x = event.position.x;
                    cam.cursor_position.y = event.position.y;               
                }
            }else{
                cam.move_detection += 1;
            }
        }
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        for ( _, mut cam, _ ) in camera.iter_mut(){
            cam.move_detection = 0;
            cam.cursor_position.x = 0.0;
            cam.cursor_position.y = 0.0;
        }
    }
}

fn counter_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<StatsText>>,
) {
    let mut text = query.single_mut();


    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(average) = fps.average() {
            text.sections[3].value = format!("{:.2}", average);
        }
    };
}

