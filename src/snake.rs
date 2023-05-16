use crate::utils::{Size, Position};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use std::time::Duration;
use crate::food::Food;
use crate::grid::{ARENA_HEIGHT, ARENA_WIDTH};
use crate::event::{GrowthEvent, GameOverEvent};

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left
        }
    }
}
#[derive(Component)]
pub struct SnakeHead{
    direction : Direction
}


const SNAKE_HEAD_COLOR : Color = Color::rgb(1.0, 0.0, 0.0);

pub fn spawn_snake(
    mut commands : Commands,
    mut segments: ResMut<SnakeTails>)
{
    *segments = SnakeTails(vec![
        commands.spawn(SpriteBundle {
                sprite: Sprite {
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                ..default()
            })
            .insert(SnakeHead {
                direction: Direction::Up,
            })
            .insert(SnakeTail)
            .insert(Position { x: 3, y: 3 })
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(commands, Position { x: 3, y: 3 }),
    ]);
}

fn snake_movement_input(keyboard_input : Res<Input<KeyCode>>,
                  mut exit : EventWriter<AppExit>,
                  mut head : Query<&mut SnakeHead>)
{
    if let Some(mut head) = head.iter_mut().next(){
        let dir = if keyboard_input.pressed(KeyCode::W) {
            Direction::Up
        }
        else if keyboard_input.pressed(KeyCode::S) {
            Direction::Down
        }
        else if keyboard_input.pressed(KeyCode::D) {
            Direction::Left
        }
        else if keyboard_input.pressed(KeyCode::A) {
            Direction::Right
        }
        else if keyboard_input.pressed(KeyCode::Escape) {
            exit.send(AppExit);
            head.direction 
        }
        else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}

pub fn snake_movement(
    segment : ResMut<SnakeTails>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut heads : Query<(Entity , &SnakeHead)>,
    mut game_over_writer: EventWriter<GameOverEvent>,
    mut positions: Query<&mut Position>
    ){
   if let Some((head_entity, head)) = heads.iter_mut().next(){
       let segment_positions = segment.iter()
           .map(|e| *positions.get_mut(*e).unwrap())
           .collect::<Vec<Position>>();
       *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
       let mut head_pos = positions.get_mut(head_entity).unwrap();
       if head_pos.x < 0 || head_pos.y < 0 || head_pos.x as u32 >= ARENA_WIDTH || head_pos.y as u32 >= ARENA_HEIGHT{
            game_over_writer.send(GameOverEvent);
       }
        match &head.direction {
            Direction::Up => head_pos.y += 1, 
            Direction::Down => head_pos.y -= 1,
            Direction::Left => head_pos.x += 1,
            Direction::Right => head_pos.x -= 1,
        }
        segment_positions
            .iter()
            .zip(segment.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos; 
            });
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>){
    for head_pos in head_positions.iter(){
        for (ent, food_pos) in food_positions.iter(){
            if head_pos == food_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(
    commands : Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segements: ResMut<SnakeTails>,
    mut growth_reader: EventReader<GrowthEvent>
    ){
    if growth_reader.iter().next().is_some(){
        segements.push(spawn_segment(commands, last_tail_position.0.unwrap()))
    }
}

//Snake Tail
#[derive(Component)]
pub struct SnakeTail;

#[derive(Default, Deref, DerefMut, Resource)]
pub struct SnakeTails(Vec<Entity>);

#[derive(Default, Resource)]
pub struct LastTailPosition(Option<Position>);

pub fn spawn_segment(
    mut commands : Commands,
    position : Position
    ) -> Entity
{
   commands.spawn(
            SpriteBundle{
                sprite: Sprite { color: Color::rgb(0., 0., 0.5),
                ..default()
                },
            ..default()
            }
       )
       .insert(SnakeTail)
       .insert(position)
       .insert(Size::square(0.5))
       .id()
}


pub struct SnakePlugin;

impl Plugin for SnakePlugin {
    fn build(&self, app: &mut App)
    {
        app.insert_resource(SnakeTails::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(spawn_snake)
        .add_systems(
            (
                snake_movement_input,
                snake_movement.run_if(on_timer(Duration::from_secs_f32(0.150)))
            ).chain())
        .add_system(snake_eating.after(snake_movement))
        .add_system(snake_growth.after(snake_eating))
        .add_startup_system(spawn_snake);
    }
}
