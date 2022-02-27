use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Attacker {
    pub handle: usize,
}

// cleaned up after every round
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct RoundEntity;

// cleaned up after the game
#[derive(Component)]
pub struct GameEntity;

#[derive(Default, Reflect, Component)]
#[reflect(Component)]
pub struct PlatformerControls {
    pub accel: f32,
    pub horizontal: f32,
}

// the u16 counts the number of frames the attacker has been in that state
#[derive(Clone, Copy, Component, Reflect, Debug)]
#[reflect(Component)]
pub enum AttackerState {
    Idle(usize),
    Jump(usize),
    Fall(usize),
    Land(usize),
    Walk(usize),
}

impl AttackerState {
    #[allow(dead_code)]
    pub fn is_grounded(&self) -> bool {
        match self {
            AttackerState::Idle(..) | AttackerState::Land(..) | AttackerState::Walk(..) => true,
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
        }
    }
}

impl Default for AttackerState {
    fn default() -> Self {
        Self::Idle(0)
    }
}
