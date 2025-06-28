use bevy::{prelude::*, window::PrimaryWindow};
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
        // .add_systems(Update, handle_image_asset_creation)
        .add_systems(Startup, setup_level)
        .add_systems(Startup, create_new_window)
        // .add_systems(Update, debug_asset)
        // .add_systems(Update, update_bird)
        // .add_systems(Update, update_obstacles)
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
struct Special {}

#[derive(Component)]
struct Bird {
    pub velocity: f32,
}

#[derive(Component)]
struct Obstacle {
    pipe_direction: f32,
}


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
        window_dimensions: Vec2::new(window.width(), window.height()),
    });

    // changes the background color
    commands.insert_resource(ClearColor(Color::srgb(0.5, 0.7, 0.8)));
    commands.spawn(Camera2d::default());
    // for the sprite below to be visible on the screen the camera must be spawn above

    commands.spawn((
        Sprite::from_image(bird_image),
        Transform::IDENTITY.with_scale(Vec3::splat(PIXEL_RATIO)),
        Bird { velocity: 0. },
    ));

    let mut rand = rng();
    spawn_obstacles(&mut commands, &mut rand, window.width(), &pipe_image);
}

fn create_new_window(mut commands: Commands) {
    commands.spawn((
        Window { ..Default::default() },
        Special {}
    ));
}


fn debug_asset(images: Res<Assets<Image>>) {
    for image in images.iter() {
        info!("Image: {:?}", image);
    }
}


fn handle_image_asset_creation(
    mut image_asset_event: EventReader<AssetEvent<Image>>,
) {
    info!("📦 Image Asset Events {:?}", image_asset_event);
}


fn update_bird(
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if let Ok((mut bird, mut transform)) = bird_query.single_mut() {
        if keys.just_pressed(KeyCode::Space) {
            bird.velocity = FLAP_FORCE;
        }

        bird.velocity -= time.delta_secs() * GRAVITY;
        transform.translation.y += bird.velocity * time.delta_secs();

        transform.rotation = Quat::from_axis_angle(
            Vec3::Z,
            f32::clamp(bird.velocity as f32 / VELOCITY_TO_ROTATION, -90., 90.).to_radians(),
        );
    }
}


fn spawn_obstacles(
    mut commands: &mut Commands,
    mut rand: &mut ThreadRng,
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


fn generate_offset(mut rand: &mut ThreadRng) -> f32 {
    rand.random_range(-OBSTACLE_VERTICAL_OFFSET..OBSTACLE_VERTICAL_OFFSET) * PIXEL_RATIO
}


fn get_centered_pipe_position() -> f32 {
    (OBSTACLE_HEIGHT / 2. + OBSTACLE_GAP_SIZE) * PIXEL_RATIO
}


fn spawn_obstacle(
    translation: Vec3,
    pipe_direction: f32,
    mut commands: &mut Commands,
    pipe_image: &Handle<Image>,
) {
    commands.spawn((
        Sprite::from_image(pipe_image.clone()),
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

        // if transform.translation.x + OBSTACLE_WIDTH * PIXEL_RATIO / 2. < game_manager.window_dimensions.x /2. {
        //     transform.translation.x += OBSTACLE_AMOUNT as f32 * OBSTACLE_SPACING * PIXEL_RATIO;
        //     transform.translation.y = get_centered_pipe_position() * obstacle.pipe_direction + y_offset;
        // }
    }
}
