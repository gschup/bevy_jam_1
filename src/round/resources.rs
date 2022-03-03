use bevy::{prelude::*, utils::HashMap};
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

#[derive(Copy, Clone, Reflect, Hash, Component)]
#[reflect(Hash)]
pub enum RoundState {
    InterludeStart,
    Interlude,
    InterludeEnd,
    RoundStart,
    Round,
    RoundEnd,
}

#[derive(Debug, Default, Clone, Reflect, Component)]
pub struct RoundData {
    pub cur_round: u32,               // the current round
    pub results: HashMap<u32, usize>, // key: round, value: remaining splats
}

impl RoundData {
    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for (k, v) in self.results.iter() {
            str.push_str(&format!("Janitor {}: {} splats left\n", k + 1, v));
        }
        let winner = self
            .results
            .iter()
            .min_by(|a, b| a.1.cmp(&b.1))
            .map(|(k, _)| *k)
            .expect("No entries in results.");

        str.push_str(&format!("\nJanitor {} wins!", winner + 1));
        str
    }
}

impl Default for RoundState {
    fn default() -> Self {
        Self::InterludeStart
    }
}
