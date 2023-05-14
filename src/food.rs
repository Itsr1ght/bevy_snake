use crate::utils::*;
use bevy::prelude::{Commands, SpriteBundle, Sprite, default, Color, Component};
use rand::random;
use crate::grid::*;

#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands){
    commands.spawn(SpriteBundle{
        sprite: Sprite {
            color: Color::rgb(0., 1., 0.),
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

