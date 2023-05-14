use bevy::prelude::*;
use crate::snake::{SnakeTails, SnakeTail,spawn_snake, snake_movement};
use crate::food::Food;

pub struct GrowthEvent;

pub struct GameOverEvent;

pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GrowthEvent>().add_event::<GameOverEvent>()
            .add_system(game_over.after(snake_movement));
    }
}

fn game_over(
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
