use avian2d::prelude::*;
use bevy::{
    app::ScheduleRunnerPlugin, diagnostic::FrameCount, prelude::*, render::RenderPlugin,
    winit::WinitPlugin,
};
use bevy_capture::{CameraTargetHeadless, CaptureBundle};
use chrono::{DateTime, Utc};
use rand::prelude::*;
use std::fs;

mod components;
mod config;
mod resources;
mod systems;

use components::*;
use config::*;
use resources::*;
use systems::*;

/// Main
fn main() {
    let mut app = App::new();

    // Simulation
    app.add_plugins((PhysicsPlugins::default(),))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                idle_energy,
                plant_regeneration_system,
                assign_targets,
                predator_movement,
                prey_movement,
                movement_energy,
                collision_kill_system,
                reproduction,
                death,
                update_text,
            )
                .chain(),
        )
        // Avian's physics
        .insert_resource(Gravity(Vec2::ZERO))
        // Miscellaneous
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)));

    // Preview vs generation
    if PREVIEW_MODE {
        println!("Preview mode: window + no capture.");
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "TeemLabs".into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Update, preview_frame_counter)
        .insert_resource(Time::<Fixed>::from_seconds(FIXED_TIME_STEP as f64));
    } else {
        println!("Generation mode: headless + capture.");
        app.add_plugins((
            DefaultPlugins
                .build()
                .disable::<WinitPlugin>()
                .set(RenderPlugin {
                    synchronous_pipeline_compilation: true,
                    ..default()
                }),
            ScheduleRunnerPlugin {
                run_mode: bevy::app::RunMode::Loop { wait: None },
            },
            bevy_capture::CapturePlugin,
        ))
        .add_systems(Update, (manual_physics_step, capture_frame));
    }

    // Debug
    if DEBUG {
        app.init_resource::<FrameCount>()
            .add_systems(Update, update_debugger);
    }

    // Run
    app.run();
}

/// Setup
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Create outputs directories
    if PREVIEW_MODE {
        // Camera
        commands.spawn(Camera2d::default());
    } else {
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

        // Headless camera
        commands.spawn((
            Camera2d,
            Camera::default().target_headless(
                WINDOW_WIDTH as u32,
                WINDOW_HEIGHT as u32,
                &mut images,
            ),
            CaptureBundle::default(),
        ));
    }

    // Walls
    generate_world(&mut commands);

    // Entities
    spawn_entities(&mut commands);

    // Texts
    spawn_hud(&mut commands);

    // Debugger
    if DEBUG {
        spawn_debugger(&mut commands);
    }
}

/// Simulation
fn generate_world(commands: &mut Commands) {
    let wall_restitution = 0.7;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + WALLS_THICKNESS / 2.0 + (WINDOW_HEIGHT - WINDOW_WIDTH);

    let walls_color = Color::linear_rgb(0.925, 0.49, 0.063);

    // Water
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0196 / 7.0, 0.267 / 7.0, 0.369 / 7.0),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0.0, (WINDOW_HEIGHT - WINDOW_WIDTH) / 2.0, 0.0),
    ));

    // Top wall
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, half_h - WALLS_THICKNESS / 2.0, 0.0),
        Restitution::new(wall_restitution),
    ));

    // Middle wall
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, middle_wall_h, 0.0),
        Restitution::new(wall_restitution),
    ));

    // Bottom wall
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, -half_h + WALLS_THICKNESS / 2.0, 0.0),
        Restitution::new(wall_restitution),
    ));

    // Left wall
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WALLS_THICKNESS, WINDOW_HEIGHT)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALLS_THICKNESS, WINDOW_HEIGHT),
        Transform::from_xyz(-half_w + WALLS_THICKNESS / 2.0, 0.0, 0.0),
        Restitution::new(wall_restitution),
    ));

    // Right wall
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WALLS_THICKNESS, WINDOW_HEIGHT)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WALLS_THICKNESS, WINDOW_HEIGHT),
        Transform::from_xyz(half_w - WALLS_THICKNESS / 2.0, 0.0, 0.0),
        Restitution::new(wall_restitution),
    ));
}

