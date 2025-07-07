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
const WINDOW_WIDTH: f32 = 1080.0;
const WINDOW_HEIGHT: f32 = 1920.0;
const FRAMERATE: f32 = 30.0;
const MAX_FRAMES_TO_CAPTURE: u32 = 61 * FRAMERATE as u32;
const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// Simulation parameters
const NB_ENTITIES: i32 = 4444;
const SQUARE_LEN: f32 = 4.0;
const MAX_SPEED: f32 = 44.0;

/// Simple velocity component
#[derive(Component)]
struct Velocity(Vec2);

/// Resources
#[derive(Resource)]
struct FramesDir(String);

/// Main
fn main() {
    let mut app = App::new();

    // Minimal configuration
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
            title: "Bouncing Ball".into(),
            resizable: false,
            ..default()
        }),
        ..default()
    }))
    .add_systems(Startup, setup);

    if PREVIEW_MODE {
        println!("Preview mode activated. Real-time rendering & no frames capture.");
        app.add_systems(Update, (move_ball, preview_frame_counter));
    } else {
        println!("Generation mode activated. Longer rendering time & frames capture.");
        app.add_systems(
            Update,
            (move_ball, take_frame_screenshot).run_if(no_capture_in_progress),
        );
    }

    app.run();
}

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

    commands.spawn(Camera2d::default());

    for _i in 0..NB_ENTITIES {
        commands.spawn((
            Sprite {
                color: Color::linear_rgb(random(), random(), random()),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            Velocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
        ));
    }
}

// Move the ball and reflect its velocity when it hits a wall
fn move_ball(mut query: Query<(&mut Transform, &mut Velocity)>, time: Res<Time>) {
    let dt = if PREVIEW_MODE {
        time.delta_secs()
    } else {
        FIXED_TIME_STEP
    };

    let half_w = WINDOW_WIDTH / 2.0 - SQUARE_LEN / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0 - SQUARE_LEN / 2.0;

    for (mut position, mut vel) in &mut query {
        position.translation += vel.0.extend(0.0) * dt;

        if position.translation.x > half_w {
            position.translation.x = half_w;
            vel.0.x = -vel.0.x;
        }
        if position.translation.x < -half_w {
            position.translation.x = -half_w;
            vel.0.x = -vel.0.x;
        }
        if position.translation.y > half_h {
            position.translation.y = half_h;
            vel.0.y = -vel.0.y;
        }
        if position.translation.y < -half_h {
            position.translation.y = -half_h;
            vel.0.y = -vel.0.y;
        }
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
