mod components;
mod resources;

use avian2d::prelude::*;
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Capturing, Screenshot, save_to_disk},
};
use chrono::{DateTime, Utc};
use rand::random;
use std::fs;

use components::*;
use resources::*;

/// Video parameters
const PREVIEW_MODE: bool = cfg!(feature = "preview");
const WINDOW_WIDTH: f32 = 720.0;
const WINDOW_HEIGHT: f32 = 1280.0;
const FRAMERATE: f32 = 30.0;
const MAX_DURATION: f32 = 61.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };
const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// HUD
const WALLS_THICKNESS: f32 = 8.0;
const ENTITIES_SIZE: f32 = 44.0;
const FONT_SIZE: f32 = 24.0;

/// Simulation parameters
const NB_PREDATORS: i32 = 4;
const NB_PREYS: i32 = 32;
const NB_PLANTS: i32 = 64;
const SQUARE_LEN: f32 = 16.0;
const MAX_SPEED: f32 = 32.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };

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
        Collider::rectangle(SQUARE_LEN, SQUARE_LEN),
        Restitution::new(0.2), // Bouncing restitution
        Friction::new(0.1),
        LockedAxes::ROTATION_LOCKED,
        Prey::new(222.2),
        CollisionEventsEnabled,
    );

    // Predators
    for _i in 0..NB_PREDATORS {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(1.0, rand_color, rand_color),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
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
            Speed::new(MAX_SPEED * random::<f32>()),
            Name::new("Predator"),
        ));
    }

    // Preys
    for _i in 0..NB_PREYS {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(rand_color, rand_color, 1.0),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(half_w - walls_paddings, half_h - walls_paddings, 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Prey,
            Hunter::new(Species::Plant, 444.4),
            Speed::new(MAX_SPEED * random::<f32>()),
            Name::new("Prey"),
        ));
    }

    // Plants
    for _i in 0..NB_PLANTS {
        let rand_color = random::<f32>().min(0.1).max(0.0);
        commands.spawn((
            entity_bundle.clone(),
            Sprite {
                color: Color::linear_rgb(rand_color, 1.0, rand_color),
                custom_size: Some(Vec2::splat(SQUARE_LEN)),
                ..default()
            },
            Transform::from_xyz(0.0, -half_h + walls_paddings + (1280.0 - 720.0), 0.0),
            // Avian's physics
            LinearVelocity(Vec2::new(
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
                MAX_SPEED * (random::<f32>() * 2.0 - 1.0),
            )),
            Species::Plant,
            Hunter::new(Species::Plant, 444.4),
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
            font_size: FONT_SIZE,
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
        Sprite {
            color: Color::linear_rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 108.0 / 2.0, (WINDOW_HEIGHT / 2.0) - 720.0 - 60.0 - WALLS_THICKNESS - FONT_SIZE / 2.0, 0.0),
        // Avian's physics
        AngularVelocity(0.4)
    ));

    // Preys
    commands.spawn((
        Text::new(format!("Preys: {}", NB_PREYS)),
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
        Sprite {
            color: Color::linear_rgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 108.0 / 2.0, (WINDOW_HEIGHT / 2.0) - 944.0 - WALLS_THICKNESS - FONT_SIZE / 2.0, 0.0),
        // Avian's physics
        AngularVelocity(0.2)
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
        Sprite {
            color: Color::linear_rgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::splat(ENTITIES_SIZE)),
            ..default()
        },
        RigidBody::Kinematic,
        Transform::from_xyz(-WINDOW_WIDTH / 2.0 + 108.0 / 2.0, (WINDOW_HEIGHT / 2.0) - 1108.0 - WALLS_THICKNESS - FONT_SIZE / 2.0, 0.0),
        // Avian's physics
        AngularVelocity(0.1)
    ));
}