fn spawn_entities(commands: &mut Commands) {
    let mut rng = rand::rng();

    let walls_paddings = WALLS_THICKNESS * 2.0 + 8.0;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + WALLS_THICKNESS / 2.0 + (WINDOW_HEIGHT - WINDOW_WIDTH);

    let entity_bundle = (
        RigidBody::Dynamic,
        Restitution::new(0.2), // Bouncing restitution
        Friction::new(0.2),
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        Consumable,
    );

    // Predators
    let predators_color = EntityColor::new(1.0, 0.05, 0.05);
    for _i in 0..NB_PREDATORS {
        let rand_speed_factor = rng.random_range(0.7..1.0);
        commands.spawn((
            entity_bundle.clone(),
            Name::new("Predator"),
            predators_color.clone(),
            Species::Predator,
            Energy::new(INITIAL_PREDATOR_ENERGY, MAX_PREDATOR_ENERGY),
            Hunter::new(Species::Prey, 400.0),
            Speed::new(MAX_SPEED * rand_speed_factor),
            Size::new(PREDATOR_SIZE),
            ActiveMover,
            Transform::from_xyz(
                -half_w + walls_paddings + PREDATOR_SIZE,
                half_h - walls_paddings - PREDATOR_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Collider::rectangle(PREDATOR_SIZE, PREDATOR_SIZE),
            Sprite {
                color: predators_color.value(),
                custom_size: Some(Vec2::splat(PREDATOR_SIZE)),
                ..default()
            },
        ));
    }

    // Prey
    let prey_color = EntityColor::new(0.05, 0.05, 1.0);
    for _i in 0..NB_PREY {
        let rand_speed_factor = rng.random_range(0.3..1.0);
        commands.spawn((
            entity_bundle.clone(),
            prey_color.clone(),
            Collider::rectangle(PREY_SIZE, PREY_SIZE),
            Sprite {
                color: prey_color.value(),
                custom_size: Some(Vec2::splat(PREY_SIZE)),
                ..default()
            },
            Transform::from_xyz(
                half_w - walls_paddings - PREY_SIZE,
                half_h - walls_paddings - PREY_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Species::Prey,
            Prey::new(200.0),
            Hunter::new(Species::Plant, 300.0),
            Speed::new(MAX_SPEED * rand_speed_factor),
            Energy::new(INITIAL_PREY_ENERGY, MAX_PREY_ENERGY),
            Size::new(PREY_SIZE),
            Name::new("Prey"),
            ActiveMover,
        ));
    }

    // Plants
    let plants_color = EntityColor::new(0.05, 1.0, 0.05);
    for _i in 0..NB_PLANTS {
        commands.spawn((
            entity_bundle.clone(),
            plants_color.clone(),
            Collider::rectangle(PLANT_SIZE, PLANT_SIZE),
            Sprite {
                color: plants_color.value(),
                custom_size: Some(Vec2::splat(PLANT_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, middle_wall_h + walls_paddings + PLANT_SIZE, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Species::Plant,
            Speed::new(0.0),
            Energy::new(INITIAL_PLANT_ENERGY, MAX_PLANT_ENERGY),
            Size::new(PLANT_SIZE),
            Name::new("Plant"),
            Photosynthesis,
        ));
    }
}

/// HUD
fn spawn_hud(commands: &mut Commands) {
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + WALLS_THICKNESS / 2.0 + (WINDOW_HEIGHT - WINDOW_WIDTH);
    let text_z = 1.0; // Ensure text is above other sprites

    // Predators
    let predators_y = middle_wall_h - 150.0; // Below middle wall
    commands.spawn((
        Text2d::new(format!("Predators: {}", NB_PREDATORS)),
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, predators_y, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
        Species::Predator,
    ));

    commands.spawn((
        Text2d::new("Predators have no fear. They hunt prey\nuntil there's nothing left to eat."),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, predators_y - TITLE_FONT_SIZE * 2.0, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
    ));

    // Predator sprite
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(1.0, 0.05, 0.05),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(0.0, predators_y + TITLE_FONT_SIZE * 2.0, 0.0),
        AngularVelocity(0.2),
    ));

    // Prey
    let prey_y = predators_y - 250.0;
    commands.spawn((
        Text2d::new(format!("Prey: {}", NB_PREY)),
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, prey_y, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
        Species::Prey,
    ));

    commands.spawn((
        Text2d::new(
            "Prey are constantly fleeing from predators.\nWhen they get a break, they eat plants.",
        ),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, prey_y - TITLE_FONT_SIZE * 2.0, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
    ));

    // Prey sprite
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.05, 0.05, 1.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(0.0, prey_y + TITLE_FONT_SIZE * 2.0, 0.0),
        AngularVelocity(0.1),
    ));

    // Plants
    let plants_y = prey_y - 250.0;
    commands.spawn((
        Text2d::new(format!("Plants: {}", NB_PLANTS)),
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, plants_y, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
        Species::Plant,
    ));

    commands.spawn((
        Text2d::new("Plants have no abilities. They are eaten\nby prey (so aren't they also..?)."),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(0.0, plants_y - TITLE_FONT_SIZE * 2.0, text_z),
        TextLayout::new_with_justify(JustifyText::Center),
    ));

    // Plants sprite
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.05, 1.0, 0.05),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(0.0, plants_y + TITLE_FONT_SIZE * 2.0, 0.0),
        AngularVelocity(0.05),
    ));
}

/// DEBUG
fn spawn_debugger(commands: &mut Commands) {
    commands.spawn((
        Text2d::new("FRAME N°0"),
        TextFont {
            font_size: DEBUG_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(
            0.0,
            -WINDOW_HEIGHT / 2.0 + DEBUG_FONT_SIZE + WALLS_THICKNESS + DEBUG_POS_PADDING,
            1.0,
        ),
        TextLayout::new_with_justify(JustifyText::Center),
        DEBUGGER,
    ));
}
