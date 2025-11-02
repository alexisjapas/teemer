use crate::components::HudBatch;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

pub const PREVIEW_MODE: bool = cfg!(feature = "preview");

/// Video
pub const WINDOW_WIDTH: f32 = 1080.0;
pub const WINDOW_HEIGHT: f32 = 1920.0;
pub const FRAMERATE: f32 = 30.0;
pub const MAX_DURATION: f32 = 61.0;
pub const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
pub const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

///Z-axis
pub const Z_WATER: f32 = 0.0;
pub const Z_HUD: f32 = 1.0;
pub const Z_ENTITIES: f32 = 2.0;

/// HUD
pub const WALLS_THICKNESS: f32 = 8.0;
pub const ENTITIES_SIZE: f32 = 64.0;
pub const TITLE_FONT_SIZE: f32 = 32.0;
pub const SUBTITLE_FONT_SIZE: f32 = 28.0;
pub const TEXT_FONT_SIZE: f32 = 26.0;

/// Simulation
// Energy
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 3.0;
// Vision
pub const WEIGHT_PREY: f32 = 6.0;
pub const WEIGHT_PREDATOR: f32 = -18.0;
pub const WEIGHT_NEUTRAL: f32 = 0.0;
// Movements
pub const TURN_RESPONSIVENESS: f32 = 20.0;
pub const ACCELERATION_FORCE: f32 = 800.0;
pub const LINEAR_DAMPING: f32 = 0.5; // Simulates water drag (not friction)
pub const ANGULAR_DAMPING: f32 = 1.0;
pub const FORWARD_ALIGNMENT_THRESHOLD: f32 = 0.2;

/// DEBUG
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// TOML Configuration Structures
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LoreConfig {
    pub biomes: HashMap<String, BiomeData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BiomeData {
    pub name: String,
    #[serde(rename = "type")]
    pub biome_type: String,
    pub environment: Environment,
    pub species: HashMap<String, SpeciesData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Environment {
    pub water_color: [f32; 3],
    pub frame_color: [f32; 3],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpeciesData {
    pub name: String,
    #[serde(rename = "type")]
    pub species_type: String,
    pub size: i32,
    pub color: [f32; 3],
    pub description: String,
    #[serde(default)]
    pub eats: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulationConfig {
    pub simulation: SimulationMeta,
    pub biome: String,
    pub populations: HashMap<String, u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimulationMeta {
    pub version: String,
    pub lab_name: String,
    pub run_id: String,
}

/// Text wrapping utility for descriptions
pub fn wrap_text(text: &str, max_width: usize) -> String {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        // Check if adding this word would exceed the limit
        let potential_length = if current_line.is_empty() {
            word.len()
        } else {
            current_line.len() + 1 + word.len() // +1 for the space
        };

        if potential_length <= max_width {
            // Add word to current line
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            // Start new line with this word
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        }
    }

    // Don't forget the last line
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines.join("\n")
}

/// Configuration Loading Functions
pub fn load_lore_config(path: &str) -> Result<LoreConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: LoreConfig = toml::from_str(&content)?;
    Ok(config)
}

pub fn load_simulation_config(path: &str) -> Result<SimulationConfig, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: SimulationConfig = toml::from_str(&content)?;
    Ok(config)
}

/// Runtime Configuration
pub struct RuntimeConfig {
    pub lore: LoreConfig,
    pub simulation: SimulationConfig,
    pub title: String,
    pub batches: Vec<HudBatch>,
}

impl RuntimeConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Load configuration files with better error context
        let lore = load_lore_config("src/lore.toml")
            .map_err(|e| format!("Failed to load lore config: {}", e))?;
        let simulation = load_simulation_config("src/simulation.toml")
            .map_err(|e| format!("Failed to load simulation config: {}", e))?;

        // Validate biome exists
        let current_biome_key = &simulation.biome;
        let current_biome = lore.biomes.get(current_biome_key).ok_or(format!(
            "Biome '{}' not found in lore config. Available biomes: {:?}",
            current_biome_key,
            lore.biomes.keys().collect::<Vec<_>>()
        ))?;

        // Validate all population species exist in the biome
        for species_key in simulation.populations.keys() {
            if !current_biome.species.contains_key(species_key) {
                return Err(format!(
                    "Population species '{}' not found in biome '{}'. Available species: {:?}",
                    species_key,
                    current_biome_key,
                    current_biome.species.keys().collect::<Vec<_>>()
                )
                .into());
            }
        }

        // Validate eating relationships
        for (species_key, species_data) in &current_biome.species {
            for prey_name in &species_data.eats {
                if !current_biome.species.contains_key(prey_name) {
                    return Err(format!(
                        "Species '{}' eats '{}' but '{}' is not defined in biome '{}'",
                        species_key, prey_name, prey_name, current_biome_key
                    )
                    .into());
                }
            }
        }

        // Generate title from biome info
        let title = format!(
            "Region: {} | Biome: {}",
            current_biome.name, current_biome.biome_type
        );

        // Generate HUD batches from species data
        let mut batches = Vec::new();
        for (_species_key, species_data) in &current_biome.species {
            if species_data.species_type == "Fauna" {
                let batch = HudBatch {
                    title: title.clone(),
                    sprite_color: (
                        species_data.color[0],
                        species_data.color[1],
                        species_data.color[2],
                    ),
                    details: format!(
                        "Species: {}\n                      <<<\nType: {}",
                        species_data.name, species_data.species_type
                    ),
                    stats: format!(
                        "Size: {}\n>>>                      \nType: {}",
                        species_data.size, species_data.species_type
                    ),
                    description: wrap_text(&species_data.description, 52),
                };
                batches.push(batch);
            }
        }

        // Ensure we have at least one batch for HUD
        if batches.is_empty() {
            return Err(format!(
                "No Fauna species found in biome '{}' for HUD display",
                current_biome_key
            )
            .into());
        }

        Ok(Self {
            lore,
            simulation,
            title,
            batches,
        })
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_batches(&self) -> &[HudBatch] {
        &self.batches
    }
}

/// Entity spawning parameters derived from config
#[derive(Debug, Clone)]
pub struct EntitySpawnParams {
    pub size: f32,
    pub color: [f32; 3],
    pub max_speed: f32,
    pub initial_energy: f32,
    pub max_energy: f32,
    pub detection_range: f32,
    pub is_active_mover: bool,
    pub photosynthesis_rate: Option<f32>,
}

impl EntitySpawnParams {
    pub fn from_species_data(data: &SpeciesData) -> Self {
        let is_plant = data.species_type.to_lowercase() == "flora";
        let size = data.size as f32;

        // Scale parameters based on size and type
        let base_speed = if is_plant { 0.0 } else { 20.0 + size * 2.0 };
        let base_energy = size * 50.0;
        let detection_range = 10.0 + size * 10.0;

        Self {
            size: size,
            color: data.color,
            max_speed: base_speed,
            initial_energy: base_energy * 0.7, // Start at 70% of max
            max_energy: base_energy,
            detection_range,
            is_active_mover: !is_plant,
            photosynthesis_rate: if is_plant { Some(10.0) } else { None },
        }
    }
}
