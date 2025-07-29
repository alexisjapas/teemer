use avian2d::prelude::*;
use bevy::prelude::*;
use chrono::{DateTime, Utc};
use rand::random;
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
    .add_systems(
        Update,
        (
            assign_targets,
            predator_movement,
            prey_movement,
            collision_kill_system,
            update_text,
        ),
    )
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
    generate_world(&mut commands);

    // Entities
    spawn_entities(&mut commands);

    // Texts
    spawn_hud(&mut commands);
}

fn generate_world(commands: &mut Commands) {
    let wall_restitution = 0.7;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    // Water
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0196, 0.267, 0.369),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0.0, (WINDOW_HEIGHT - WINDOW_WIDTH) / 2.0, 0.0),
    ));

    // Top wall
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.4, 0.1, 0.2),
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
            color: Color::linear_rgb(0.4, 0.1, 0.2),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, -half_h + WALLS_THICKNESS / 2.0 + (1280.0 - 720.0), 0.0),
        Restitution::new(wall_restitution),
    ));

    // Bottom wall
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.4, 0.1, 0.2),
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
            color: Color::linear_rgb(0.4, 0.1, 0.2),
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
            color: Color::linear_rgb(0.4, 0.1, 0.2),
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
    let walls_paddings = WALLS_THICKNESS * 2.0 + 8.0;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;

    let entity_bundle = (
        RigidBody::Dynamic,
        Restitution::new(0.2), // Bouncing restitution
        Friction::new(0.2),
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        Consumable,
    );

    // Predators
    for _i in 0..NB_PREDATORS {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Collider::rectangle(PREDATOR_SIZE, PREDATOR_SIZE),
            Sprite {
                color: Color::linear_rgb(1.0, rand_color, rand_color),
                custom_size: Some(Vec2::splat(PREDATOR_SIZE)),
                ..default()
            },
            Transform::from_xyz(-half_w + walls_paddings, half_h - walls_paddings, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Predator,
            Hunter::new(Species::Prey, 444.4),
            Speed::new(MAX_SPEED * random::<f32>().max(0.1)),
            Name::new("Predator"),
        ));
    }

    // Prey
    for _i in 0..NB_PREY {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Collider::rectangle(PREY_SIZE, PREY_SIZE),
            Sprite {
                color: Color::linear_rgb(rand_color, rand_color, 1.0),
                custom_size: Some(Vec2::splat(PREY_SIZE)),
                ..default()
            },
            Transform::from_xyz(half_w - walls_paddings, half_h - walls_paddings, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Prey,
            Prey::new(222.2),
            Hunter::new(Species::Plant, 444.4),
            Speed::new(MAX_SPEED * random::<f32>().max(0.2)),
            Name::new("Prey"),
        ));
    }

    // Plants
    for _i in 0..NB_PLANTS {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Collider::rectangle(PLANT_SIZE, PLANT_SIZE),
            Sprite {
                color: Color::linear_rgb(rand_color, 1.0, rand_color),
                custom_size: Some(Vec2::splat(PLANT_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, -half_h + walls_paddings + (1280.0 - 720.0), 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Plant,
            Speed::new(0.0),
            Name::new("Plant"),
        ));
    }
}

fn spawn_hud(commands: &mut Commands) {
    // Predators
    commands.spawn((
        Text::new(format!("Predators: {}", NB_PREDATORS)),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: TITLE_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(720.0 + 60.0),
            left: Val::Px(108.0),
            ..default()
        },
        Species::Predator,
    ));
    commands.spawn((
        Text::new("Predators have no fear. They hunt prey\nuntil there's nothing left to eat."),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(720.0 + 60.0 + TITLE_FONT_SIZE * 2.0),
            left: Val::Px(108.0 + 10.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(
            -WINDOW_WIDTH / 2.0 + 108.0 / 2.0,
            (WINDOW_HEIGHT / 2.0) - 720.0 - 60.0 - WALLS_THICKNESS - TITLE_FONT_SIZE / 2.0,
            0.0,
        ),
        // Avian's physics
        AngularVelocity(0.4),
    ));

    // Prey
    commands.spawn((
        Text::new(format!("Prey: {}", NB_PREY)),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(944.0),
            left: Val::Px(108.0),
            ..default()
        },
        Species::Prey,
    ));
    commands.spawn((
        Text::new(
            "Prey are constantly fleeing from predators.\nWhen they get a break, they eat plants.",
        ),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(944.0 + TITLE_FONT_SIZE * 2.0),
            left: Val::Px(108.0 + 10.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(
            -WINDOW_WIDTH / 2.0 + 108.0 / 2.0,
            (WINDOW_HEIGHT / 2.0) - 944.0 - WALLS_THICKNESS - TITLE_FONT_SIZE / 2.0,
            0.0,
        ),
        // Avian's physics
        AngularVelocity(0.2),
    ));

    // Plants
    commands.spawn((
        Text::new(format!("Plants: {}", NB_PLANTS)),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(1108.0),
            left: Val::Px(108.0),
            ..default()
        },
        Species::Plant,
    ));
    commands.spawn((
        Text::new("Plants have no abilities. They are eaten\nby prey (so aren't they also...?)."),
        TextLayout::new_with_justify(JustifyText::Left),
        TextFont {
            font_size: TEXT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(1108.0 + TITLE_FONT_SIZE * 2.0),
            left: Val::Px(108.0 + 10.0),
            ..default()
        },
    ));
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(
            -WINDOW_WIDTH / 2.0 + 108.0 / 2.0,
            (WINDOW_HEIGHT / 2.0) - 1108.0 - WALLS_THICKNESS - TITLE_FONT_SIZE / 2.0,
            0.0,
        ),
        // Avian's physics
        AngularVelocity(0.1),
    ));
}
