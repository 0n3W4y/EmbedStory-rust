enum MainButtonComponent{
    Return,
    Start,
}

enum IncreaseDecreseButtonComponent{

}

pub struct CreatCharatcterPlugin;

impl Plugin for CreatCharatcterPlugin{
    fn build( &self, app: &mut App ){
        //app.add_system_set(SystemSet::on_enter( SceneState::MainMenuScene ).with_system( setup ));
        //app.add_system_set( SystemSet::on_update( SceneState::MainMenuScene ).with_system( button_handle_system ));
        app.add_system_set( SystemSet::on_exit( SceneState::MainMenuScene ).with_system( cleanup ));
    }
}

fn cleanup(){}