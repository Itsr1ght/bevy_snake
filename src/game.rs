use bevy::prelude::*;

fn setup_window_settings(mut windows: Query<&mut Window>){
    for mut window in &mut windows {
        window.title = "Snake".to_string();
        window.resolution.set(500., 500.);
    }
}

fn setup_camera(mut commands : Commands){
    commands.spawn(Camera2dBundle::default());
}


pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App)
    {
        app.add_startup_system(setup_window_settings)
        .add_startup_system(setup_camera);
    }
}


