mod game;
mod snake;
mod event;
mod grid;
mod utils;
mod food;

use crate::game::GamePlugin;
use crate::event::EventPlugin;
use crate::snake::SnakePlugin;
use crate::food::FoodPlugin;
use crate::grid::GridPlugin;

use bevy::{
    DefaultPlugins,
    prelude::*
};


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(EventPlugin)
        .add_plugin(GridPlugin)
        .add_plugin(FoodPlugin)
        .add_plugin(GamePlugin)
        .add_plugin(SnakePlugin)
        .run();
}
