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
        .add_systems(
            Startup,
            (
                setup,
                generate_world,
                spawn_entities,
                spawn_hud,
                spawn_debugger,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                idle_energy,
                plant_regeneration_system,
                assign_targets,
                update_vision_system,
                predator_movement,
                prey_movement,
                movement_energy,
                collision_kill_system,
                reproduction,
                death,
                update_hud,
                visualize_raycast,
            )
                .chain(),
        )
        // Avian's physics
        .insert_resource(Gravity(Vec2::ZERO))
        // Miscellaneous
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .init_resource::<FrameCount>()
        .add_systems(Update, update_debugger);

    // Preview vs generation
    if PREVIEW_MODE {
        println!("Preview mode: window + no capture.");
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32).into(),
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

    // Run
    app.run();
}

/// Setup
fn setup(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    // Load runtime configuration
    let runtime_config = RuntimeConfig::load().expect("Failed to load configuration");

    // Insert configuration as resource
    commands.insert_resource(GameConfig {
        runtime: runtime_config,
    });

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

        // Insert as a resource
        commands.insert_resource(SimulationMetadata {
            path_dir: sim_dir,
            name: str_date,
        });

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
}

/// Simulation
fn generate_world(mut commands: Commands, config: Res<GameConfig>) {
    let wall_restitution = 0.7;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + WALLS_THICKNESS / 2.0 + (WINDOW_HEIGHT - WINDOW_WIDTH);

    // Get current biome data
    let current_biome = config
        .runtime
        .lore
        .biomes
        .get(&config.runtime.simulation.biome)
        .expect("Current biome not found in lore config");
    let walls_color = Color::linear_rgb(
        current_biome.environment.frame_color[0],
        current_biome.environment.frame_color[1],
        current_biome.environment.frame_color[2],
    );
    let water_color = Color::linear_rgb(
        current_biome.environment.water_color[0],
        current_biome.environment.water_color[1],
        current_biome.environment.water_color[2],
    );
    commands.spawn((
        Sprite {
            color: water_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WINDOW_WIDTH)),
            ..default()
        },
        Transform::from_xyz(0.0, (WINDOW_HEIGHT - WINDOW_WIDTH) / 2.0, Z_WATER),
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
        Transform::from_xyz(0.0, half_h - WALLS_THICKNESS / 2.0, Z_HUD),
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
        Transform::from_xyz(0.0, middle_wall_h, Z_HUD),
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
        Transform::from_xyz(0.0, middle_wall_h - 64.0, Z_HUD),
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
        Transform::from_xyz(0.0, -half_h + 54.0, Z_HUD),
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
        Transform::from_xyz(0.0, -half_h + WALLS_THICKNESS / 2.0, Z_HUD),
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
        Transform::from_xyz(-half_w + WALLS_THICKNESS / 2.0, 0.0, Z_HUD),
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
        Transform::from_xyz(half_w - WALLS_THICKNESS / 2.0, 0.0, Z_HUD),
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
        Transform::from_xyz(-77.0, -254.0, Z_HUD),
        Restitution::new(wall_restitution),
    ));
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WALLS_THICKNESS, 154.0)),
            ..default()
        },
        RigidBody::Static,
        Transform::from_xyz(77.0, -254.0, Z_HUD),
        Restitution::new(wall_restitution),
    ));
    commands.spawn((
        Sprite {
            color: walls_color.clone(),
            custom_size: Some(Vec2::new(WINDOW_WIDTH, WALLS_THICKNESS)),
            ..default()
        },
        RigidBody::Static,
        Transform::from_xyz(0.0, -250.0 - 154.0 / 2.0, Z_HUD),
        Restitution::new(wall_restitution),
    ));
}

