use bevy::prelude::*;

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Component)]
pub struct PlatformerControls {
    pub accel: f32,
    pub horizontal: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Grounded(pub bool);
