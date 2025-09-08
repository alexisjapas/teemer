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
pub const TEXT_FONT_SIZE: f32 = 26.0;

/// Simulation parameters
pub const MAX_SPEED: f32 = 64.0 * if PREVIEW_MODE { 2.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 3.0;
pub const HUNTING_REACTIVITY: f32 = 10.0;
pub const FLEEING_REACTIVITY: f32 = 10.0;

/// DEBUG
pub const TITLE: &str = "Region: Onomora | Biome: Swamp";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "1";
pub const RUN_IT: &str = "28";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// WORLD
pub const WALLS_COLOR: (f32, f32, f32) = (0.18, 0.35, 0.12);
pub const WATER_COLOR: (f32, f32, f32) = (0.38, 0.34, 0.26);

/// FIRST ELEMENT (latest in the batch)
pub const START_MAX_ENERGY: f32 = 900.0;
pub const START_ENERGY: f32 = 600.0;
pub const START_MAX_SPEED: f32 = 40.0;
pub const START_DETAILS: &str = "Species: Zyralith\n                      <<<\nType: Apex predator";
pub const START_STATS: &str = "Max energy: 900.0\n>>>                      \nMax speed: 40.0";
pub const START_DESCRIPTION: &str =
"Apex predator of Irr’Umar, inhabiting the deepest,
most shaded areas. Zyralith ignores Qirval,
allowing primary grazers to flourish, but dominates
all higher trophic levels. Its massive indigo-black
body blends seamlessly into the swamp shadows,
silently controlling the ecosystem.";
pub const START_COLOR: (f32, f32, f32) = (0.138, 0.125, 0.245);

/// ENTITIES
// Plants
pub const NB_VYRMOSA: i32 = 512;
pub const VYRMOSA_SIZE: f32 = 6.0;
pub const MAX_VYRMOSA_ENERGY: f32 = 150.0;
pub const INITIAL_VYRMOSA_ENERGY: f32 = 100.0;
pub const VYRMOSA_ENERGY_REGEN: f32 = 10.0;
pub const VYRMOSA_COLOR: (f32, f32, f32) = (0.160, 0.690, 0.250);
pub const VYRMOSA_MAX_SPEED: f32 = 0.0;

// Prey
pub const NB_QIRVAL: i32 = 64;
pub const QIRVAL_SIZE: f32 = 8.0;
pub const MAX_QIRVAL_ENERGY: f32 = 200.0;
pub const INITIAL_QIRVAL_ENERGY: f32 = 150.0;
pub const QIRVAL_COLOR: (f32, f32, f32) = (0.925, 0.803, 0.365);
pub const QIRVAL_MAX_SPEED: f32 = 90.0;

// Predators p1
pub const NB_LORYNTH: i32 = 32;
pub const LORYNTH_SIZE: f32 = 12.0;
pub const MAX_LORYNTH_ENERGY: f32 = 450.0;
pub const INITIAL_LORYNTH_ENERGY: f32 = 350.0;
pub const LORYNTH_COLOR: (f32, f32, f32) = (0.678, 0.325, 0.298);
pub const LORYNTH_MAX_SPEED: f32 = 70.0;

// Predators p2
pub const NB_DRAVYM: i32 = 16;
pub const DRAVYM_SIZE: f32 = 18.0;
pub const MAX_DRAVYM_ENERGY: f32 = 700.0;
pub const INITIAL_DRAVYM_ENERGY: f32 = 500.0;
pub const DRAVYM_COLOR: (f32, f32, f32) = (0.432, 0.357, 0.568);
pub const DRAVYM_MAX_SPEED: f32 = 60.0;

// Apex predators
pub const NB_ZYRALITH: i32 = 8;
pub const ZYRALITH_SIZE: f32 = 24.0;
pub const MAX_ZYRALITH_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_ZYRALITH_ENERGY: f32 = START_ENERGY;
pub const ZYRALITH_COLOR: (f32, f32, f32) = START_COLOR;
pub const ZYRALITH_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[    
    HudBatch {
        title: TITLE,
        sprite_color: VYRMOSA_COLOR,
        details: "Species: Vyrmosa\n                      <<<\nType: Plant",
        stats: "Max energy: 150.0\n>>>                      \nMax speed: 0.0",
        description:
"Dense, filamentous plant forming floating mats and\n
root-like tangles among submerged debris. Vyrmosa\n
thrives in tannin-rich, low-oxygen waters,\n
providing both shelter and the primary energy\n
source for the swamp. Its green filaments shimmer\n
faintly under dim light, harboring micro-fauna.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: QIRVAL_COLOR,
        details: "Species: Qirval\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 200.0\n>>>                      \nMax speed: 90.0"),
        description:
"Small, nimble grazer that feeds on Vyrmosa\n
filaments. Qirval moves in swarms, creating\n
rippling currents and shaping the spatial\n
distribution of plant mats. Its golden-yellow\n
segmented body is reinforced to survive murky,\n
detritus-laden waters.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: LORYNTH_COLOR,
        details: "Species: Lorynth\n                      <<<\nType: Predator",
        stats: concat!("Max energy: 450.0\n>>>                      \nMax speed: 70.0"),
        description:
"Agile predator of Qirval, Lorynth darts along\n
submerged roots and detritus with short bursts of\n
speed. Its reddish-brown armored body helps it\n
blend into the dark, tannin-stained water.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: DRAVYM_COLOR,
        details: "Species: Dravym\n                      <<<\nType: Super predator",
        stats: concat!("Max energy: 700.0\n>>>                      \nMax speed: 60.0"),
        description:
"Elongated, serpentine super predator that hunts\n
both Lorynth and Qirval. Dravym moves deliberately,\n
wrapping around prey before crushing or engulfing\n
it. Its violet-blue body is semi-translucent,\n
reflecting the dim swamp light.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: START_COLOR,
        details: START_DETAILS,
        stats: START_STATS,
        description: START_DESCRIPTION
    },
];
