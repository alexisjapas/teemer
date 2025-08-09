/// Video parameters
pub const PREVIEW_MODE: bool = cfg!(feature = "preview");
pub const WINDOW_WIDTH: f32 = 720.0;
pub const WINDOW_HEIGHT: f32 = 1280.0;
pub const FRAMERATE: f32 = 30.0;
pub const MAX_DURATION: f32 = 61.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };
pub const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
pub const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// HUD
pub const WALLS_THICKNESS: f32 = 8.0;
pub const ENTITIES_SIZE: f32 = 44.0 * WINDOW_WIDTH / 720.0;
pub const TITLE_FONT_SIZE: f32 = 24.0 * WINDOW_WIDTH / 720.0;
pub const TEXT_FONT_SIZE: f32 = 20.0 * WINDOW_WIDTH / 720.0;

/// Simulation parameters
pub const MAX_SPEED: f32 = 21.0 * WINDOW_WIDTH / 720.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };
pub const MOVEMENT_ENERGY_COST_FACTOR: f32 = 3.0E-5;
pub const ENERGY_TRANSFER_RATE: f32 = 1.0 / 2.0;

pub const NB_PREDATORS: i32 = 4;
pub const PREDATOR_SIZE: f32 = 15.0;
pub const INITIAL_PREDATOR_ENERGY: f32 = 200.0;
pub const MAX_PREDATOR_ENERGY: f32 = 250.0;

pub const NB_PREY: i32 = 22;
pub const PREY_SIZE: f32 = 7.0;
pub const INITIAL_PREY_ENERGY: f32 = 100.0;
pub const MAX_PREY_ENERGY: f32 = 200.0;

pub const NB_PLANTS: i32 = 111;
pub const PLANT_SIZE: f32 = 5.0;
pub const INITIAL_PLANT_ENERGY: f32 = 10.0;
pub const MAX_PLANT_ENERGY: f32 = 50.0;
pub const PLANT_ENERGY_REGEN: f32 = 0.1;

/// DEBUG
pub const DEBUG: bool = true;
pub const DEBUG_FONT_SIZE: f32 = 16.0;
pub const DEBUG_POS_PADDING: f32 = 2.0;
pub const DEBUG_POS_TOP: f32 =
    WINDOW_HEIGHT - DEBUG_FONT_SIZE - WALLS_THICKNESS - DEBUG_POS_PADDING;
pub const DEBUG_POS_LEFT: f32 = WALLS_THICKNESS + DEBUG_POS_PADDING;
