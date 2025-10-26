use crate::config::RuntimeConfig;
use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Species {
    // Plants
    Sahlalga,
    Mirajun,
    Lodril,
    // Prey
    Dunetide,
    Vanyr,
    // Predators
    Gharlox,
    Thalvyrn,
    // Apex predators
    Hakursa,
    Myrrkul,
    // Ezerast - Salt marsh
    Qyrsel,
    Ozyrae,
    Veytris,
    Brisqal,
    Chalyth,
    Vorqualis,
    // Irr'Umar - Swamp
    Vyrmosa,
    Qirval,
    Lorynth,
    Dravym,
    Zyralith,
    // Onomora - Thermal spring basin
    Lyrvane,
    Omyra,
    Cindralys,
    Pyrralis,
    Onytheron,
}

impl Species {
    pub fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            // Plants
            "sahlalga" => Some(Species::Sahlalga),
            "mirajun" => Some(Species::Mirajun),
            "lodril" => Some(Species::Lodril),
            // Prey
            "dunetide" => Some(Species::Dunetide),
            "vanyr" => Some(Species::Vanyr),
            // Predators
            "gharlox" => Some(Species::Gharlox),
            "thalvyrn" => Some(Species::Thalvyrn),
            // Apex predators
            "hakursa" => Some(Species::Hakursa),
            "myrrkul" => Some(Species::Myrrkul),
            // Ezerast - Salt marsh
            "qyrsel" => Some(Species::Qyrsel),
            "ozyrae" => Some(Species::Ozyrae),
            "veytris" => Some(Species::Veytris),
            "brisqal" => Some(Species::Brisqal),
            "chalyth" => Some(Species::Chalyth),
            "vorqualis" => Some(Species::Vorqualis),
            // Irr'Umar - Swamp
            "vyrmosa" => Some(Species::Vyrmosa),
            "qirval" => Some(Species::Qirval),
            "lorynth" => Some(Species::Lorynth),
            "dravym" => Some(Species::Dravym),
            "zyralith" => Some(Species::Zyralith),
            // Onomora - Thermal spring basin
            "lyrvane" => Some(Species::Lyrvane),
            "omyra" => Some(Species::Omyra),
            "cindralys" => Some(Species::Cindralys),
            "pyrralis" => Some(Species::Pyrralis),
            "onytheron" => Some(Species::Onytheron),
            _ => None,
        }
    }
}

/// Actions
#[derive(Component, Clone)]
pub struct Vision {
    pub detection_range: f32,
    pub nb_rays: u32,
    pub field_of_view: f32,
}
impl Vision {
    pub fn new(detection_range: f32, nb_rays: u32, field_of_view: f32) -> Self {
        Self {
            detection_range,
            nb_rays,
            field_of_view,
        }
    }
}

#[derive(Component, Default)]
pub struct VisionResults {
    pub rays: Vec<RayResult>,
}

#[derive(Clone)]
pub struct RayResult {
    pub origin: Vec2,
    pub direction: Vec2,
    pub max_distance: f32,
    pub hit: Option<RayHitInfo>,
}

#[derive(Clone)]
pub struct RayHitInfo {
    pub entity: Entity,
    pub distance: f32,
    pub point: Vec2,
}

#[derive(Component, Default)]
pub struct MovementIntent {
    pub desired_direction: Vec2,
    pub desired_force: Vec2,
}

/// Traits
#[derive(Component, Clone)]
pub struct Hunter {
    pub hunts: Vec<Species>,
}
impl Hunter {
    pub fn new(hunts: Vec<Species>) -> Self {
        Self { hunts }
    }
}

#[derive(Component, Clone)]
pub struct Speed(pub f32);
impl Speed {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Clone)]
pub struct Energy {
    pub current: f32,
    pub max: f32,
}
impl Energy {
    pub fn new(initial: f32, max: f32) -> Self {
        Self {
            current: initial,
            max: max,
        }
    }

    pub fn value(&self) -> f32 {
        self.current
    }

    pub fn gain(&mut self, amount: f32) {
        self.current = (self.current + amount).min(self.max);
    }

    pub fn lose(&mut self, amount: f32) {
        self.current -= amount;
    }
}

#[derive(Component, Clone)]
pub struct Size(f32);
impl Size {
    pub fn new(value: f32) -> Self {
        Self(value)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

#[derive(Component, Clone)]
pub struct EntityColor(Color);
impl EntityColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Color::linear_rgb(r, g, b))
    }

    pub fn value(&self) -> Color {
        self.0
    }
}

#[derive(Component, Clone)]
pub struct Consumable;

#[derive(Component, Clone)]
pub struct ActiveMover;

#[derive(Component, Clone)]
pub struct Photosynthesis(f32);
impl Photosynthesis {
    pub fn new(regen: f32) -> Self {
        Self(regen)
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}

/// HUD
#[derive(Component)]
pub struct HudTitle;

#[derive(Component)]
pub struct HudSprite;

#[derive(Component)]
pub struct HudStats;

#[derive(Component)]
pub struct HudDescription;

#[derive(Resource)]
pub struct HudEntities {
    pub title: Entity,
    pub sprite: Entity,
    pub details: Entity,
    pub stats: Entity,
    pub description: Entity,
}

#[derive(Resource)]
pub struct HudBatches {
    pub index: usize,
    pub batches: Vec<HudBatch>,
}

#[derive(Clone)]
pub struct HudBatch {
    pub title: String,
    pub sprite_color: (f32, f32, f32),
    pub details: String,
    pub stats: String,
    pub description: String,
}

/// Configuration
#[derive(Resource)]
pub struct GameConfig {
    pub runtime: RuntimeConfig,
}

/// DEBUG
#[derive(Component)]
pub struct DEBUGGER;

/// Rendering
#[derive(Component)]
pub struct RaycastVisualization;

#[derive(Component)]
pub struct HitPointVisualization;
