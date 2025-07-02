use bevy::prelude::{App, Plugin, Startup, Update};

use crate::systems::{setup_level, update_bird, update_obstacles};

pub struct FlappyBirdPlugin;

impl Plugin for FlappyBirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_level);
        app.add_systems(Update, (update_bird, update_obstacles));
    }
}
