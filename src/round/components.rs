use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Attacker {
    pub handle: usize,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Defender {
    pub handle: usize,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct ScreenTimer;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Cake;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Splat;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct NetworkStatsUi;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Crosshair;

// cleaned up after every round
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct RoundEntity;

// cleaned up after the game
#[derive(Component)]
pub struct GameEntity;

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct AttackerControls {
    pub vertical: f32,
    pub horizontal: f32,
}

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct DefenderControls {
    pub vertical: f32,
    pub horizontal: f32,
    pub fire: bool,
}

#[derive(Clone, Copy, Component, Reflect, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub enum FacingDirection {
    Left,
    Right,
}

impl Default for FacingDirection {
    fn default() -> Self {
        Self::Right
    }
}

// the usize counts the number of frames the attacker has been in that state
#[derive(Clone, Copy, Component, Reflect, Debug)]
#[reflect(Component)]
pub enum AttackerState {
    Idle(usize),
    Jump(usize),
    Fall(usize),
    Land(usize),
    Walk(usize),
    Hit(usize),
}

impl AttackerState {
    pub fn can_walk(&self) -> bool {
        match self {
            AttackerState::Hit(..) => false,
            _ => true,
        }
    }

    pub fn is_stunned(&self) -> bool {
        match self {
            AttackerState::Hit(..) => true,
            _ => false,
        }
    }

    pub fn can_clean(&self) -> bool {
        match self {
            AttackerState::Idle(..) | AttackerState::Walk(..) | AttackerState::Land(..) => true,
            _ => false,
        }
    }

    pub fn can_jump(&self) -> bool {
        match self {
            AttackerState::Idle(..) | AttackerState::Walk(..) => true,
            _ => false,
        }
    }

    pub fn get_frame(&self) -> usize {
        match self {
            AttackerState::Idle(f) => *f,
            AttackerState::Jump(f) => *f,
            AttackerState::Fall(f) => *f,
            AttackerState::Land(f) => *f,
            AttackerState::Walk(f) => *f,
            AttackerState::Hit(f) => *f,
        }
    }
}

impl Default for AttackerState {
    fn default() -> Self {
        Self::Idle(0)
    }
}

// the usize counts the number of frames the defender has been in that state
#[derive(Clone, Copy, Component, Reflect, Debug)]
#[reflect(Component)]
pub enum DefenderState {
    Idle(usize),
    Fire(usize),
}

impl Default for DefenderState {
    fn default() -> Self {
        Self::Idle(0)
    }
}

impl DefenderState {
    pub fn get_frame(&self) -> usize {
        match self {
            DefenderState::Idle(f) => *f,
            DefenderState::Fire(f) => *f,
        }
    }
}