fn spawn_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    config: Res<GameConfig>,
) {
    let mut rng = rand::rng();

    let walls_paddings = WALLS_THICKNESS * 2.0 + 8.0;
    let half_w = WINDOW_WIDTH / 2.0;
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + WALLS_THICKNESS / 2.0 + (WINDOW_HEIGHT - WINDOW_WIDTH);

    let entity_bundle = (
        RigidBody::Dynamic,
        Restitution::new(0.2), // Bouncing restitution
        Friction::new(0.2),
        CollisionEventsEnabled,
        Consumable,
    );

    // Get current biome data
    let current_biome = config
        .runtime
        .lore
        .biomes
        .get(&config.runtime.simulation.biome)
        .expect("Current biome not found in lore config");

    // Spawn entities dynamically based on config
    for (species_key, species_data) in &current_biome.species {
        if let Some(population) = config.runtime.simulation.populations.get(species_key) {
            if *population == 0 {
                continue;
            }

            let params = EntitySpawnParams::from_species_data(species_data);
            let species_enum = Species::from_string(species_key)
                .expect(&format!("Unknown species: {}", species_key));
            let entity_color = EntityColor::new(params.color[0], params.color[1], params.color[2]);

            // Create hunting relationships
            let mut hunts = Vec::new();
            for prey_name in &species_data.eats {
                if let Some(prey_species) = Species::from_string(prey_name) {
                    hunts.push(prey_species);
                }
            }

            // Entity spawn
            let circle = Circle::new(params.size);
            for _i in 0..*population {
                let rand_speed_factor = rng.random_range(0.3..1.0);
                let angle = rng.random_range(0.0..std::f32::consts::TAU);
                let rotation = Quat::from_rotation_z(angle);
                let mut entity_commands = commands.spawn((
                    entity_bundle.clone(),
                    entity_color.clone(),
                    species_enum,
                    Energy::new(params.initial_energy, params.max_energy),
                    Size::new(params.size),
                    Speed::new(params.max_speed * rand_speed_factor),
                    Collider::circle(params.size),
                    Mesh2d(meshes.add(circle)),
                    MeshMaterial2d(materials.add(entity_color.value())),
                    Transform::from_xyz(
                        rng.random::<f32>() * (WINDOW_WIDTH - walls_paddings) - half_w
                            + walls_paddings,
                        rng.random::<f32>() * (WINDOW_WIDTH - walls_paddings) + middle_wall_h,
                        Z_ENTITIES,
                    )
                    .with_rotation(rotation),
                    LinearVelocity(Vec2::new(
                        params.max_speed * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                        params.max_speed * rand_speed_factor * (rng.random::<f32>() * 2.0 - 1.0),
                    )),
                ));

                // Add type-specific components
                match species_data.species_type.as_str() {
                    "Flora" => {
                        entity_commands.insert((
                            Name::new("Plant"),
                            Photosynthesis::new(params.photosynthesis_rate.unwrap_or(5.0)),
                        ));
                    }
                    "Fauna" => {
                        entity_commands.insert(Name::new("Fauna"));

                        // Add hunter component if this species eats others
                        if !hunts.is_empty() {
                            entity_commands
                                .insert(Hunter::new(hunts.clone(), params.detection_range));
                        }

                        // Add prey component if this species can be eaten
                        let can_be_eaten = current_biome
                            .species
                            .values()
                            .any(|other| other.eats.contains(species_key));
                        if can_be_eaten {
                            entity_commands.insert(Prey::new(params.detection_range));
                        }

                        // Add active mover for non-plant species
                        if params.is_active_mover {
                            entity_commands.insert(ActiveMover);
                        }

                        // Add vision with 3 rays at ±30°
                        entity_commands.insert(Vision::new(
                            params.detection_range,
                            99,
                            60.0_f32.to_radians(),
                        ));
                        entity_commands.insert(VisionResults::default());
                    }
                    _ => {
                        entity_commands.insert(Name::new("Unknown"));
                    }
                }
            }
        }
    }
}

/// HUD
fn spawn_hud(mut commands: Commands, config: Res<GameConfig>) {
    let half_h = WINDOW_HEIGHT / 2.0;
    let middle_wall_h = -half_h + (WINDOW_HEIGHT - WINDOW_WIDTH);
    let text_z = 1.0; // Ensure text is above other sprites

    // Get the first batch for initial display
    let batches = config.runtime.get_batches();
    let first_batch = batches.first().expect("No HUD batches available");

    // Insert HudBatches resource
    commands.insert_resource(HudBatches {
        batches: batches.to_vec(),
        index: 0,
    });

    // Title
    let title_y = middle_wall_h - TITLE_FONT_SIZE + WALLS_THICKNESS / 2.0;
    let title = commands
        .spawn((
            Text2d::new(config.runtime.get_title()),
            TextFont {
                font_size: TITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, title_y, text_z),
            TextLayout::new_with_justify(Justify::Center),
            HudTitle,
        ))
        .id();

    // Entity sprite
    let (start_r, start_g, start_b) = first_batch.sprite_color;
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
            Text2d::new(&first_batch.details),
            TextFont {
                font_size: SUBTITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(-314.0, sprite_y - ENTITIES_SIZE / 2.0, text_z),
            TextLayout::new_with_justify(Justify::Right),
            HudStats,
        ))
        .id();

    // Statistics
    let stats = commands
        .spawn((
            Text2d::new(&first_batch.stats),
            TextFont {
                font_size: SUBTITLE_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(314.0, sprite_y - ENTITIES_SIZE / 2.0, text_z),
            TextLayout::new_with_justify(Justify::Left),
            HudStats,
        ))
        .id();

    // Description
    let description = commands
        .spawn((
            Text2d::new(&first_batch.description),
            TextFont {
                font_size: TEXT_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform::from_xyz(0.0, (-250.0 - 154.0 / 2.0 - half_h + 54.0) / 2.0, text_z),
            TextLayout::new_with_justify(Justify::Center),
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
fn spawn_debugger(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn((
        Text2d::new(format!(
            "VERSION: V{} | LAB: L{} | RUN: R{} | FRAME N°0",
            config.runtime.simulation.simulation.version,
            config.runtime.simulation.simulation.lab_name,
            config.runtime.simulation.simulation.run_id
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
        TextLayout::new_with_justify(Justify::Center),
        DEBUGGER,
    ));
}
