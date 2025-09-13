use avian2d::prelude::*;
use bevy::{app::AppExit, diagnostic::FrameCount, prelude::*};
use bevy_capture::{Capture, encoder::frames};
use rand::prelude::*;

use crate::components::*;
use crate::config::*;
use crate::resources::*;

/// Simulation
pub fn idle_energy(mut entities: Query<&mut Energy, With<Species>>) {
    let mut rng = rand::rng();
    for mut energy in entities.iter_mut() {
        energy.lose(IDLE_ENERGY_LOSS * FIXED_TIME_STEP * rng.random::<f32>());
    }
}

pub fn plant_regeneration_system(
    mut plants: Query<(&mut Energy, &Photosynthesis), With<Photosynthesis>>,
) {
    let mut rng = rand::rng();
    for (mut energy, photosynthesis) in plants.iter_mut() {
        energy.gain(photosynthesis.value() * FIXED_TIME_STEP * rng.random::<f32>());
    }
}

pub fn movement_energy(
    mut entities: Query<(&mut Energy, &LinearVelocity, &Size), With<ActiveMover>>,
) {
    // todo only take into account active velocity. Take mass into account.
    for (mut energy, velocity, size) in entities.iter_mut() {
        let speed = velocity.length();
        let energy_cost = speed
            * speed
            * size.value()
            * size.value()
            * MOVEMENT_ENERGY_COST_FACTOR
            * FIXED_TIME_STEP;
        energy.lose(energy_cost);
    }
}

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
                if predator.hunts.contains(prey_species) {
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
                let max_force = hunt_speed * HUNTING_REACTIVITY; // TODO set a per entity component
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
            if hunter.hunts.contains(prey_species) {
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
            let max_force = flee_speed * FLEEING_REACTIVITY; // Prey can change direction quickly when hunted
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
    mut query: Query<(Option<&Hunter>, Option<&Species>, Option<&mut Energy>)>,
) {
    for event in collision_events.read() {
        let entity1 = event.0;
        let entity2 = event.1;

        // Get the components for both entities involved in the collision
        let Ok([mut entity1_comps, mut entity2_comps]) = query.get_many_mut([entity1, entity2])
        else {
            continue;
        };

        // Check Case 1: Entity1 is the predator, Entity2 is the prey
        if let (Some(predator), Some(prey_species)) = (entity1_comps.0, entity2_comps.1) {
            if predator.hunts.contains(prey_species) {
                // Get part of prey energy
                if let Some(prey_energy) = entity2_comps.2.as_ref() {
                    let energy_gained = prey_energy.value() * ENERGY_TRANSFER_RATE;
                    if let Some(predator_energy) = entity1_comps.2.as_mut() {
                        predator_energy.gain(energy_gained);
                    }
                }
                // Kill the entity
                commands.entity(entity2).despawn();
                continue;
            }
        }

        // Check Case 2: Entity2 is the predator, Entity1 is the prey
        if let (Some(predator), Some(prey_species)) = (entity2_comps.0, entity1_comps.1) {
            if predator.hunts.contains(prey_species) {
                // Get part of prey energy
                if let Some(prey_energy) = entity1_comps.2.as_ref() {
                    let energy_gained = prey_energy.value() * ENERGY_TRANSFER_RATE;
                    if let Some(predator_energy) = entity2_comps.2.as_mut() {
                        predator_energy.gain(energy_gained);
                    }
                }
                // Kill entity
                commands.entity(entity1).despawn();
            }
        }
    }
}

pub fn death(mut commands: Commands, entities: Query<(Entity, &Energy), With<Species>>) {
    for (entity, energy) in entities.iter() {
        if energy.value() <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn reproduction(
    mut commands: Commands,
    mut entities: Query<
        (
            &Name,
            &EntityColor,
            &Species,
            &mut Energy,
            Option<&Hunter>,
            Option<&Prey>,
            Option<&Photosynthesis>,
            &Speed,
            &Size,
            Option<&ActiveMover>,
            &Transform,
            &LinearVelocity,
        ),
        With<Species>,
    >,
) {
    // Prepare children common attributes
    let entity_bundle = (
        RigidBody::Dynamic,
        Restitution::new(0.2), // Bouncing restitution
        Friction::new(0.2),
        LockedAxes::ROTATION_LOCKED,
        CollisionEventsEnabled,
        Consumable,
    );

    // Get parents
    let mut parents = Vec::new();
    for (
        name,
        color,
        species,
        mut energy,
        hunter,
        prey,
        photosynthesis,
        speed,
        size,
        active_mover,
        transform,
        linear_velocity,
    ) in entities.iter_mut()
    {
        if energy.value() >= 0.8 * energy.max {
            // Lose energy (10%)
            // then divide it by two so it is shared between the entity and its child
            let energy_loss = energy.value() * 0.55;
            energy.lose(energy_loss);

            // Adds to parents
            parents.push((
                name.clone(),
                color.clone(),
                species.clone(),
                energy.clone(),
                hunter.map(|h| h.clone()),
                prey.map(|p| p.clone()),
                photosynthesis.map(|p| p.clone()),
                speed.clone(),
                size.clone(),
                active_mover.map(|a| a.clone()),
                *transform,
                *linear_velocity,
            ))
        }
    }

    // Spawn children (perfect clones)
    for (
        name,
        color,
        species,
        energy,
        hunter,
        prey,
        photosynthesis,
        speed,
        size,
        active_mover,
        transform,
        linear_velocity,
    ) in parents
    {
        let speed_value = speed.value();
        let size_value = size.value();
        let color_value = color.value();
        let mut child = commands.spawn((
            entity_bundle.clone(),
            name,
            color,
            species,
            energy,
            speed,
            size,
            transform,
            LinearVelocity(Vec2::new(
                (10.0 + speed_value) * (rand::random::<f32>() * 2.0 - 1.0),
                (10.0 + speed_value) * (rand::random::<f32>() * 2.0 - 1.0),
            )),
            Collider::rectangle(size_value, size_value),
            Sprite {
                color: color_value,
                custom_size: Some(Vec2::splat(size_value)),
                ..default()
            },
        ));
        if let Some(hunter_component) = hunter {
            child.insert(hunter_component);
        }
        if let Some(prey_component) = prey {
            child.insert(prey_component);
        }
        if let Some(photosynthesis_component) = photosynthesis {
            child.insert(photosynthesis_component);
        }
        if let Some(active_mover_component) = active_mover {
            child.insert(active_mover_component);
        }
    }
}

/// Capture
pub fn capture_frame(
    mut app_exit: EventWriter<AppExit>,
    mut capture: Query<&mut Capture>,
    mut frame_counter: Local<u32>,
    frames_dir: Res<FramesDir>,
) {
    let mut capture = capture.single_mut().unwrap();
    if !capture.is_capturing() {
        capture.start(frames::FramesEncoder::new(format!("{}", frames_dir.0)));
    }

    *frame_counter += 1;
    println!("{}", *frame_counter);
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE {
        app_exit.write(AppExit::Success);
    }
}

pub fn preview_frame_counter(mut frame_counter: Local<u32>, mut exit: EventWriter<AppExit>) {
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE {
        println!("Generation done. Exiting.");
        exit.write(AppExit::Success);
        return;
    }
    *frame_counter += 1;
}

pub fn manual_physics_step(mut physics_time: ResMut<Time<Physics>>) {
    println!("Physics step advancing.");
    physics_time.advance_by(std::time::Duration::from_secs_f32(FIXED_TIME_STEP));
}

/// HUD
pub fn update_hud(
    frame_count: Res<FrameCount>,
    mut hud_batches: ResMut<HudBatches>,
    hud_entities: Res<HudEntities>,
    mut text_query: Query<&mut Text2d>,
    mut sprite_query: Query<&mut Sprite>,
) {
    if frame_count.0 % FRAMES_PER_UPDATE == 0 && frame_count.0 > 0 {
        // Get next batch
        hud_batches.index = (hud_batches.index + 1) % hud_batches.batches.len();
        let batch = &hud_batches.batches[hud_batches.index];

        // Update text fields
        if let Ok(mut text) = text_query.get_mut(hud_entities.title) {
            **text = batch.title.to_string();
        }
        if let Ok(mut text) = text_query.get_mut(hud_entities.details) {
            **text = batch.details.to_string();
        }
        if let Ok(mut text) = text_query.get_mut(hud_entities.stats) {
            **text = batch.stats.to_string();
        }
        if let Ok(mut text) = text_query.get_mut(hud_entities.description) {
            **text = batch.description.to_string();
        }

        // Update sprite color
        if let Ok(mut sprite) = sprite_query.get_mut(hud_entities.sprite) {
            let (r, g, b) = batch.sprite_color;
            sprite.color = Color::linear_rgb(r, g, b);
        }
    }
}

/// DEBUG
pub fn update_debugger(
    frame_count: Res<FrameCount>,
    config: Res<GameConfig>,
    mut debugger_query: Query<&mut Text2d, With<DEBUGGER>>,
) {
    if let Ok(mut text) = debugger_query.single_mut() {
        **text = format!(
            "VERSION: V{} | LAB: L{} | RUN: R{} | FRAME F{}",
            config.runtime.simulation.simulation.version,
            config.runtime.simulation.simulation.lab_name,
            config.runtime.simulation.simulation.run_id,
            frame_count.0
        );
    }
}
