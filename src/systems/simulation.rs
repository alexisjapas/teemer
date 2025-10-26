use avian2d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

use crate::components::*;
use crate::config::*;

/// Energy
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

/// Decision & movement
pub fn update_vision_system(
    mut query: Query<(Entity, &Transform, &Vision, &mut VisionResults)>,
    spatial_query: SpatialQuery,
) {
    for (entity, transform, vision, mut results) in &mut query {
        results.rays.clear();

        let start_angle = -vision.field_of_view / 2.0;
        let angle_step = if vision.nb_rays > 1 {
            vision.field_of_view / (vision.nb_rays - 1) as f32
        } else {
            0.0
        };

        let origin = transform.translation.truncate();
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;

        for i in 0..vision.nb_rays {
            let angle = start_angle + angle_step * i as f32;
            let local_direction = Vec2::new(angle.cos(), angle.sin());
            let world_direction = local_direction.rotate(Vec2::from_angle(rotation));

            let direction = Dir2::new(world_direction).unwrap_or(Dir2::X);

            let hit_info = spatial_query
                .cast_ray(
                    origin,
                    direction,
                    vision.detection_range,
                    true,
                    &SpatialQueryFilter::from_excluded_entities([entity]), // Don't hit self
                )
                .map(|hit| RayHitInfo {
                    entity: hit.entity,
                    distance: hit.distance,
                    point: origin + world_direction * hit.distance,
                });

            results.rays.push(RayResult {
                origin,
                direction: world_direction,
                max_distance: vision.detection_range,
                hit: hit_info,
            });
        }
    }
}

pub fn vision_analysis_system(
    mut entity_query: Query<(
        &VisionResults,
        &Species,
        Option<&Hunter>,
        &mut MovementIntent,
    )>,
    obstacles_query: Query<(&Species, Option<&Hunter>)>,
) {
    for (vision_result, species, hunter, mut movement_intent) in entity_query.iter_mut() {
        let mut direction = Vec2::ZERO;
        for ray in &vision_result.rays {
            if let Some(hit) = &ray.hit {
                let hit_entity = hit.entity;

                let weight =
                    if let Ok((hit_species, hit_hunter_opt)) = obstacles_query.get(hit_entity) {
                        if let Some(ray_owner_hunter) = hunter {
                            if ray_owner_hunter.hunts.contains(hit_species) {
                                // Hit entity is prey for ray owner -> Green
                                3.0
                            } else if let Some(hit_hunter) = hit_hunter_opt {
                                // Check if ray owner is prey for the hit entity
                                if hit_hunter.hunts.contains(species) {
                                    // Hit entity is predator for ray owner
                                    -5.0
                                } else {
                                    // Hit entity is neither prey nor predator for ray owner
                                    -0.0
                                }
                            } else {
                                // Hit entity is not a hunter
                                -0.0
                            }
                        } else {
                            // Ray owner is not a hunter, check if hit entity is a predator
                            if let Some(hit_hunter) = hit_hunter_opt {
                                if hit_hunter.hunts.contains(species) {
                                    // Hit entity is predator for ray owner
                                    -5.0
                                } else {
                                    // Hit entity doesn't hunt ray owner
                                    -0.0
                                }
                            } else {
                                // Hit entity is not a hunter
                                -0.0
                            }
                        }
                    } else {
                        // Hit entity has no species/hunter info
                        -0.0
                    };

                let dist_factor = 1.0 - (hit.distance / ray.max_distance);
                direction += ray.direction.normalize() * weight * dist_factor;
            }
        }

        if direction.length_squared() > 0.0 {
            movement_intent.desired_direction = direction.normalize();
            movement_intent.desired_force = direction.normalize() * 10.0;
        } else {
            movement_intent.desired_direction = Vec2::ZERO;
            movement_intent.desired_force = Vec2::ZERO;
        }
    }
}

pub fn apply_movement_system(
    mut query: Query<
        (
            &mut LinearVelocity,
            &mut AngularVelocity,
            &Transform,
            &MovementIntent,
            &Speed,
        ),
        With<ActiveMover>,
    >,
) {
    for (mut lin_vel, mut ang_vel, transform, movement_intent, speed) in query.iter_mut() {
        // Si pas d'intention de mouvement, on ne fait rien
        if movement_intent.desired_direction.length_squared() < 0.001 {
            continue;
        }

        // Direction actuelle de l'entité
        let forward_dir = transform.rotation.to_euler(EulerRot::XYZ).2;
        let facing = Vec2::from_angle(forward_dir);

        // Direction désirée
        let desired_dir = movement_intent.desired_direction.normalize_or_zero();

        // Calcul de l'angle entre direction actuelle et direction désirée
        let cross = facing.perp_dot(desired_dir);
        let dot = facing.dot(desired_dir);

        // Rotation angulaire proportionnelle à l'angle
        let turn_speed = 3.0; // Vitesse de rotation
        ang_vel.0 = cross * turn_speed;

        // Accélération vers l'avant si on regarde dans la bonne direction
        // (on avance plus vite quand on est aligné)
        let forward_factor = (dot * 0.5 + 0.5).max(0.0); // 0 à 1
        let acceleration = speed.value() * 20.0 * forward_factor; // Force d'accélération

        // Application de la force dans la direction désirée
        let force = desired_dir * acceleration * FIXED_TIME_STEP;
        lin_vel.0 += force;

        // Limitation de la vitesse max
        if lin_vel.length() > speed.value() {
            lin_vel.0 = lin_vel.normalize() * speed.value();
        }
    }
}

/// Life & death
pub fn collision_kill_system(
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionStart>,
    mut query: Query<(Option<&Hunter>, Option<&Species>, Option<&mut Energy>)>,
) {
    for event in collision_events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

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
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut entities: Query<
        (
            &Name,
            &EntityColor,
            &Species,
            &mut Energy,
            Option<&Hunter>,
            Option<&Photosynthesis>,
            Option<&Vision>,
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
        photosynthesis,
        vision,
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
                photosynthesis.map(|p| p.clone()),
                vision.map(|v| v.clone()),
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
        photosynthesis,
        vision,
        speed,
        size,
        active_mover,
        transform,
        _linear_velocity,
    ) in parents
    {
        let speed_value = speed.value();
        let size_value = size.value();
        let color_value = color.value();
        let circle = Circle::new(size_value);
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
            Collider::circle(size_value),
            Mesh2d(meshes.add(circle)),
            MeshMaterial2d(materials.add(color_value)),
            MovementIntent::default(),
        ));
        if let Some(hunter_component) = hunter {
            child.insert(hunter_component);
        }
        if let Some(photosynthesis_component) = photosynthesis {
            child.insert(photosynthesis_component);
        }
        if let Some(active_mover_component) = active_mover {
            child.insert(active_mover_component);
        }
        if let Some(vision_component) = vision {
            child.insert(vision_component);
            child.insert(VisionResults::default());
        }
    }
}
