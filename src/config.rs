use crate::components::*;

/// Video parameters
pub const PREVIEW_MODE: bool = cfg!(feature = "preview");
pub const WINDOW_WIDTH: f32 = 1080.0;
pub const WINDOW_HEIGHT: f32 = 1920.0;
pub const FRAMERATE: f32 = 30.0;
pub const MAX_DURATION: f32 = 61.0;
pub const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
pub const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// HUD
pub const WALLS_THICKNESS: f32 = 8.0;
pub const ENTITIES_SIZE: f32 = 64.0;
pub const TITLE_FONT_SIZE: f32 = 32.0;
pub const SUBTITLE_FONT_SIZE: f32 = 28.0;
pub const TEXT_FONT_SIZE: f32 = 24.0;

/// Simulation parameters
pub const MAX_SPEED: f32 = 64.0 * if PREVIEW_MODE { 2.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 2.0;
pub const HUNTING_REACTIVITY: f32 = 10.0;
pub const FLEEING_REACTIVITY: f32 = 10.0;

pub const NB_SUPER_PREDATORS: i32 = 2;
pub const SUPER_PREDATOR_SIZE: f32 = 20.0;
pub const MAX_SUPER_PREDATOR_ENERGY: f32 = 400.0;
pub const INITIAL_SUPER_PREDATOR_ENERGY: f32 = 300.0;

pub const NB_PREDATORS: i32 = 10;
pub const PREDATOR_SIZE: f32 = 14.0;
pub const MAX_PREDATOR_ENERGY: f32 = 400.0;
pub const INITIAL_PREDATOR_ENERGY: f32 = 300.0;

pub const NB_PREY: i32 = 6;
pub const PREY_SIZE: f32 = 10.0;
pub const MAX_PREY_ENERGY: f32 = 200.0;
pub const INITIAL_PREY_ENERGY: f32 = 150.0;

pub const NB_PLANTS: i32 = 16;
pub const PLANT_SIZE: f32 = 6.0;
pub const MAX_PLANT_ENERGY: f32 = 80.0;
pub const INITIAL_PLANT_ENERGY: f32 = 50.0;
pub const PLANT_ENERGY_REGEN: f32 = 6.0;

/// DEBUG
pub const TITLE: &str = "Region: Irr'Hakur desert | Biome: Oasis pond";
pub const VERSION_NAME: &str = "Aquatic chaos";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 150;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[
    HudBatch {
        title: TITLE,
        sprite_color: (0.1, 0.2, 0.3),
        details: "Species: Toutou\n                      <<<\nType: Superpredator",
        stats: "Max energy: 100\n>>>                      \nMax speed: 100",
        description: "You are in the forest",
    },
    HudBatch {
        title: TITLE,
        sprite_color: (0.3, 0.2, 0.1),
        details: "Species: Poiuyt\n                      <<<\nType: Plant",
        stats: "Max energy: 100\n>>>                      \nMax speed: 100",
        description: "You are in the fdezdforest",
    },
];
