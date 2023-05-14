use bevy::prelude::*;

pub fn setup_window_settings(mut windows: Query<&mut Window>){
    for mut window in &mut windows {
        window.title = "Snake".to_string();
        //window.resolution.set(500., 500.);
    }
}

pub fn setup_camera(mut commands : Commands){
    commands.spawn(Camera2dBundle::default());
} 
