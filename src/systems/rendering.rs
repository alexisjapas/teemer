use avian2d::prelude::*;
use bevy::{app::AppExit, diagnostic::FrameCount, prelude::*};
use bevy_capture::{encoder::mp4_ffmpeg_cli_pipe, Capture};
use std::time::Instant;

use crate::components::*;
use crate::config::*;
use crate::resources::*;

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

pub fn update_debugger(
    frame_count: Res<FrameCount>,
    config: Res<GameConfig>,
    mut debugger_query: Query<&mut Text2d, With<DEBUGGER>>,
) {
    if let Ok(mut text) = debugger_query.single_mut() {
        **text = format!(
            "VERSION: V{} | LAB: L{} | RUN: R{} | FRAME F{}",
            env!("CARGO_PKG_VERSION"),
            config.runtime.simulation.simulation.lab_name,
            config.runtime.simulation.simulation.run_id,
            frame_count.0
        );
    }
}

pub fn visualize_raycast(
    mut commands: Commands,
    ray_query: Query<(&VisionResults, Option<&Hunter>, &Species)>,
    old_viz: Query<Entity, Or<(With<RaycastVisualization>, With<HitPointVisualization>)>>,
    entity_query: Query<(&Species, Option<&Hunter>)>,
) {
    // Clean up old visualizations
    for entity in &old_viz {
        commands.entity(entity).despawn();
    }

    // Draw new visualizations
    for (results, ray_owner_hunter, ray_owner_species) in &ray_query {
        for ray in &results.rays {
            let origin = ray.origin;
            let direction = ray.direction;
            let max_distance = ray.max_distance;

            // Determine the end point and color based on what we hit
            let (end_point, line_color) = if let Some(hit) = &ray.hit {
                let hit_entity = hit.entity;

                // Determine color based on the relationship between ray owner and hit entity
                let color = if let Ok((hit_species, hit_hunter_opt)) = entity_query.get(hit_entity)
                {
                    // Check if hit entity is prey for the ray owner
                    if let Some(ray_owner_hunter) = ray_owner_hunter {
                        if ray_owner_hunter.hunts.contains(hit_species) {
                            // Hit entity is prey for ray owner -> Green
                            Color::srgba(0.0, 1.0, 0.0, 0.04)
                        } else if let Some(hit_hunter) = hit_hunter_opt {
                            // Check if ray owner is prey for the hit entity
                            if hit_hunter.hunts.contains(ray_owner_species) {
                                // Hit entity is predator for ray owner -> Red
                                Color::srgba(1.0, 0.0, 0.0, 0.04)
                            } else {
                                // Hit entity is neither prey nor predator -> Blue
                                Color::srgba(0.0, 0.0, 1.0, 0.04)
                            }
                        } else {
                            // Hit entity is not a hunter, not in prey list -> Blue
                            Color::srgba(0.0, 0.0, 1.0, 0.04)
                        }
                    } else {
                        // Ray owner is not a hunter, check if hit entity is a predator
                        if let Some(hit_hunter) = hit_hunter_opt {
                            if hit_hunter.hunts.contains(ray_owner_species) {
                                // Hit entity is predator for ray owner -> Red
                                Color::srgba(1.0, 0.0, 0.0, 0.04)
                            } else {
                                // Hit entity doesn't hunt ray owner -> Blue
                                Color::srgba(0.0, 0.0, 1.0, 0.04)
                            }
                        } else {
                            // Hit entity is not a hunter -> Blue
                            Color::srgba(0.0, 0.0, 1.0, 0.04)
                        }
                    }
                } else {
                    // Hit entity has no species/hunter info -> Blue
                    Color::srgba(0.0, 0.0, 1.0, 0.04)
                };

                (hit.point, color)
            } else {
                // No hit -> Gray
                (
                    origin + direction * max_distance,
                    Color::srgba(0.5, 0.5, 0.5, 0.04),
                )
            };

            // Calculate line properties
            let length = (end_point - origin).length();
            let angle = direction.y.atan2(direction.x);
            let midpoint = (origin + end_point) / 2.0;

            // Spawn the ray line
            commands.spawn((
                Sprite {
                    color: line_color,
                    custom_size: Some(Vec2::new(length, 2.0)),
                    ..default()
                },
                Transform::from_translation(midpoint.extend(2.0))
                    .with_rotation(Quat::from_rotation_z(angle)),
                RaycastVisualization,
            ));
        }
    }
}

/// Capture
pub fn capture_frame(
    mut app_exit: MessageWriter<AppExit>,
    mut capture_q: Query<&mut Capture>,
    mut frame_counter: Local<u32>,
    mut stop_requested: Local<bool>,
    mut stop_requested_at: Local<Option<Instant>>,
    simulation_metadata: Res<SimulationMetadata>,
) {
    let mut capture = capture_q.single_mut().unwrap();

    if !capture.is_capturing() && !*stop_requested {
        capture.start(
            mp4_ffmpeg_cli_pipe::Mp4FfmpegCliPipeEncoder::new(format!(
                "{}/{}.mp4",
                simulation_metadata.path_dir, simulation_metadata.name
            ))
            .expect("Failed to create MP4 encoder")
            .with_framerate(30)
            .with_crf(18)
            .with_preset("p7".to_string()),
        );
    }

    *frame_counter += 1;
    println!("{}", *frame_counter);

    // When we reach the frame limit: request a stop (don't exit yet).
    if *frame_counter >= MAX_FRAMES_TO_CAPTURE && !*stop_requested {
        *stop_requested = true;
        *stop_requested_at = Some(Instant::now());
        // Prefer calling the API stop() if available:
        capture.stop(); // if Capture exposes stop(); otherwise see note below.
        println!(
            "Requested capture stop at frame {}, waiting for encoder to finish...",
            *frame_counter
        );
    }

    // If stop was requested, wait for capture to end (or timeout).
    if *stop_requested {
        // If capture finished cleanly -> exit
        if !capture.is_capturing() {
            println!("Capture finished — exiting.");
            app_exit.write(AppExit::Success);
            return;
        }

        // Safety: force exit if encoder never finishes within a reasonable wall-clock time.
        if let Some(started) = *stop_requested_at {
            if started.elapsed().as_secs() > 60 {
                eprintln!("Capture did not finish within 60s — forcing exit.");
                app_exit.write(AppExit::Success);
            }
        }
    }
}

pub fn manual_physics_step(mut physics_time: ResMut<Time<Physics>>) {
    println!("Physics step advancing.");
    physics_time.advance_by(std::time::Duration::from_secs_f32(FIXED_TIME_STEP));
}
