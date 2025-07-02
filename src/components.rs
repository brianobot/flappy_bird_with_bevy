use bevy::prelude::{Component, Handle, Image, Vec2, Resource};


#[derive(Resource)]
pub struct GameManager {
    // since the application would create multiple assets from the pipe image handle
    // it makes sense to create and store this handle in a general place like a Resource instance
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

#[derive(Component)]
pub struct Bird {
    pub velocity: f32,
}

#[derive(Component)]
pub struct Obstacle {
    pub pipe_direction: f32,
}

