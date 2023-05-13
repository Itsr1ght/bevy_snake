use bevy::{prelude::*, app::AppExit, window::PrimaryWindow, time::common_conditions::on_timer};
use rand::random; 

// Position and other component
#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
struct Size {
    width: f32,
    height: f32
}

impl Size {
    pub fn square(x: f32) -> Self{
        Self { width: x, height: x }
    }
}
// Setup the Snake
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
struct SnakeHead{
    direction : Direction
}


const SNAKE_HEAD_COLOR : Color = Color::rgb(1.0, 0.0, 0.0);

fn spawn_snake(
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
        if keyboard_input.pressed(KeyCode::W) {
            head.direction = Direction::Up
        }
        else if keyboard_input.pressed(KeyCode::S) {
            head.direction = Direction::Down
        }
        else if keyboard_input.pressed(KeyCode::D) {
            head.direction = Direction::Left
        }
        else if keyboard_input.pressed(KeyCode::A) {
            head.direction = Direction::Right
        }
        else if keyboard_input.pressed(KeyCode::Escape) {
            exit.send(AppExit)
        }
    }
}

fn snake_movement(
    segment : ResMut<SnakeTails>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut heads : Query<(Entity , &SnakeHead)>,
    mut positions: Query<&mut Position>
    ){
   if let Some((head_entity, head)) = heads.iter_mut().next(){
       let segment_positions = segment.iter()
           .map(|e| *positions.get_mut(*e).unwrap())
           .collect::<Vec<Position>>();
       *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));
       let mut head_pos = positions.get_mut(head_entity).unwrap();
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

//Snake Tail
#[derive(Component)]
struct SnakeTail;

#[derive(Default, Deref, DerefMut, Resource)]
struct SnakeTails(Vec<Entity>);

#[derive(Default, Resource)]
struct LastTailPosition(Option<Position>);

fn spawn_segment(
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

// Setup The Grid

const ARENA_WIDTH: u32 = 10;
const ARENA_HEIGHT: u32 = 10;

fn size_scaling(
        window_query: Query<&Window, With<PrimaryWindow>>,
        mut q: Query<(&Size, &mut Transform, )>
    )
    {
        let window = window_query.get_single().unwrap();
        for (sprite_size, mut transform) in q.iter_mut(){
            transform.scale = Vec3::new( 
                sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
                sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
                1.0);
        }
    }

fn position_translation(
        window_query: Query<&Window, With<PrimaryWindow>>,
        mut q: Query<(&Position, &mut Transform)>
    ){
    let window = window_query.get_single().unwrap();
    fn convert(
            pos: f32,
            bound_window: f32,
            bound_game: f32
        ) -> f32 {
            let tile_size = bound_game / bound_window;
            pos / bound_game * bound_window - (bound_window / 2f32) + (tile_size / 2f32)
    }   
    for (pos, mut transform) in q.iter_mut(){
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0);
    }
}

//Food
#[derive(Component)]
struct Food;

struct GrowthEvent;

fn food_spawner(mut commands: Commands){
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


// SetUp The Game
fn setup_window_settings(mut windows: Query<&mut Window>){
    for mut window in &mut windows {
        window.title = "Snake".to_string();
    }
}

fn setup_camera(mut commands : Commands){
    commands.spawn(Camera2dBundle::default());
} 

fn main() {
    use std::time::Duration;
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(SnakeTails::default())
        .insert_resource(LastTailPosition::default())
        .add_event::<GrowthEvent>()
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
        .add_system(position_translation)
        .add_system(food_spawner.run_if(on_timer(Duration::from_secs_f32(1.))))
        .add_system(size_scaling)
        .run();
}
