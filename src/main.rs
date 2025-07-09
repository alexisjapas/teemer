use avian2d::prelude::*;
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Capturing, Screenshot, save_to_disk},
};
use chrono::{DateTime, Utc};
use rand::random;
use std::fs;

/// Video parameters
const PREVIEW_MODE: bool = cfg!(feature = "preview");
const WINDOW_WIDTH: f32 = 720.0;
const WINDOW_HEIGHT: f32 = 1280.0;
const FRAMERATE: f32 = 30.0;
const MAX_DURATION: f32 = 61.0;
const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// Simulation parameters
const NB_ENTITIES_PER_TEAM: i32 = 64;
const SQUARE_LEN: f32 = 32.0;
const MAX_SPEED: f32 = 64.0;

/// Resources
#[derive(Resource)]
struct FramesDir(String);

/// Main
fn main() {
    let mut app = App::new();

    // Minimal configuration
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "TeemLabs".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        PhysicsPlugins::default(),
    ))
    .add_systems(Startup, setup)
    .add_systems(Update, collision_kill_system)
    // Avian's physics
    .insert_resource(Gravity(Vec2::ZERO))
    // Miscellaneous
    .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

    // Specific configuration
    if PREVIEW_MODE {
        println!("Preview mode activated. Real-time rendering & no frames capture.");
        app.add_systems(Update, preview_frame_counter)
            .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIME_STEP as f64));
    } else {
        println!("Generation mode activated. Longer rendering time & frames capture.");
        app.add_systems(
            Update,
            (manual_physics_step, take_frame_screenshot)
                .run_if(no_capture_in_progress)
                .chain(),
        );
    }

    // Run
    app.run();
}

/// Components
#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
enum Species {
    Chicken,
    Fox,
    Snake,
}

#[derive(Component, Debug)]
struct Hunter {
    hunts: Species,
}

#[derive(Component, Clone)]
struct Prey;

/// Systems
// Spawn the 2-D camera and the ball entity
fn setup(mut commands: Commands) {
    // Create outputs directories
    if !PREVIEW_MODE {
        // Get and format date
        let now: DateTime<Utc> = Utc::now();
        let str_date = now.format("sim_%Y-%m-%d-%H-%M-%SZ").to_string();

        // Generate directories
        let sim_dir = format!("./outputs/{}", str_date);
        fs::create_dir_all(&sim_dir).expect("Failed to create simulation directory.");
        let frames_dir = format!("{}/frames", sim_dir);
        fs::create_dir_all(&frames_dir).expect("Failed to create frames directory.");

        // Insert as a resource
        commands.insert_resource(FramesDir(frames_dir));
    }

    // Camera
    commands.spawn(Camera2d::default());

    // Walls
    create_walls(&mut commands);

    // Entities
    spawn_entities(&mut commands);
}

fn create_walls(commands: &mut Commands) {
    let wall_thickness = 10.0;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    // Top wall
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.1, 0.2, 0.3),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, wall_thickness)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, wall_thickness),
        Transform::from_xyz(0.0, half_h - wall_thickness / 2.0, 0.0),
        Restitution::new(1.0),
    ));

    // Bottom wall
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.1, 0.2, 0.3),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, wall_thickness)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, wall_thickness),
        Transform::from_xyz(0.0, -half_h + wall_thickness / 2.0, 0.0),
        Restitution::new(1.0),
    ));

    // Left wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(wall_thickness, WINDOW_HEIGHT),
        Transform::from_xyz(-half_w - wall_thickness / 2.0, 0.0, 0.0),
        Restitution::new(1.0),
    ));

    // Right wall
    commands.spawn((
        RigidBody::Static,
        Collider::rectangle(wall_thickness, WINDOW_HEIGHT),
        Transform::from_xyz(half_w + wall_thickness / 2.0, 0.0, 0.0),
        Restitution::new(1.0),
    ));
}

fn spawn_entities(commands: &mut Commands) {
    let entity_bundle = (
        RigidBody::Dynamic,
        Collider::rectangle(SQUARE_LEN, SQUARE_LEN),
        Restitution::new(1.0), // Bouncing restitution
        Friction::new(0.0),
        LockedAxes::ROTATION_LOCKED,
        Prey,
        CollisionEventsEnabled,
    );

    // Chicken
    for _i in 0..NB_ENTITIES_PER_TEAM {
        let rand_color = random::<f32>().min(0.4).max(0.1);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(rand_color, rand_color, 1.0),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(300.0, 0.0, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Chicken,
            Hunter {
                hunts: Species::Snake,
            },
            Name::new("Chicken"),
        ));
    }

    // Fox
    for _i in 0..NB_ENTITIES_PER_TEAM {
        let rand_color = random::<f32>().min(0.4).max(0.1);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(rand_color, 1.0, rand_color),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(-150.0, 260.0, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Fox,
            Hunter {
                hunts: Species::Chicken,
            },
            Name::new("Fox"),
        ));
    }

    // snake
    for _i in 0..NB_ENTITIES_PER_TEAM {
        let rand_color = random::<f32>().min(0.4).max(0.1);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(1.0, rand_color, rand_color),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(-150.0, -260.0, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Snake,
            Hunter {
                hunts: Species::Fox,
            },
            Name::new("Snake"),
        ));
    }
}

fn take_frame_screenshot(
    mut commands: Commands,
    mut frame_counter: Local<u32>,
    mut exit: EventWriter<AppExit>,
    frames_dir: Res<FramesDir>,
) {
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE {
        println!("Generation done. Exiting.");
        exit.write(AppExit::Success);
        return;
    }

    let path = format!("{}/frame_{:04}.png", frames_dir.0, *frame_counter);
    *frame_counter += 1;
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(path));
}

fn preview_frame_counter(mut frame_counter: Local<u32>, mut exit: EventWriter<AppExit>) {
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE {
        println!("Generation done. Exiting.");
        exit.write(AppExit::Success);
        return;
    }
    *frame_counter += 1;
}

fn no_capture_in_progress(capturing: Query<(), With<Capturing>>) -> bool {
    capturing.is_empty()
}

fn manual_physics_step(mut physics_time: ResMut<Time<Physics>>) {
    println!("Physics step advancing.");
    physics_time.advance_by(std::time::Duration::from_secs_f32(FIXED_TIME_STEP));
}

fn collision_kill_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    query: Query<(Option<&Hunter>, Option<&Species>), With<Prey>>,
) {
    for event in collision_events.read() {
        // Get the components for both entities involved in the collision
        let Ok([entity1_comps, entity2_comps]) = query.get_many([event.0, event.1]) else {
            continue;
        };

        // Check Case 1: Entity1 is the hunter, Entity2 is the prey
        if let (Some(hunter), Some(prey_species)) = (entity1_comps.0, entity2_comps.1) {
            if hunter.hunts == *prey_species {
                commands.entity(event.1).despawn();
                continue;
            }
        }

        // Check Case 2: Entity2 is the hunter, Entity1 is the prey
        if let (Some(hunter), Some(prey_species)) = (entity2_comps.0, entity1_comps.1) {
            if hunter.hunts == *prey_species {
                commands.entity(event.0).despawn();
            }
        }
    }
}
