use bevy::ui::{BorderColor, UiRect};
use bevy::{prelude::*, transform, window::PrimaryWindow};

use rand::{Rng, rng, rngs::ThreadRng};
// use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

const GRAVITY: f32 = 200.;
const FLAP_FORCE: f32 = 100.;
const PIXEL_RATIO: f32 = 4.0;
const VELOCITY_TO_ROTATION: f32 = 7.5;

const OBSTACLE_AMOUNT: i32 = 5;
const OBSTACLE_WIDTH: f32 = 32.;
const OBSTACLE_HEIGHT: f32 = 144.;
const OBSTACLE_SPACING: f32 = 60.;
const OBSTACLE_GAP_SIZE: f32 = 15.;
const OBSTACLE_SCROLL_SPEED: f32 = 150.;
const OBSTACLE_VERTICAL_OFFSET: f32 = 30.;

// Entities (Collection of components) Player Entity: [Health, Damage, Male]
// Components [Individual attributes]
// Systems [Functions that perform actions on entities and components]

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Flappy Bird By Brian"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512., 512.).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        // .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        // .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, setup_level)
        .add_systems(Update, update_bird)
        .add_systems(Update, update_obstacles)
        .add_systems(Update, debug_cursor_position)
        .run();
}

#[derive(Resource)]
struct GameManager {
    // since the application would create multiple assets from the pipe image handle
    // it makes sense to create and store this handle in a general place like a Resource instance
    pub pipe_image: Handle<Image>,
    pub window_dimensions: Vec2,
}

#[derive(Component)]
struct Bird {
    pub velocity: f32,
}

#[derive(Component)]
struct Obstacle {
    pipe_direction: f32,
}

#[derive(Component)]
struct CoordinateText;

fn setup_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // the Handle(s) are simple ids that references assets loaded into the game
    let pipe_image = asset_server.load("pipe.png");
    let bird_image = asset_server.load("bird.png");

    // the assumption is that there is only One PrimaryWindow
    // this Window component has some attributes about the window in Question
    // these attributes are important for the structure of the game as we would see later on
    let window = window_query.single_mut().unwrap();

    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    // adding a resource more than once simply replaces the former instance with the new instance
    commands.insert_resource(GameManager {
        pipe_image: pipe_image.clone(),
        // this is helpful to store so we don't keep query for the window in other systems
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    // changes the background color
    // it was not intuitive why the change color process would be through a Resource
    // But since there is usually one window this makes sense sort of
    // no it actually does not make sense, this should have been set as some attribute of the
    // of the window component in my opinion
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));

    commands.spawn(Camera2d::default());
    // for the sprite below to be visible on the screen the camera must be spawn above
    // Cameras in Bevy are mandatory to see anything: they configure the rendering.

    commands.spawn((
        // Sprite::from_image(bird_image.clone()),
        // A Transform is what allows you to place an object in the game world. It is a combination of the object's
        // "translation" (position/coordinates), "rotation", and "scale" (size adjustment).
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)), // makes the image PIXEL_RATIO times larger in all dimensions
        // Transform has 3 fields
        // - translation: moves objects around
        // - rotation: rotate objects
        // - scale: make them larger or smaller
        Bird { velocity: 0. },
    ));

    commands.spawn((
        Sprite::from_image(bird_image.clone()),
        Transform {
            translation: Vec3::splat(0.),
            scale: Vec3::splat(4.),
            ..Default::default()
        },
        Bird { velocity: 0. },
    ));

    let mut rand = rng();
    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image);
}

fn update_bird(
    time: Res<Time>,
    mut commands: Commands,
    mut bird_query: Query<(&mut Bird, &mut Transform), Without<Obstacle>>,
    mut obstacle_query: Query<(Entity, &mut Transform), With<Obstacle>>,
    keys: Res<ButtonInput<KeyCode>>,
    game_manager: Res<GameManager>,
) {
    for (mut bird, mut transform) in bird_query.iter_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }

        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity as f32 / VELOCITY_TO_ROTATION, -90., 90.).to_radians(),
        );

        let mut dead = false;
        if transform.translation.y <= -game_manager.window_dimensions.y / 2. {
            dead = true
        } else {
            for (_entity, obstacle_transform) in obstacle_query.iter() {
                if (obstacle_transform.translation.y - transform.translation.y).abs()
                    < OBSTACLE_HEIGHT * PIXEL_RATIO / 2.
                    && (obstacle_transform.translation.x - transform.translation.x).abs()
                        < OBSTACLE_WIDTH * PIXEL_RATIO / 2.
                {
                    dead = true
                }
            }
        }

        if dead {
            transform.translation = Vec3::ZERO;
            bird.velocity = 0.;

            for (entity, _obstacle_transform) in obstacle_query.iter_mut() {
                commands.entity(entity).despawn();
            }

            let mut rand = rng();
            spawn_obstacles(
                &mut commands,
                &mut rand,
                game_manager.window_dimensions.x,
                &game_manager.pipe_image.clone(),
            );
        }
    }
}

fn spawn_obstacles(
    commands: &mut Commands,
    rand: &mut ThreadRng,
    window_width: f32,
    pipe_image: &Handle<Image>,
) {
    for i in 0..OBSTACLE_AMOUNT {
        let y_offset = generate_offset(rand);
        let x_pos = window_width / 2.0 * OBSTACLE_SPACING * PIXEL_RATIO * i as f32;

        info!("index: {} [Y Offset: {}, X Pos: {}]", i, y_offset, x_pos);

        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (get_centered_pipe_position() + y_offset),
            1.,
            commands,
            pipe_image,
        );
        spawn_obstacle(
            Vec3::X * x_pos + Vec3::Y * (-get_centered_pipe_position() + y_offset),
            -1.,
            commands,
            pipe_image,
        );
    }
}

fn generate_offset(rand: &mut ThreadRng) -> f32 {
    rand.random_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}

fn get_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}

fn spawn_obstacle(
    translation: Vec3,
    pipe_direction: f32,
    commands: &mut Commands,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite::from_image(pipe_image.clone()),
        // This chains some Transform methods to create a Transform instance with it translation and scale set
        Transform::from_translation(translation).with_scale(Vec3::new(
            PIXEL_RATIO,
            PIXEL_RATIO * -pipe_direction,
            PIXEL_RATIO,
        )),
        Obstacle { pipe_direction },
    ));
}

fn update_obstacles(
    time: Res<Time>,
    game_manager: Res<GameManager>,
    mut obstacle_query: Query<(&mut Obstacle, &mut Transform)>,
) {
    let mut rand = rng();
    let y_offset = generate_offset(&mut rand);
    for (obstacle, mut transform) in obstacle_query.iter_mut() {
        transform.translation.x -= time.delta_secs() * OBSTACLE_SCROLL_SPEED;

        if (transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2.)
            < -game_manager.window_dimensions.x / 2.
        {
            transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
            transform.translation.y =
                get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        }
    }
}

fn debug_cursor_position(
    query_window: Query<&Window>,
    query_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let (camera, camera_transform) = query_camera.single().unwrap();
    let window = query_window.single().unwrap();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor).ok())
        .map(|ray| ray.origin.truncate())
    {
        info!("Screen Position: {}", world_position);
    }
}
