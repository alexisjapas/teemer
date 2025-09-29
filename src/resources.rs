use bevy::prelude::*;

/// Resources
#[derive(Resource)]
pub struct SimulationMetadata {
    pub path_dir: String,
    pub name: String,
}
