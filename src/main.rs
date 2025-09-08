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
                update_hud,
            )
                .chain(),
        )
        // Avian's physics
        .insert_resource(Gravity(Vec2::ZERO))
        // Miscellaneous
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .insert_resource(HudBatches {
            batches: BATCHES.to_vec(),
            index: 0,
        });

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

    let (water_r, water_g, water_b) = WALLS_COLOR;
    let walls_color = Color::linear_rgb(water_r, water_g, water_b);

    // Water
    let (water_r, water_g, water_b) = WATER_COLOR;
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(water_r, water_g, water_b),
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

    // Middle walls
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
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, middle_wall_h - 64.0, 0.0),
        Restitution::new(wall_restitution),
    ));

    // Bottom walls
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(WINDOW_WIDTH, WALLS_THICKNESS),
        Transform::from_xyz(0.0, -half_h + 54.0, 0.0),
        Restitution::new(wall_restitution),
    ));
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

    // Sprite walls
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WALLS_THICKNESS, 154.0)),
            ..default()
        },
        RigidBody::Static,
        Transform::from_xyz(-77.0, -254.0, 0.0),
        Restitution::new(wall_restitution),
    ));
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WALLS_THICKNESS, 154.0)),
            ..default()
        },
        RigidBody::Static,
        Transform::from_xyz(77.0, -254.0, 0.0),
        Restitution::new(wall_restitution),
    ));
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Transform::from_xyz(0.0, -250.0 - 154.0 / 2.0, 0.0),
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

    // Plants
    // Vyrmosa
    let (vyrmosa_r, vyrmosa_g, vyrmosa_b) = VYRMOSA_COLOR;
    let vyrmosa_color = EntityColor::new(vyrmosa_r, vyrmosa_g, vyrmosa_b);
    for _i in 0..NB_VYRMOSA {
        commands.spawn((
            entity_bundle.clone(),
            vyrmosa_color.clone(),
            Collider::rectangle(VYRMOSA_SIZE, VYRMOSA_SIZE),
            Sprite {
                color: vyrmosa_color.value(),
                custom_size: Some(Vec2::splat(VYRMOSA_SIZE)),
                ..default()
            },
            Transform::from_xyz(0.0, middle_wall_h + half_w, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Species::Vyrmosa,
            Speed::new(VYRMOSA_MAX_SPEED),
            Energy::new(INITIAL_VYRMOSA_ENERGY, MAX_VYRMOSA_ENERGY),
            Size::new(VYRMOSA_SIZE),
            Name::new("Plant"),
            Photosynthesis::new(VYRMOSA_ENERGY_REGEN),
        ));
    }

    // Prey
    // Qirval
    let (qirval_r, qirval_g, qirval_b) = QIRVAL_COLOR;
    let qirval_color = EntityColor::new(qirval_r, qirval_g, qirval_b);
    for _i in 0..NB_QIRVAL {
        let rand_speed_factor = rng.random_range(0.3..1.0);
        commands.spawn((
            entity_bundle.clone(),
            qirval_color.clone(),
            Collider::rectangle(QIRVAL_SIZE, QIRVAL_SIZE),
            Sprite {
                color: qirval_color.value(),
                custom_size: Some(Vec2::splat(QIRVAL_SIZE)),
                ..default()
            },
            Transform::from_xyz(
                -half_w + walls_paddings + QIRVAL_SIZE,
                middle_wall_h + walls_paddings + QIRVAL_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Species::Qirval,
            Prey::new(200.0),
            Hunter::new(vec![Species::Vyrmosa], 200.0),
            Speed::new(QIRVAL_MAX_SPEED * rand_speed_factor),
            Energy::new(INITIAL_QIRVAL_ENERGY, MAX_QIRVAL_ENERGY),
            Size::new(QIRVAL_SIZE),
            Name::new("Prey"),
            ActiveMover,
        ));
    }
    
    // Predators    
    // Lorynth
    let (lorynth_r, lorynth_g, lorynth_b) = LORYNTH_COLOR;
    let lorynth_color = EntityColor::new(lorynth_r, lorynth_g, lorynth_b);
    for _i in 0..NB_LORYNTH {
        let rand_speed_factor = rng.random_range(0.7..1.0);
        commands.spawn((
            entity_bundle.clone(),
            Name::new("Predator"),
            lorynth_color.clone(),
            Species::Lorynth,
            Energy::new(INITIAL_LORYNTH_ENERGY, MAX_LORYNTH_ENERGY),
            Prey::new(350.0),
            Hunter::new(vec![Species::Qirval], 400.0),
            Speed::new(LORYNTH_MAX_SPEED * rand_speed_factor),
            Size::new(LORYNTH_SIZE),
            ActiveMover,
            Transform::from_xyz(
                half_w - walls_paddings - LORYNTH_SIZE,
                half_h - walls_paddings - LORYNTH_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Collider::rectangle(LORYNTH_SIZE, LORYNTH_SIZE),
            Sprite {
                color: lorynth_color.value(),
                custom_size: Some(Vec2::splat(LORYNTH_SIZE)),
                ..default()
            },
        ));
    }
    
    // Dravym
    let (dravym_r, dravym_g, dravym_b) = DRAVYM_COLOR;
    let dravym_color = EntityColor::new(dravym_r, dravym_g, dravym_b);
    for _i in 0..NB_DRAVYM {
        let rand_speed_factor = rng.random_range(0.7..1.0);
        commands.spawn((
            entity_bundle.clone(),
            Name::new("Predator"),
            dravym_color.clone(),
            Species::Dravym,
            Energy::new(INITIAL_DRAVYM_ENERGY, MAX_DRAVYM_ENERGY),
            Prey::new(350.0),
            Hunter::new(vec![Species::Qirval, Species::Lorynth], 400.0),
            Speed::new(DRAVYM_MAX_SPEED * rand_speed_factor),
            Size::new(DRAVYM_SIZE),
            ActiveMover,
            Transform::from_xyz(
                -half_w + walls_paddings + DRAVYM_SIZE,
                half_h - walls_paddings - DRAVYM_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Collider::rectangle(DRAVYM_SIZE, DRAVYM_SIZE),
            Sprite {
                color: dravym_color.value(),
                custom_size: Some(Vec2::splat(DRAVYM_SIZE)),
                ..default()
            },
        ));
    }

    // Apex predators
    // Zyralith
    let (zyralith_r, zyralith_g, zyralith_b) = ZYRALITH_COLOR;
    let zyralith_color = EntityColor::new(zyralith_r, zyralith_g, zyralith_b);
    for _i in 0..NB_ZYRALITH {
        let rand_speed_factor = rng.random_range(0.7..1.0);
        commands.spawn((
            entity_bundle.clone(),
            Name::new("Super Predator"),
            zyralith_color.clone(),
            Species::Zyralith,
            Energy::new(INITIAL_ZYRALITH_ENERGY, MAX_ZYRALITH_ENERGY),
            Hunter::new(vec![Species::Dravym, Species::Lorynth], 500.0),
            Speed::new(ZYRALITH_MAX_SPEED * rand_speed_factor),
            Size::new(ZYRALITH_SIZE),
            ActiveMover,
            Transform::from_xyz(
                half_w - walls_paddings - ZYRALITH_SIZE,
                middle_wall_h + walls_paddings + ZYRALITH_SIZE,
                0.0,
            ),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
            )),
            Collider::rectangle(ZYRALITH_SIZE, ZYRALITH_SIZE),
            Sprite {
                color: zyralith_color.value(),
                custom_size: Some(Vec2::splat(ZYRALITH_SIZE)),
                ..default()
            },
        ));
    }
}

/// HUD
fn spawn_hud(commands: &mut Commands) {
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + (WINDOW_HEIGHT - WINDOW_WIDTH);
    let text_z = 1.0; // Ensure text is above other sprites

    // Title
    let title_y = middle_wall_h - TITLE_FONT_SIZE + WALLS_THICKNESS / 2.0;
    let title = commands
        .spawn((
            Text2d::new(TITLE),
            TextFont {
                font_size: TITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, title_y, text_z),
            TextLayout::new_with_justify(JustifyText::Center),
            HudTitle,
        ))
        .id();

    // Entity sprite
    let (start_r, start_g, start_b) = START_COLOR;
    let sprite_y = middle_wall_h - WALLS_THICKNESS / 2.0 - TITLE_FONT_SIZE - 64.0; // Below middle wall
    let sprite = commands
        .spawn((
            Sprite {
                color: Color::linear_rgb(start_r, start_g, start_b),
                custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
                ..default()
            },
            RigidBody::Kinematic,
            Transform::from_xyz(0.0, sprite_y - ENTITIES_SIZE / 2.0, 0.0),
            AngularVelocity(0.2),
            HudSprite,
        ))
        .id();

    // Details
    let details = commands
        .spawn((
            Text2d::new(START_DETAILS),
            TextFont {
                font_size: SUBTITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(-314.0, sprite_y - ENTITIES_SIZE / 2.0, text_z),
            TextLayout::new_with_justify(JustifyText::Right),
            HudStats,
        ))
        .id();

    // Statistics
    let stats = commands
        .spawn((
            Text2d::new(START_STATS),
            TextFont {
                font_size: SUBTITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(314.0, sprite_y - ENTITIES_SIZE / 2.0, text_z),
            TextLayout::new_with_justify(JustifyText::Left),
            HudStats,
        ))
        .id();

    // Description
    let description = commands
        .spawn((
            Text2d::new(START_DESCRIPTION),
            TextFont {
                font_size: TEXT_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, (-250.0 - 154.0 / 2.0 - half_h + 54.0) / 2.0, text_z),
            TextLayout::new_with_justify(JustifyText::Center),
            HudDescription,
        ))
        .id();

    // Resources
    commands.insert_resource(HudEntities {
        title,
        sprite,
        details,
        stats,
        description,
    });
}

/// DEBUG
fn spawn_debugger(commands: &mut Commands) {
    commands.spawn((
        Text2d::new(format!(
            "VERSION: V{} | LAB: L{} | RUN: R{} | FRAME N°0",
            VERSION_NAME, LAB_NAME, RUN_IT
        )),
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
