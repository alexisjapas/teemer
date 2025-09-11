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
pub const TITLE: &str = "Region: Onomora | Biome: Thermal spring basin";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "1";
pub const RUN_IT: &str = "34";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// WORLD
pub const WALLS_COLOR: (f32, f32, f32) = (0.623, 0.184, 0.141);
pub const WATER_COLOR: (f32, f32, f32) = (0.780, 0.694, 0.565);

/// FIRST ELEMENT (latest in the batch)
pub const START_MAX_ENERGY: f32 = 700.0;
pub const START_ENERGY: f32 = 500.0;
pub const START_MAX_SPEED: f32 = 50.0;
pub const START_DETAILS: &str = "Species: Onytheron\n                      <<<\nType: Apex predator";
pub const START_STATS: &str = "Max energy: 700.0\n>>>                      \nMax speed: 50.0";
pub const START_DESCRIPTION: &str =
"Apex predator inhabiting the hottest, chemically\n
extreme zones near vent cores. Onytheron consumes\n
Pyrralis and Cindralis, ignoring Omyra. Its\n
colossal indigo-black body blends seamlessly with\n
mineral plumes, silently defining the basin's food\n
chain.";
pub const START_COLOR: (f32, f32, f32) = (0.090, 0.113, 0.224);

/// ENTITIES
// Plants
pub const NB_LYRVANE: i32 = 768;
pub const LYRVANE_SIZE: f32 = 6.0;
pub const MAX_LYRVANE_ENERGY: f32 = 180.0;
pub const INITIAL_LYRVANE_ENERGY: f32 = 100.0;
pub const LYRVANE_ENERGY_REGEN: f32 = 17.0;
pub const LYRVANE_COLOR: (f32, f32, f32) = (0.894, 0.471, 0.086);
pub const LYRVANE_MAX_SPEED: f32 = 0.0;

// Prey
pub const NB_OMYRA: i32 = 96;
pub const OMYRA_SIZE: f32 = 10.0;
pub const MAX_OMYRA_ENERGY: f32 = 250.0;
pub const INITIAL_OMYRA_ENERGY: f32 = 150.0;
pub const OMYRA_COLOR: (f32, f32, f32) = (0.961, 0.824, 0.247);
pub const OMYRA_MAX_SPEED: f32 = 100.0;

// Predators p1
pub const NB_CINDRALYS: i32 = 48;
pub const CINDRALYS_SIZE: f32 = 14.0;
pub const MAX_CINDRALYS_ENERGY: f32 = 400.0;
pub const INITIAL_CINDRALYS_ENERGY: f32 = 300.0;
pub const CINDRALYS_COLOR: (f32, f32, f32) = (0.780, 0.180, 0.149);
pub const CINDRALYS_MAX_SPEED: f32 = 80.0;

// Predators p2
pub const NB_PYRRALIS: i32 = 24;
pub const PYRRALIS_SIZE: f32 = 20.0;
pub const MAX_PYRRALIS_ENERGY: f32 = 550.0;
pub const INITIAL_PYRRALIS_ENERGY: f32 = 400.0;
pub const PYRRALIS_COLOR: (f32, f32, f32) = (0.396, 0.318, 0.647);
pub const PYRRALIS_MAX_SPEED: f32 = 70.0;

// Apex predators
pub const NB_ONYTHERON: i32 = 12;
pub const ONYTHERON_SIZE: f32 = 26.0;
pub const MAX_ONYTHERON_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_ONYTHERON_ENERGY: f32 = START_ENERGY;
pub const ONYTHERON_COLOR: (f32, f32, f32) = START_COLOR;
pub const ONYTHERON_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[    
    HudBatch {
        title: TITLE,
        sprite_color: LYRVANE_COLOR,
        details: "Species: Lyrvane\n                      <<<\nType: Plant",
        stats: "Max energy: 180.0\n>>>                      \nMax speed: 0.0",
        description:
"Heat-adapted phototroph forming dense, amber-\n
orange mats on mineral surfaces near vents. Lyrvane\n
absorbs both sunlight and geothermal energy,\n
stabilizing sediments and providing the primary\n
energy source in the basin. Its glimmering\n
filaments create micro-habitats for smaller\n
organisms.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: OMYRA_COLOR,
        details: "Species: Omyra\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 250.0\n>>>                      \nMax speed: 100.0"),
        description:
"A nimble, golden-hued grazer with crystalline\n
armor resistant to scalding currents. Omyra scrapes\n
Lyrvane mats for nutrients and often moves in\n
cohesive swarms, generating shimmering ripples\n
across the thermal waters.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: CINDRALYS_COLOR,
        details: "Species: Cindralys\n                      <<<\nType: Predator",
        stats: concat!("Max energy: 400.0\n>>>                      \nMax speed: 80.0"),
        description:
"Swift, fiery-red predator hunting Omyra. Cindralys\n
darts explosively through the hot currents,\n
impaling swarms before retreating to cooler\n
microzones. Its segmented body and heat-resistant\n
exoskeleton make it a dominant hunter in turbulent\n
thermal waters.",
    },
    
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
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: START_COLOR,
        details: START_DETAILS,
        stats: START_STATS,
        description: START_DESCRIPTION
    },
];
