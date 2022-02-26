use bevy::prelude::*;

use super::PIXELS_PER_METER;

#[derive(Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        // For real-world gravity,
        // we should probably tweak this, though. Maybe even have per object gravity?
        // Self(Vec2::new(0., -9.81 * PIXELS_PER_METER))
        Self(Vec2::new(0., -9.81 * PIXELS_PER_METER * 4.0))
        // TODO: maybe derive from hangtime as suggested in: https://www.youtube.com/watch?v=hG9SzQxaCm8
    }
}

#[derive(Default, Debug)]
pub struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Default, Debug)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Default, Debug)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);
