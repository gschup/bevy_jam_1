use bevy::prelude::*;

use crate::round::{JUMP_HEIGHT, JUMP_TIME_TO_PEAK};

use super::PIXELS_PER_METER;

#[derive(Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        // For real-world gravity,
        // we should probably tweak this, though. Maybe even have per object gravity?
        // Self(Vec2::new(0., -9.81 * PIXELS_PER_METER))
        let grav = (-2. * JUMP_HEIGHT) / JUMP_TIME_TO_PEAK; // derived as suggested in: https://www.youtube.com/watch?v=hG9SzQxaCm8
        Self(Vec2::new(0., grav * PIXELS_PER_METER))
    }
}

#[derive(Component, Reflect, Default, Debug, Hash)]
#[reflect(Hash)]
pub struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Component, Reflect, Default, Debug)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Component, Reflect, Default, Debug)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);
