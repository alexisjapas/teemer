use avian2d::prelude::*;
use bevy::{
    app::AppExit,
    prelude::*,
    render::view::screenshot::{Capturing, Screenshot, save_to_disk},
};

use crate::components::*;
use crate::config::*;
use crate::resources::*;

pub fn update_text(
    species_query: Query<&Species, Without<Text>>,
    mut text_query: Query<(&mut Text, &Species), With<Text>>,
) {
    let predators_count = species_query
        .iter()
        .filter(|species| **species == Species::Predator)
        .count();
    let prey_count = species_query
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
            Species::Prey => text.0 = format!("Prey: {}", prey_count),
            Species::Plant => text.0 = format!("Plants: {}", plants_count),
        }
    }
}

pub fn take_frame_screenshot(
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

pub fn preview_frame_counter(mut frame_counter: Local<u32>, mut exit: EventWriter<AppExit>) {
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE {
        println!("Generation done. Exiting.");
        exit.write(AppExit::Success);
        return;
    }
    *frame_counter += 1;
}

pub fn no_capture_in_progress(capturing: Query<(), With<Capturing>>) -> bool {
    capturing.is_empty()
}

pub fn manual_physics_step(mut physics_time: ResMut<Time<Physics>>) {
    println!("Physics step advancing.");
    physics_time.advance_by(std::time::Duration::from_secs_f32(FIXED_TIME_STEP));
}

// Hunting
pub fn assign_targets(
    mut predators: Query<(Entity, &mut Hunter, &Transform, &Species), With<Hunter>>,
    potential_prey: Query<(Entity, &Transform, &Species), (With<Species>, With<Consumable>)>,
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

pub fn predator_movement(
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

pub fn prey_movement(
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
            let max_force = flee_speed * 5.0; // Prey can change direction quickly when hunted
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

pub fn collision_kill_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionStarted>,
    query: Query<(Option<&Hunter>, Option<&Species>)>,
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
