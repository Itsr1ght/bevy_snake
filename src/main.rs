use bevy::{prelude::*, app::AppExit, window::PrimaryWindow}; 

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
#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR : Color = Color::rgb(1.0, 0.0, 0.0);

fn spawn_snake(mut commands : Commands)
{
    commands.spawn(
            SpriteBundle{
                sprite:Sprite{
                    color: SNAKE_HEAD_COLOR,
                    ..default()
                },
                transform: Transform{
                    scale: Vec3::new(10.0, 10.0, 10.0),
                    ..default()
                },
                ..default()
            }
        ).insert(SnakeHead)
        .insert(Position{x: 3, y: 3})
        .insert(Size::square(0.8))
        ;
}

fn snake_movement(keyboard_input : Res<Input<KeyCode>>,
                  mut exit : EventWriter<AppExit>,
                  mut head_pos : Query<(&SnakeHead, &mut Transform)>)
{
   for (_head, mut tranform) in head_pos.iter_mut(){
        if keyboard_input.pressed(KeyCode::W){
            tranform.translation.y += 1f32;
        }
        else if keyboard_input.pressed(KeyCode::S) {
            tranform.translation.y -= 1f32;
        }
        else if keyboard_input.pressed(KeyCode::D) {
            tranform.translation.x += 1f32;
        }
        else if keyboard_input.pressed(KeyCode::A) {
            tranform.translation.x -= 1f32;
        }
        if keyboard_input.pressed(KeyCode::Escape)
        {
            exit.send(AppExit)
        }
   }
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

// SetUp The Game
fn setup_camera(mut commands : Commands){
    commands.spawn(Camera2dBundle::default());
} 

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .add_system(snake_movement)
        .add_system(size_scaling)
        .run();
}
