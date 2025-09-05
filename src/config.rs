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
pub const TEXT_FONT_SIZE: f32 = 38.0;

/// Simulation parameters
pub const MAX_SPEED: f32 = 64.0 * if PREVIEW_MODE { 2.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 3.0;
pub const HUNTING_REACTIVITY: f32 = 10.0;
pub const FLEEING_REACTIVITY: f32 = 10.0;

/// DEBUG
pub const TITLE: &str = "Region: Aganandor mountains | Biome: Montain lake";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "1";
pub const RUN_IT: &str = "13";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// WORLD
pub const WALLS_COLOR: (f32, f32, f32) = (0.88, 0.92, 0.95);
pub const WATER_COLOR: (f32, f32, f32) = (0.07, 0.24, 0.38);

/// FIRST ELEMENT (latest in the batch)
pub const START_MAX_ENERGY: f32 = 1000.0;
pub const START_ENERGY: f32 = 600.0;
pub const START_MAX_SPEED: f32 = 50.0;
pub const START_DETAILS: &str = "Species: Myrrkul\n                      <<<\nType: Apex predator";
pub const START_STATS: &str = "Max energy: 1000.0\n>>>                      \nMax speed: 50.0";
pub const START_DESCRIPTION: &str =
"An immense, slow shadow that
rarely surfaces. Legends tell of
Myrrkul as a slumbering sentinel,
its gaze alone enough to summon
avalanches when the balance is
disturbed.";
pub const START_COLOR: (f32, f32, f32) = (0.05, 0.08, 0.15);

/// ENTITIES
// Plants
pub const NB_LODRIL: i32 = 1024;
pub const LODRIL_SIZE: f32 = 6.0;
pub const MAX_LODRIL_ENERGY: f32 = 150.0;
pub const INITIAL_LODRIL_ENERGY: f32 = 100.0;
pub const LODRIL_ENERGY_REGEN: f32 = 11.0;
pub const LODRIL_COLOR: (f32, f32, f32) = (0.2, 0.65, 0.3);
pub const LODRIL_MAX_SPEED: f32 = 0.0;

// Prey
pub const NB_VANYR: i32 = 128;
pub const VANYR_SIZE: f32 = 12.0;
pub const MAX_VANYR_ENERGY: f32 = 300.0;
pub const INITIAL_VANYR_ENERGY: f32 = 200.0;
pub const VANYR_COLOR: (f32, f32, f32) = (0.65, 0.85, 0.90);
pub const VANYR_MAX_SPEED: f32 = 100.0;

// Predators
pub const NB_THALVYRN: i32 = 32;
pub const THALVYRN_SIZE: f32 = 18.0;
pub const MAX_THALVYRN_ENERGY: f32 = 400.0;
pub const INITIAL_THALVYRN_ENERGY: f32 = 300.0;
pub const THALVYRN_COLOR: (f32, f32, f32) = (0.8, 0.2, 0.2);
pub const THALVYRN_MAX_SPEED: f32 = 70.0;

// Apex predators
pub const NB_MYRRKUL: i32 = 8;
pub const MYRRKUL_SIZE: f32 = 30.0;
pub const MAX_MYRRKUL_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_MYRRKUL_ENERGY: f32 = START_ENERGY;
pub const MYRRKUL_COLOR: (f32, f32, f32) = START_COLOR;
pub const MYRRKUL_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[    
    HudBatch {
        title: TITLE,
        sprite_color: LODRIL_COLOR,
        details: "Species: Lodril\n                      <<<\nType: Plant",
        stats: "Max energy: 100.0\n>>>                      \nMax speed: 0.0",
        description:
"Delicate glowing threads swaying
on the rocky floor, said to mirror
the auroras that crown the mountain
nights. Pilgrims whisper that the
lake \"breathes\" through them.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: VANYR_COLOR,
        details: "Species: Vanyr\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 300.0\n>>>                      \nMax speed: 100.0"),
        description:
"Streamlined grazers gliding in
coordinated arcs. The old texts of
Aganandor claim their shimmering
formations sketch divine runes upon
the lake's surface.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: THALVYRN_COLOR,
        details: "Species: Thalvyrn\n                      <<<\nType: Predator",
        stats: concat!("Max energy: 400.0\n>>>                      \nMax speed: 70.0"),
        description:
"Swift, spectral hunters, their
translucent bodies flashing red
when they strike. Local myths call
them the \"blades of the mountain\",
born from ancient sacrifices in the
high passes.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: START_COLOR,
        details: START_DETAILS,
        stats: START_STATS,
        description: START_DESCRIPTION
    },
];
