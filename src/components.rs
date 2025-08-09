use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Species {
    Predator,
    Prey,
    Plant,
}

#[derive(Component)]
pub struct Hunter {
    pub hunts: Species,
    pub detection_range: f32,
    pub current_target: Option<Entity>,
}
impl Hunter {
    pub fn new(hunts: Species, detection_range: f32) -> Self {
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
pub struct Consumable;

#[derive(Component, Clone)]
pub struct ActiveMover;

#[derive(Component, Clone)]
pub struct Photosynthesis;

/// DEBUG
#[derive(Component)]
pub struct DEBUGGER;
