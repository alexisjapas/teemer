use crate::components::*;


pub const PREVIEW_MODE: bool = cfg!(feature = "preview");

/// Video
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
pub const TEXT_FONT_SIZE: f32 = 26.0;

/// Simulation
pub const MAX_SPEED: f32 = 64.0 * if PREVIEW_MODE { 2.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 3.0;
pub const HUNTING_REACTIVITY: f32 = 10.0;
pub const FLEEING_REACTIVITY: f32 = 10.0;

/// DEBUG
pub const TITLE: &str = "Region: Onomora | Biome: Thermal spring basin";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "1";
pub const RUN_IT: &str = "34";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// ENTITIES
// Plants
pub const NB_LYRVANE: i32 = 64;
pub const LYRVANE_SIZE: f32 = 6.0;
pub const MAX_LYRVANE_ENERGY: f32 = 180.0;
pub const INITIAL_LYRVANE_ENERGY: f32 = 100.0;
pub const LYRVANE_ENERGY_REGEN: f32 = 17.0;
pub const LYRVANE_COLOR: (f32, f32, f32) = (0.894, 0.471, 0.086);
pub const LYRVANE_MAX_SPEED: f32 = 0.0;

// Apex predators
pub const NB_ONYTHERON: i32 = 4;
pub const ONYTHERON_SIZE: f32 = 26.0;
pub const MAX_ONYTHERON_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_ONYTHERON_ENERGY: f32 = START_ENERGY;
pub const ONYTHERON_COLOR: (f32, f32, f32) = START_COLOR;
pub const ONYTHERON_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[    
    HudBatch {
        title: TITLE,
        sprite_color: PYRRALIS_COLOR,
        details: "Species: Pyrralis\n                      <<<\nType: Super predator",
        stats: concat!("Max energy: 550.0\n>>>                      \nMax speed: 70.0"),
        description:
"Serpentine super predator that coils around\n
Cindralys or Omyra. Pyrralis uses ambush tactics in\n
geothermal eddies, striking with calculated\n
precision. Its violet-blue iridescence shimmers\n
like a spectral heat wave in the mineral-rich\n
waters.",
    }
];
