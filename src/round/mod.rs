mod components;
pub mod resources;
mod rollback_systems;
pub mod systems;

/// re-exports of things needed to to use the physics module
pub mod prelude {
    pub use crate::round::{components::*, resources::*, rollback_systems::*, systems::*};
}

// inputs
const INPUT_UP: u8 = 0b00001;
const INPUT_DOWN: u8 = 0b00010;
const INPUT_LEFT: u8 = 0b00100;
const INPUT_RIGHT: u8 = 0b01000;
const INPUT_ACT: u8 = 0b10000;

// animation params
const FRAMES_PER_SPRITE: usize = 10; // TODO: variable frame length per animation and per frame in animation

// physics param
const ATTACKER_SIZE: f32 = 24.;
const MAX_SPEED: f32 = 100.;
pub const JUMP_HEIGHT: f32 = 2. * ATTACKER_SIZE;
pub const JUMP_TIME_TO_PEAK: f32 = 1.;
const DEFENDER_SIZE: f32 = 168.;
const GROUND_LEVEL: f32 = -100.;
const CAKE_SIZE: f32 = 16.;

// controls
const CROSSHAIR_SPEED: f32 = 3.;
const IDLE_THRESH: f32 = 0.01;
const LAND_FRAMES: usize = 3;
const STUN_FRAMES: usize = 60;

// round params
const NUM_ROUNDS: u32 = 2;
const INTERLUDE_LENGTH: u32 = 60;
const ROUND_LENGTH: u32 = 6000;

// fortress pos
const DEF_X_POS: f32 = 250.;

// cake splat params
const MIN_SPLAT: u32 = 1;
const MAX_SPLAT: u32 = 5;
const SPLAT_SPREAD: f32 = 20.;
