use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

#[derive(Copy, Clone, Reflect, Hash, Component, PartialEq)]
#[reflect_value(PartialEq)]
//#[reflect(Hash)]
pub enum RoundState {
    InterludeStart,
    Interlude,
    InterludeEnd,
    RoundStart,
    Round,
    RoundEnd,
}

#[derive(Debug, Default, Copy, Clone, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct RoundData {
    pub cur_round: u32, // the current round
}

impl Default for RoundState {
    fn default() -> Self {
        Self::InterludeStart
    }
}
