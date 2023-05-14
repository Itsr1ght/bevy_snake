use bevy::prelude::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct Size {
    pub width: f32,
    pub height: f32
}

impl Size {
    pub fn square(x: f32) -> Self{
        Self { width: x, height: x }
    }
}

