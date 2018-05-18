use specs::{Component, NullStorage, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Controlled;

impl Component for Controlled {
    type Storage = NullStorage<Self>;
}

// #[derive(Debug)]
// pub struct Collision {
//     pub x0: f32,
//     pub y0: f32,
//     pub x1: f32,
//     pub x2: f32,
// }

// impl Component for Collision {
//     type Storage = VecStorage<Self>;
// }

// use ggez::graphics;

#[derive(Debug)]
pub struct Sprite {
    pub image_name: String,
    pub sprite_index: i32,
    pub frames_since_last_change: i32,
    pub animation_rate: i32,
}

impl Sprite {
    pub fn new(name: String) -> Self {
        Self {
            image_name: name,
            sprite_index: 0,
            frames_since_last_change: 0,
            animation_rate: 0,
        }
    }
}

impl Component for Sprite {
    type Storage = VecStorage<Self>;
}
