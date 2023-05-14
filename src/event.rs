use bevy::prelude::*;
use crate::snake::{SnakeTails, SnakeTail,spawn_snake};
use crate::food::Food;

pub struct GrowthEvent;

pub struct GameOverEvent;

pub fn game_over(
    mut commands : Commands,
    mut reader: EventReader<GameOverEvent>,
    segements: ResMut<SnakeTails>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<SnakeTail>>
    ) {
    if reader.iter().next().is_some(){
        for ent in food.iter().chain(segments.iter()) {
            commands.entity(ent).despawn();
        }
        spawn_snake(commands, segements);
    }
}
