use crate::utils::{Size, Position};
use bevy::prelude::IntoSystemConfig;
use bevy::prelude::{
    Commands,
    SpriteBundle,
    Sprite,
    default,
    Color,
    Plugin,
    App
};
use bevy::time::common_conditions::on_timer; 
use rand::random;
use crate::grid::{ARENA_WIDTH, ARENA_HEIGHT};
use std::time::Duration;

 pub fn food_spawner(mut commands: Commands){
    commands.spawn(SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0., 1., 0.5),
            ..default()
        },
        ..default()
    })
    .insert(Food)
    .insert(Position{
        x : (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y : (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    })
    .insert(Size::square(0.8));
}
use bevy::prelude::Component;
#[derive(Component)]
pub struct Food;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App)
    {
        app.add_system(food_spawner.run_if(on_timer(Duration::from_secs_f32(1.))));
    }
}