fn update_text(
    species_query: Query<&Species, Without<Text>>,
    mut text_query: Query<(&mut Text, &Species), With<Text>>,
) {
    let predators_count = species_query
        .iter()
        .filter(|species| **species == Species::Predator)
        .count();
    let preys_count = species_query
        .iter()
        .filter(|species| **species == Species::Prey)
        .count();
    let plants_count = species_query
        .iter()
        .filter(|species| **species == Species::Plant)
        .count();

    for (mut text, species) in text_query.iter_mut() {
        match species {
            Species::Predator => text.0 = format!("Predators: {}", predators_count),
            Species::Prey => text.0 = format!("Preys: {}", preys_count),
            Species::Plant => text.0 = format!("Plants: {}", plants_count),
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

fn manual_physics_step(mut physics_time: ResMut<Time<Physics>>) {
    println!("Physics step advancing.");
    physics_time.advance_by(std::time::Duration::from_secs_f32(FIXED_TIME_STEP));
}

// Hunting
fn assign_targets(
    mut predators: Query<(Entity, &mut Hunter, &Transform, &Species), With<Hunter>>,
    potential_prey: Query<(Entity, &Transform, &Species), With<Species>>,
) {
    for (_, mut predator, predator_transform, _) in predators.iter_mut() {
        // Clear target if out of range or dead
        if let Some(current_target) = predator.current_target {
            if let Ok((_, target_transform, _)) = potential_prey.get(current_target) {
                let distance = predator_transform
                    .translation
                    .distance(target_transform.translation);
                if distance > predator.detection_range {
                    predator.current_target = None;
                }
            } else {
                predator.current_target = None;
            }
        }

        // Find new target
        if predator.current_target.is_none() {
            let mut closest_distance = predator.detection_range;
            let mut closest_prey = None;

            for (prey_entity, prey_transform, prey_species) in potential_prey.iter() {
                if predator.hunts == *prey_species {
                    let distance = predator_transform
                        .translation
                        .distance(prey_transform.translation);
                    if distance < closest_distance {
                        closest_distance = distance;
                        closest_prey = Some(prey_entity);
                    }
                }
            }

            predator.current_target = closest_prey;
        }
    }
}

fn predator_movement(
    mut predators: Query<(&mut Hunter, &Transform, &mut LinearVelocity, &Speed)>,
    prey: Query<&Transform>,
) {
    for (mut predator, predator_transform, mut velocity, predator_speed) in predators.iter_mut() {
        if let Some(target_entity) = predator.current_target {
            if let Ok(target_transform) = prey.get(target_entity) {
                let current_pos = predator_transform.translation.truncate();
                let target_pos = target_transform.translation.truncate();
                let current_velocity = Vec2::new(velocity.x, velocity.y);
                let hunt_speed = predator_speed.value();

                // Compute desired velocity toward prey
                let direction = (target_pos - current_pos).normalize();
                let desired_velocity = direction * hunt_speed;

                // Apply steering force for smoother movements
                let steering_force = desired_velocity - current_velocity;
                let max_force = hunt_speed * 3.0;
                let steering_force = steering_force.clamp_length_max(max_force);

                let new_velocity = current_velocity + steering_force * FIXED_TIME_STEP;
                let new_velocity = new_velocity.clamp_length_max(hunt_speed);

                velocity.x = new_velocity.x;
                velocity.y = new_velocity.y;
            } else {
                predator.current_target = None;
            }
        } else {
            //todo!("If no target, slow down and move randomly.")
        }
    }
}

fn prey_movement(
    mut prey: Query<(&mut Prey, &Transform, &mut LinearVelocity, &Species, &Speed)>,
    predators: Query<(Entity, &Transform, &Hunter)>,
) {
    for (mut prey_comp, prey_transform, mut velocity, prey_species, prey_speed) in prey.iter_mut() {
        let mut flee_direction = Vec2::ZERO;
        let mut threat_found = false;

        for (predator_entity, predator_transform, hunter) in predators.iter() {
            // Check if predator predate
            if hunter.hunts == *prey_species {
                let distance = prey_transform
                    .translation
                    .distance(predator_transform.translation);

                if distance < prey_comp.detection_range {
                    // Compute where to flee
                    let flee_vec =
                        (prey_transform.translation - predator_transform.translation).normalize();
                    flee_direction += flee_vec.truncate() / (distance + 1.0);
                    threat_found = true;
                    prey_comp.current_threat = Some(predator_entity);
                }
            }
        }

        if threat_found {
            // Normalize the combined flee direction
            flee_direction = flee_direction.normalize();

            // Set flee speed
            let flee_speed = prey_speed.value();
            let desired_velocity = flee_direction * flee_speed;

            // Steering force for smoother movement
            let current_velocity = Vec2::new(velocity.x, velocity.y);
            let steering_force = desired_velocity - current_velocity;
            let max_force = flee_speed * 2.0; // Prey can change direction quickly when hunted
            let steering_force = steering_force.clamp_length_max(max_force);

            let new_velocity = current_velocity + steering_force * FIXED_TIME_STEP;
            let new_velocity = new_velocity.clamp_length_max(flee_speed);

            velocity.x = new_velocity.x;
            velocity.y = new_velocity.y;
        } else {
            prey_comp.current_threat = None;
            // TODO Add behaviour when not fleeing
        }
    }
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

        // Check Case 1: Entity1 is the predator, Entity2 is the prey
        if let (Some(predator), Some(prey_species)) = (entity1_comps.0, entity2_comps.1) {
            if predator.hunts == *prey_species {
                commands.entity(event.1).despawn();
                continue;
            }
        }

        // Check Case 2: Entity2 is the predator, Entity1 is the prey
        if let (Some(predator), Some(prey_species)) = (entity2_comps.0, entity1_comps.1) {
            if predator.hunts == *prey_species {
                commands.entity(event.0).despawn();
            }
        }
    }
}
