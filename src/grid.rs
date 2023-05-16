use crate::utils::*;
use bevy::window::{Window, PrimaryWindow};
use bevy::prelude::{Query, Vec3, With, Transform, Plugin, App};


pub const ARENA_WIDTH: u32 = 10;
pub const ARENA_HEIGHT: u32 = 10;

fn size_scaling(
        window_query: Query<&Window, With<PrimaryWindow>>,
        mut q: Query<(&Size, &mut Transform )>
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

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App){
        app.add_system(position_translation)
        .add_system(size_scaling);
    }
}
