use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Attacker {
    pub handle: usize,
}

// cleaned up after every round
#[derive(Component)]
pub struct RoundEntity;

// cleaned up after the game
#[derive(Component)]
pub struct GameEntity;

#[derive(Default, Reflect, Component)]
pub struct PlatformerControls {
    pub accel: f32,
    pub horizontal: f32,
}

// the u16 counts the number of frames the attacker has been in that state
#[derive(Clone, Copy, Component, Reflect, Debug, PartialEq)]
#[reflect(Component)]
#[reflect_value(PartialEq)]
pub enum AttackerState {
    Idle(u16),
    Jump(u16),
    Fall(u16),
    Land(u16),
    Walk(u16),
}

impl AttackerState {
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
}

impl Default for AttackerState {
    fn default() -> Self {
        Self::Idle(0)
    }
}
