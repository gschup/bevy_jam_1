use bevy::prelude::*;
use derive_more::From;

// todo: register all of these as rollback components

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct Aabb {
    pub(crate) min: Vec2,
    pub(crate) max: Vec2,
}

impl Aabb {
    pub fn intersects(&self, other: &Self) -> bool {
        self.max.x >= other.min.x
            && self.max.y >= other.min.y
            && self.min.x <= other.max.x
            && self.min.y <= other.max.y
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, From)]
#[reflect(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self { radius: 0.5 }
    }
}

#[derive(Component, Reflect, Debug, Clone, Copy, From)]
#[reflect(Component)]
pub struct BoxCollider {
    pub size: Vec2,
}

impl BoxCollider {
    // pub fn inertia_inv_from_mass_inv(&self, mass_inv: f32) -> f32 {
    //     12. * mass_inv / self.size.length_squared()
    // }
}

impl Default for BoxCollider {
    fn default() -> Self {
        Self { size: Vec2::ONE }
    }
}

#[derive(Component, Reflect, Debug, Default, Clone, Copy, From)]
#[reflect(Component)]
pub struct Pos(pub Vec2);

#[derive(Component, Reflect, Debug, Default, Clone, Copy, From)]
#[reflect(Component)]
pub struct PrevPos(pub Vec2);

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct Vel(pub(crate) Vec2);

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
pub struct PreSolveVel(pub(crate) Vec2);

#[derive(Component, Reflect, Debug, Clone, Copy, From)]
#[reflect(Component)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(1.)
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Restitution(pub f32);

impl Default for Restitution {
    fn default() -> Self {
        Self(0.) // no bounce, could just derive...
    }
}
