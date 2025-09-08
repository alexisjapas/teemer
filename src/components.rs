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
    Zyralith
}

#[derive(Component, Clone)]
pub struct Hunter {
    pub hunts: Vec<Species>,
    pub detection_range: f32,
    pub current_target: Option<Entity>,
}
impl Hunter {
    pub fn new(hunts: Vec<Species>, detection_range: f32) -> Self {
        Self {
            hunts,
            detection_range,
            current_target: None,
        }
    }
}

#[derive(Component, Clone)]
pub struct Prey {
    pub detection_range: f32,
    pub current_threat: Option<Entity>,
}
impl Prey {
    pub fn new(detection_range: f32) -> Self {
        Self {
            detection_range,
            current_threat: None,
        }
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
    pub title: &'static str,
    pub sprite_color: (f32, f32, f32),
    pub details: &'static str,
    pub stats: &'static str,
    pub description: &'static str,
}

/// DEBUG
#[derive(Component)]
pub struct DEBUGGER;
