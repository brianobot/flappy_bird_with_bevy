use bevy::prelude::*;

use rand::{Rng, rng, rngs::ThreadRng};

mod components;
#[allow(unused)]
use components::*;

mod constants;
#[allow(unused)]
use constants::*;

mod systems;
#[allow(unused)]
use systems::*;

mod plugins;
use plugins::FlappyBirdPlugin;

mod utils;
#[allow(unused)]
use utils::*;

// Entities (Collection of components) Player Entity: [Health, Damage, Male]
// Components [Individual attributes]
// Systems [Functions that perform actions on entities and components]

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird 🐦"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(FlappyBirdPlugin)
        .run();
}
