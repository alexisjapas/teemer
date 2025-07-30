/// Video parameters
pub const PREVIEW_MODE: bool = cfg!(feature = "preview");
pub const WINDOW_WIDTH: f32 = 1080.0;
pub const WINDOW_HEIGHT: f32 = 1920.0;
pub const FRAMERATE: f32 = 30.0;
pub const MAX_DURATION: f32 = 61.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };
pub const MAX_FRAMES_TO_CAPTURE: u32 = MAX_DURATION as u32 * FRAMERATE as u32;
pub const FIXED_TIME_STEP: f32 = 1.0 / FRAMERATE;

/// HUD
pub const WALLS_THICKNESS: f32 = 8.0;
pub const ENTITIES_SIZE: f32 = 66.0;
pub const TITLE_FONT_SIZE: f32 = 36.0;
pub const TEXT_FONT_SIZE: f32 = 30.0;

/// Simulation parameters
pub const NB_PREDATORS: i32 = 5;
pub const NB_PREY: i32 = 111;
pub const NB_PLANTS: i32 = 222;
pub const PREDATOR_SIZE: f32 = 11.0;
pub const PREY_SIZE: f32 = 7.0;
pub const PLANT_SIZE: f32 = 5.0;
pub const MAX_SPEED: f32 = 32.0 * if PREVIEW_MODE { 4.0 } else { 1.0 };
