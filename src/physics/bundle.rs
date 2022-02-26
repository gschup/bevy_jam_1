use bevy::prelude::*;

use super::components::*;

#[derive(Bundle, Default)]
pub struct ParticleBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass,
    pub collider: CircleCollider,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub restitution: Restitution,
    pub aabb: Aabb,
}

impl ParticleBundle {
    // pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
    //     Self {
    //         pos: Pos(pos),
    //         prev_pos: PrevPos(pos - vel * SUB_DT),
    //         vel: Vel(vel),
    //         ..Default::default()
    //     }
    // }
}

#[derive(Bundle, Default)]
pub struct DynamicBoxBundle {
    pub pos: Pos,
    pub prev_pos: PrevPos,
    pub mass: Mass,
    pub collider: BoxCollider,
    pub vel: Vel,
    pub pre_solve_vel: PreSolveVel,
    pub restitution: Restitution,
    pub aabb: Aabb,
}

impl DynamicBoxBundle {
    // pub fn new_with_pos_and_vel(pos: Vec2, vel: Vec2) -> Self {
    //     Self {
    //         pos: Pos(pos),
    //         prev_pos: PrevPos(pos - vel * SUB_DT),
    //         vel: Vel(vel),
    //         ..Default::default()
    //     }
    // }
}

#[derive(Bundle, Default)]
pub struct StaticCircleBundle {
    pub pos: Pos,
    pub collider: CircleCollider,
    pub restitution: Restitution,
}

#[derive(Bundle, Default)]
pub struct StaticBoxBundle {
    pub pos: Pos,
    pub collider: BoxCollider,
    pub restitution: Restitution,
}
