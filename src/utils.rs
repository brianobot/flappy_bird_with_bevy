use crate::constants;
use crate::{ThreadRng, Rng};


pub fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.random_range(-constants::OBSTACLE_VERTICAL_OFFSET..constants::OBSTACLE_VERTICAL_OFFSET) * constants::PIXEL_RATIO
}

pub fn get_centered_pipe_position() -> f32 {
    (constants::OBSTACLE_HEIGHT / 2. + constants::OBSTACLE_GAP_SIZE) * constants::PIXEL_RATIO
}