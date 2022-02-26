use bevy::prelude::*;

#[derive(Debug)]
pub struct Gravity(pub Vec2);

impl Default for Gravity {
    fn default() -> Self {
        Self(Vec2::new(0., -9.81))
    }
}

#[derive(Default, Debug)]
pub struct CollisionPairs(pub Vec<(Entity, Entity)>);

#[derive(Default, Debug)]
pub struct Contacts(pub Vec<(Entity, Entity, Vec2)>);

#[derive(Default, Debug)]
pub struct StaticContacts(pub Vec<(Entity, Entity, Vec2)>);
