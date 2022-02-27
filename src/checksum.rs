use bevy::prelude::*;
use bevy_ggrs::Rollback;

use crate::{
    physics::{components::Vel, prelude::Pos},
    round::prelude::Attacker,
};

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct Checksum {
    value: u16,
}

pub fn checksum_attackers(
    mut query: Query<(&Transform, &Vel, &Pos, &mut Checksum), (With<Attacker>, With<Rollback>)>,
) {
    for (t, v, p, mut checksum) in query.iter_mut() {
        let translation = t.translation;
        // println!("Attacker: translation: {translation:?}, vel: {v:?}, pos: {p:?}",);
        let mut bytes = Vec::with_capacity(20);
        bytes.extend_from_slice(&translation.x.to_le_bytes());
        bytes.extend_from_slice(&translation.y.to_le_bytes());
        bytes.extend_from_slice(&translation.z.to_le_bytes()); // this z will probably never matter, but removing it probably also will not matter...

        // TODO: checksum more physics types here
        bytes.extend_from_slice(&v.0.x.to_le_bytes());
        bytes.extend_from_slice(&v.0.y.to_le_bytes());

        bytes.extend_from_slice(&p.0.x.to_le_bytes());
        bytes.extend_from_slice(&p.0.y.to_le_bytes());

        // naive checksum implementation
        checksum.value = fletcher16(&bytes);
    }
}

/// Computes the fletcher16 checksum, copied from wikipedia: <https://en.wikipedia.org/wiki/Fletcher%27s_checksum>
fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for byte in data {
        sum1 = (sum1 + *byte as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}
