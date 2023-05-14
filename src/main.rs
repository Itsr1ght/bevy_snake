mod game;
mod snake;
mod event;
mod grid;
mod utils;
mod food;

use crate::game::*;
use crate::event::*;
use crate::snake::*;
use crate::food::*;
use crate::grid::*;

use bevy::{DefaultPlugins,
            prelude::*,
            time::common_conditions::on_timer            
};


fn main() {
    use std::time::Duration;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(SnakeTails::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_window_settings)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_systems(
            (
                snake_movement_input,
                snake_movement.run_if(on_timer(Duration::from_secs_f32(0.150)))
            ).chain())
        .add_system(snake_eating.after(snake_movement))
        .add_system(snake_growth.after(snake_eating))
        .add_system(position_translation)
        .add_system(food_spawner.run_if(on_timer(Duration::from_secs_f32(1.))))
        .add_system(size_scaling)
        .add_system(game_over.after(snake_movement))
        .run();
}
