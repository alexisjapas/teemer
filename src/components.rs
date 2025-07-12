use bevy::prelude::*;

#[derive(Component, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Species {
    Chicken,
    Fox,
    Snake,
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
            detection_range, current_threat: None
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