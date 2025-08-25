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
pub const TEXT_FONT_SIZE: f32 = 28.0;

/// Simulation parameters
pub const MAX_SPEED: f32 = 64.0 * if PREVIEW_MODE { 2.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5 * if PREVIEW_MODE { 0.2 } else { 3.0 };
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 3.0;
pub const IDLE_ENERGY_LOSS: f32 = 2.0;
pub const HUNTING_REACTIVITY: f32 = 10.0;
pub const FLEEING_REACTIVITY: f32 = 10.0;

/// DEBUG
pub const TITLE: &str = "Region: Irr'Hakur desert | Biome: Oasis pond";
pub const VERSION_NAME: &str = "0.0.0 [AQUATIC_CHAOS]";
pub const LAB_NAME: &str = "0";
pub const RUN_IT: &str = "0";
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 20.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const FRAMES_PER_UPDATE: u32 = 180;

/// WORLD
pub const WALLS_COLOR: (f32, f32, f32) = (0.784, 0.631, 0.396);
pub const WATER_COLOR: (f32, f32, f32) = (0.035, 0.231, 0.278);

/// FIRST ELEMENT (latest in the batch)
pub const START_MAX_ENERGY: f32 = 800.0;
pub const START_ENERGY: f32 = 650.0;
pub const START_MAX_SPEED: f32 = 40.0;
pub const START_DETAILS: &str = "Species: Hakursa\n                      <<<\nType: Apex predator";
pub const START_STATS: &str = "Max energy: 800.0\n>>>                      \nMax speed: 40.0";
pub const START_DESCRIPTION: &str =
"The Hakursa is the largest organism in the
Irr'Hakur oasis, a colossal, mass of segmented
muscle and primitive sensory ridges running the
length of its body. The Hakursa patrols the pond
continuously, gliding through the water in slow,
deliberate arcs. When it detects vibrations from
Dunetide swarms or the darting pulses of a Gharlox,
it accelerates in a sudden but sustained pursuit,
sweeping its flexible jaws sideways to engulf its
target whole.
Its presence is constant, every creature in the
oasis adjusts its behavior when the low-frequency
hum of its movement passes through the water.
Desert myths call it \"the living tide\", believing
the pond itself retreats when a Hakursa dies.";
pub const START_COLOR: (f32, f32, f32) = (0.027, 0.027, 0.027);

/// ENTITIES
// Plants
pub const NB_SAHLALGA: i32 = 32;
pub const SAHLALGA_SIZE: f32 = 6.0;
pub const MAX_SAHLALGA_ENERGY: f32 = 60.0;
pub const INITIAL_SAHLALGA_ENERGY: f32 = 40.0;
pub const SAHLALGA_ENERGY_REGEN: f32 = 6.0;
pub const SAHLALGA_COLOR: (f32, f32, f32) = (0.133, 0.800, 0.400);
pub const SAHLALGA_MAX_SPEED: f32 = 0.0;

pub const NB_MIRAJUN: i32 = 16;
pub const MIRAJUN_SIZE: f32 = 8.0;
pub const MAX_MIRAJUN_ENERGY: f32 = 100.0;
pub const INITIAL_MIRAJUN_ENERGY: f32 = 70.0;
pub const MIRAJUN_ENERGY_REGEN: f32 = 7.0;
pub const MIRAJUN_COLOR: (f32, f32, f32) = (0.000, 0.702, 0.533);
pub const MIRAJUN_MAX_SPEED: f32 = 0.0;

// Prey
pub const NB_DUNETIDE: i32 = 8;
pub const DUNETIDE_SIZE: f32 = 12.0;
pub const MAX_DUNETIDE_ENERGY: f32 = 200.0;
pub const INITIAL_DUNETIDE_ENERGY: f32 = 150.0;
pub const DUNETIDE_COLOR: (f32, f32, f32) = (0.741, 0.741, 0.741);
pub const DUNETIDE_MAX_SPEED: f32 = 84.0;

// Predators
pub const NB_GHARLOX: i32 = 4;
pub const GHARLOX_SIZE: f32 = 20.0;
pub const MAX_GHARLOX_ENERGY: f32 = 400.0;
pub const INITIAL_GHARLOX_ENERGY: f32 = 300.0;
pub const GHARLOX_COLOR: (f32, f32, f32) = (1.000, 0.231, 0.188);
pub const GHARLOX_MAX_SPEED: f32 = 64.0;

// Apex predators
pub const NB_HAKURSA: i32 = 2;
pub const HAKURSA_SIZE: f32 = 32.0;
pub const MAX_HAKURSA_ENERGY: f32 = START_MAX_ENERGY;
pub const INITIAL_HAKURSA_ENERGY: f32 = START_ENERGY;
pub const HAKURSA_COLOR: (f32, f32, f32) = START_COLOR;
pub const HAKURSA_MAX_SPEED: f32 = START_MAX_SPEED;

/// HUD BATCHES
pub static BATCHES: &[HudBatch] = &[
    HudBatch {
        title: TITLE,
        sprite_color: SAHLALGA_COLOR,
        details: "Species: Sahlalga\n                      <<<\nType: Plant",
        stats: "Max energy: 60.0\n>>>                      \nMax speed: 0.0",
        description: 
"Sahlalga grows in buoyant mats across the pond’s
surface. Its body is a sheet of interwoven
filaments with many simple light-harvesting plates
(analogous to chloroplasts) arranged in layered
membranes to catch harsh, angled desert light. The
filaments form small gas bladders that keep the
mat near the surface so it receives steady
illumination at midday. Ancient oasis maps show
Sahlalga rings as markers of deep water; elders
say the mats hum in heat. Ecologically it is a
slow but resilient energy accumulator and the
dominant surface producer.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: MIRAJUN_COLOR,
        details: "Species: Mirajun\n                      <<<\nType: Plant",
        stats: "Max energy: 100.0\n>>>                      \nMax speed: 0.0",
        description:
"Mirajun is a shimmering carpet that clings to
shallow stones and the pond floor. Made of densely
packed, crystalline cells, it scatters and refracts
incoming sun so that even diffuse light is usable.
Its thin mucilaginous layer traps silt and
stabilizes the substrate. Mirajun is the fast-
regenerating primary producer of the shallow
margins and the first food for grazing consumers.
Petroglyphs around Irr'Hakur depict hands touching
a shimmering Mirajun patch as a sign of blessing.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: DUNETIDE_COLOR,
        details: "Species: Dunetide\n                      <<<\nType: Prey",
        stats: concat!("Max energy: 200.0\n>>>                      \nMax speed: 84.0"),
        description:
"Dunetide is a compact, colonial grazer that swims
in slow pulses across Mirajun and Sahlalga. Each
animal is a modular cluster of ciliated units with
a simple mouth-like feeding groove that scrapes and
ingests biofilm. It senses chemical cues with
distributed receptor cells and performs rapid jet-
like escapes by coordinated ciliary reversals.
Tribes call them \"glass-paddles\" for the way
sunlight fractures through their bodies when they
flee. Dunetide is the main mobile plant consumer
and shelters in deeper water columns when predators
are near.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: GHARLOX_COLOR,
        details: "Species: Gharlox\n                      <<<\nType: Predator",
        stats: concat!("Max energy: 400.0\n>>>                      \nMax speed: 64.0"),
        description:
"Gharlox is a low-profile, soft-bodied hunter that
patrols reedbeds and submerged stone.
Morphologically simple, it has a tubular, suction
mouth and rows of mechanosensory papillae;
propulsion is by muscular contractions and flicking
flaps rather than fins. Gharlox lies in wait in
shadow, then creates a sudden suction to pull
nearby Dunetide from their grazing patches. Its
nervous system is a compact nerve net giving fast
local reflexes but limited long-range planning, so
it abandons kills when larger scents or vibrations
(Hakursa) approach. Hunters carved the Gharlox
silhouette as a symbol of patience.",
    },
    
    HudBatch {
        title: TITLE,
        sprite_color: START_COLOR,
        details: START_DETAILS,
        stats: START_STATS,
        description: START_DESCRIPTION
    },
];
