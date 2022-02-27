use bevy::prelude::*;

mod components;
pub mod resources;
mod rollback_systems;
pub mod systems;

/// re-exports of things needed to to use the physics module
pub mod prelude {
    pub use crate::round::{components::*, resources::*, rollback_systems::*, systems::*};
}

const INPUT_UP: u8 = 0b00001;
const INPUT_DOWN: u8 = 0b00010;
const INPUT_LEFT: u8 = 0b00100;
const INPUT_RIGHT: u8 = 0b01000;
const INPUT_ACT: u8 = 0b10000;

const GROUND: Color = Color::rgb(0.5, 0.5, 0.5);

const PLAYER_SIZE: f32 = 24.;
const MAX_SPEED: f32 = 100.;
pub const JUMP_HEIGHT: f32 = 2. * PLAYER_SIZE;
pub const JUMP_TIME_TO_PEAK: f32 = 1.;
const GROUND_LEVEL: f32 = -100.;
const NUM_ROUNDS: u32 = 2;
const FRAMES_PER_SPRITE: usize = 10; // TODO: variable frame length per animation and per frame in animation
