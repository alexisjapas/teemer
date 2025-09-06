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
pub const TITLE: &str = "Region: Erezast lands | Biome: Salt marsh";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "1";
pub const RUN_IT: &str = "20";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// WORLD
pub const WALLS_COLOR: (f32, f32, f32) = (0.12, 0.12, 0.12);
pub const WATER_COLOR: (f32, f32, f32) = (0.847, 0.886, 0.882);

/// FIRST ELEMENT (latest in the batch)
pub const START_MAX_ENERGY: f32 = 900.0;
pub const START_ENERGY: f32 = 600.0;
pub const START_MAX_SPEED: f32 = 40.0;
pub const START_DETAILS: &str = "Species: Vorqualis\n                      <<<\nType: Apex predator";
pub const START_STATS: &str = "Max energy: 900.0\n>>>                      \nMax speed: 40.0";
pub const START_DESCRIPTION: &str =
"The silent apex of the Ezerast marsh. Vorqualis\n
drifts slowly, its dark indigo mass almost blending\n
into shadowy depths. It engulfs Chalyth predator\n
whole, but also opportunistically devours Brisqal\n
when swarms cross its path. Despite its lethargic\n
movement, it is nearly impossible to escape once\n
drawn into its engulfing folds. Its presence\n
defines the top of the food chain in this biome.";
pub const START_COLOR: (f32, f32, f32) = (0.278, 0.243, 0.478);

/// ENTITIES
// Plants
pub const NB_QYRSEL: i32 = 512;
pub const QYRSEL_SIZE: f32 = 6.0;
pub const MAX_QYRSEL_ENERGY: f32 = 100.0;
pub const INITIAL_QYRSEL_ENERGY: f32 = 70.0;
pub const QYRSEL_ENERGY_REGEN: f32 = 9.0;
pub const QYRSEL_COLOR: (f32, f32, f32) = (0.090, 0.682, 0.220);
pub const QYRSEL_MAX_SPEED: f32 = 0.0;

pub const NB_OZYRAE: i32 = 512;
pub const OZYRAE_SIZE: f32 = 8.0;
pub const MAX_OZYRAE_ENERGY: f32 = 200.0;
pub const INITIAL_OZYRAE_ENERGY: f32 = 150.0;
pub const OZYRAE_ENERGY_REGEN: f32 = 15.0;
pub const OZYRAE_COLOR: (f32, f32, f32) = (0.090, 0.800, 0.650);
pub const OZYRAE_MAX_SPEED: f32 = 0.0;

// Prey
pub const NB_VEYTRIS: i32 = 16;
pub const VEYTRIS_SIZE: f32 = 10.0;
pub const MAX_VEYTRIS_ENERGY: f32 = 300.0;
pub const INITIAL_VEYTRIS_ENERGY: f32 = 200.0;
pub const VEYTRIS_COLOR: (f32, f32, f32) = (1.000, 0.650, 0.000);
pub const VEYTRIS_MAX_SPEED: f32 = 60.0;

pub const NB_BRISQAL: i32 = 16;
pub const BRISQAL_SIZE: f32 = 12.0;
pub const MAX_BRISQAL_ENERGY: f32 = 450.0;
pub const INITIAL_BRISQAL_ENERGY: f32 = 350.0;
pub const BRISQAL_COLOR: (f32, f32, f32) = (1.000, 0.250, 0.250);
pub const BRISQAL_MAX_SPEED: f32 = 100.0;

// Predators
pub const NB_CHALYTH: i32 = 8;
pub const CHALYTH_SIZE: f32 = 18.0;
pub const MAX_CHALYTH_ENERGY: f32 = 700.0;
pub const INITIAL_CHALYTH_ENERGY: f32 = 500.0;
pub const CHALYTH_COLOR: (f32, f32, f32) = (0.780, 0.000, 0.180);
pub const CHALYTH_MAX_SPEED: f32 = 70.0;

// Apex predators
pub const NB_VORQUALIS: i32 = 4;
pub const VORQUALIS_SIZE: f32 = 26.0;
pub const MAX_VORQUALIS_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_VORQUALIS_ENERGY: f32 = START_ENERGY;
pub const VORQUALIS_COLOR: (f32, f32, f32) = START_COLOR;
pub const VORQUALIS_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[    
    HudBatch {
        title: TITLE,
        sprite_color: QYRSEL_COLOR,
        details: "Species: Qyrsel\n                      <<<\nType: Plant",
        stats: "Max energy: 100.0\n>>>                      \nMax speed: 0.0",
        description:
"A filamentous algal organism that binds together\n
into sprawling mats across saline sediments. Its\n
thin green threads absorb light efficiently,\n
creating micro-forests that shelter other life.\n
Qyrsel thrives in high-salinity zones where few\n
other organisms can survive.",
    },
    HudBatch {
        title: TITLE,
        sprite_color: OZYRAE_COLOR,
        details: "Species: Ozyrae\n                      <<<\nType: Plant",
        stats: "Max energy: 200.0\n>>>                      \nMax speed: 0.0",
        description:
"A free-floating phytoplankton with translucent\n
crystalline shells that shimmer faintly under\n
light. Ozyrae forms loose drifting colonies,\n
providing both food and micro-habitats. Its saline-\n
resistant walls protect it from osmotic collapse in\n
harsh brine.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: VEYTRIS_COLOR,
        details: "Species: Veytris\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 300.0\n>>>                      \nMax speed: 60.0"),
        description:
"A swift grazer resembling a translucent crustacean\n
shard. Veytris filter-feeds directly from Qyrsel\n
mats, scraping the filaments into digestible\n
fragments. Often traveling in swarms, it creates\n
rippling waves of movement through the brackish\n
water.",
    },
    HudBatch {
        title: TITLE,
        sprite_color: BRISQAL_COLOR,
        details: "Species: Brisqal\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 450.0\n>>>                      \nMax speed: 100.0"),
        description:
"An amoeboid organism with a shifting, coral-hued\n
body. Brisqal is a cunning grazer of Ozyrae\n
colonies, using its pseudopodia to engulf cells one\n
by one. Though small, its speed and adaptability\n
make it difficult for larger hunters to capture.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: CHALYTH_COLOR,
        details: "Species: Chalyth\n                      <<<\nType: Predator",
        stats: concat!("Max energy: 700.0\n>>>                      \nMax speed: 70.0"),
        description:
"A formidable predator armed with rigid spines and\n
pulsating vacuoles. Chalyth stalks Veytris swarms\n
and Brisqal clusters, capturing them with sudden\n
suction bursts. Its reddish hue signals aggression\n
and dominance in the salt marsh microcosm.\n
Solitary, but feared by all lesser creatures.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: START_COLOR,
        details: START_DETAILS,
        stats: START_STATS,
        description: START_DESCRIPTION
    },
];
