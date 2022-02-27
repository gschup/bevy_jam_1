use crate::physics::contact;
use crate::physics::contact::Contact;
use crate::physics::utils::QueryExt;
use crate::physics::SUB_DT;

use super::components::*;
use super::resources::*;
use super::COLLISION_PAIR_VEL_MARGIN_FACTOR;
use bevy::prelude::*;

pub fn update_aabb_ball(mut query: Query<(&mut Aabb, &Pos, &Vel, &CircleCollider)>) {
    for (mut aabb, pos, vel, circle) in query.iter_mut() {
        let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
        let half_extents = Vec2::splat(circle.radius + margin);
        aabb.min = pos.0 - half_extents;
        aabb.max = pos.0 + half_extents;
    }
}

pub fn update_aabb_box(mut query: Query<(&mut Aabb, &Pos, &Vel, &BoxCollider)>) {
    for (mut aabb, pos, vel, r#box) in query.iter_mut() {
        let margin = COLLISION_PAIR_VEL_MARGIN_FACTOR * vel.0.length();
        let half_extents = r#box.size / 2. + Vec2::splat(margin);
        aabb.min = pos.0 - half_extents;
        aabb.max = pos.0 + half_extents;
    }
}

pub fn collect_collision_pairs(
    query: Query<(Entity, &Aabb)>,
    mut collision_pairs: ResMut<CollisionPairs>,
) {
    debug!("collect_collision_pairs");
    collision_pairs.0.clear();

    let mut iter = query.iter_combinations();
    while let Some([(entity_a, aabb_a), (entity_b, aabb_b)]) = iter.fetch_next() {
        if aabb_a.intersects(aabb_b) {
            collision_pairs.0.push((entity_a, entity_b));
        }
    }
}

pub fn integrate(
    mut query: Query<(&mut Pos, &mut PrevPos, &mut Vel, &mut PreSolveVel, &Mass)>,
    gravity: Res<Gravity>,
) {
    debug!("  integrate");
    for (mut pos, mut prev_pos, mut vel, mut pre_solve_vel, mass) in query.iter_mut() {
        prev_pos.0 = pos.0;

        let gravitation_force = mass.0 * gravity.0;
        let external_forces = gravitation_force;
        vel.0 += SUB_DT * external_forces / mass.0;
        pos.0 += SUB_DT * vel.0;
        pre_solve_vel.0 = vel.0;
    }
}

pub fn clear_contacts(mut contacts: ResMut<Contacts>, mut static_contacts: ResMut<StaticContacts>) {
    debug!("  clear_contacts");
    contacts.0.clear();
    static_contacts.0.clear();
}

pub fn solve_pos_ball_ball(
    mut query: Query<(&mut Pos, &CircleCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
    collision_pairs: Res<CollisionPairs>,
) {
    debug!("  solve_pos");
    for (entity_a, entity_b) in collision_pairs.0.iter().cloned() {
        if let Ok(((mut pos_a, circle_a, mass_a), (mut pos_b, circle_b, mass_b))) =
            query.get_pair_mut(entity_a, entity_b)
        {
            if let Some(Contact {
                normal,
                penetration,
            }) = contact::ball_ball(pos_a.0, circle_a.radius, pos_b.0, circle_b.radius)
            {
                constrain_body_positions(
                    &mut pos_a,
                    &mut pos_b,
                    mass_a,
                    mass_b,
                    normal,
                    penetration,
                );
                contacts.0.push((entity_a, entity_b, normal));
            }
        }
    }
}

pub fn solve_pos_box_box(
    mut query: Query<(&mut Pos, &BoxCollider, &Mass)>,
    mut contacts: ResMut<Contacts>,
    collision_pairs: Res<CollisionPairs>,
) {
    for (entity_a, entity_b) in collision_pairs.0.iter().cloned() {
        if let Ok(((mut pos_a, box_a, mass_a), (mut pos_b, box_b, mass_b))) =
            query.get_pair_mut(entity_a, entity_b)
        {
            if let Some(Contact {
                normal,
                penetration,
            }) = contact::box_box(pos_a.0, box_a.size, pos_b.0, box_b.size)
            {
                constrain_body_positions(
                    &mut pos_a,
                    &mut pos_b,
                    mass_a,
                    mass_b,
                    normal,
                    penetration,
                );
                contacts.0.push((entity_a, entity_b, normal));
            }
        }
    }
}

pub fn solve_pos_static_ball_ball(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &CircleCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, circle_b) in statics.iter() {
            if let Some(Contact {
                normal,
                penetration,
            }) = contact::ball_ball(pos_a.0, circle_a.radius, pos_b.0, circle_b.radius)
            {
                constrain_body_position(&mut pos_a, normal, penetration);
                contacts.0.push((entity_a, entity_b, normal));
            }
        }
    }
}

pub fn solve_pos_static_box_ball(
    mut dynamics: Query<(Entity, &mut Pos, &CircleCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, circle_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, box_b) in statics.iter() {
            if let Some(Contact {
                normal,
                penetration,
            }) = contact::ball_box(pos_a.0, circle_a.radius, pos_b.0, box_b.size)
            {
                constrain_body_position(&mut pos_a, normal, penetration);
                contacts.0.push((entity_a, entity_b, normal));
            }
        }
    }
}

pub fn solve_pos_static_box_box(
    mut dynamics: Query<(Entity, &mut Pos, &BoxCollider), With<Mass>>,
    statics: Query<(Entity, &Pos, &BoxCollider), Without<Mass>>,
    mut contacts: ResMut<StaticContacts>,
) {
    for (entity_a, mut pos_a, box_a) in dynamics.iter_mut() {
        for (entity_b, pos_b, box_b) in statics.iter() {
            if let Some(Contact {
                normal,
                penetration,
            }) = contact::box_box(pos_a.0, box_a.size, pos_b.0, box_b.size)
            {
                constrain_body_position(&mut pos_a, normal, penetration);
                contacts.0.push((entity_a, entity_b, normal));
            }
        }
    }
}

pub fn update_vel(mut query: Query<(&Pos, &PrevPos, &mut Vel)>) {
    debug!("  update_vel");
    for (pos, prev_pos, mut vel) in query.iter_mut() {
        vel.0 = (pos.0 - prev_pos.0) / SUB_DT;
    }
}

pub fn solve_vel(
    mut query: Query<(&mut Vel, &PreSolveVel, &Mass, &Restitution)>,
    contacts: Res<Contacts>,
) {
    debug!("  solve_vel");
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (
            (mut vel_a, pre_solve_vel_a, mass_a, restitution_a),
            (mut vel_b, pre_solve_vel_b, mass_b, restitution_b),
        ) = query.get_pair_mut(entity_a, entity_b).unwrap();
        constrain_body_velocities(
            &mut vel_a,
            &mut vel_b,
            pre_solve_vel_a,
            pre_solve_vel_b,
            mass_a,
            mass_b,
            restitution_a,
            restitution_b,
            n,
        );
    }
}

pub fn solve_vel_statics(
    mut dynamics: Query<(&mut Vel, &PreSolveVel, &Restitution), With<Mass>>,
    statics: Query<&Restitution, Without<Mass>>,
    contacts: Res<StaticContacts>,
) {
    for (entity_a, entity_b, n) in contacts.0.iter().cloned() {
        let (mut vel_a, pre_solve_vel_a, restitution_a) = dynamics.get_mut(entity_a).unwrap();
        let restitution_b = statics.get(entity_b).unwrap();
        constrain_body_velocity(&mut vel_a, pre_solve_vel_a, restitution_a, restitution_b, n);
    }
}

/// Copies positions from the physics world to bevy Transforms
pub fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    debug!("sync_transforms");
    for (mut transform, pos) in query.iter_mut() {
        let z = transform.translation.z;
        transform.translation = pos.0.extend(z);
    }
}

// Helpers, not systems:

fn constrain_body_positions(
    pos_a: &mut Pos,
    pos_b: &mut Pos,
    mass_a: &Mass,
    mass_b: &Mass,
    n: Vec2,
    penetration_depth: f32,
) {
    let w_a = 1. / mass_a.0;
    let w_b = 1. / mass_b.0;
    let w_sum = w_a + w_b;
    let pos_impulse = n * (-penetration_depth / w_sum);
    pos_a.0 += pos_impulse * w_a;
    pos_b.0 -= pos_impulse * w_b;
}

// todo: just inline this, it's not worth a function
fn constrain_body_position(pos: &mut Pos, n: Vec2, penetration_depth: f32) {
    pos.0 -= n * penetration_depth;
}

fn constrain_body_velocities(
    vel_a: &mut Vel,
    vel_b: &mut Vel,
    pre_solve_vel_a: &PreSolveVel,
    pre_solve_vel_b: &PreSolveVel,
    mass_a: &Mass,
    mass_b: &Mass,
    restitution_a: &Restitution,
    restitution_b: &Restitution,
    n: Vec2,
) {
    let pre_solve_relative_vel = pre_solve_vel_a.0 - pre_solve_vel_b.0;
    let pre_solve_normal_vel = Vec2::dot(pre_solve_relative_vel, n);

    let relative_vel = vel_a.0 - vel_b.0;
    let normal_vel = Vec2::dot(relative_vel, n);
    let restitution = (restitution_a.0 + restitution_b.0) / 2.;

    let w_a = 1. / mass_a.0;
    let w_b = 1. / mass_b.0;
    let w_sum = w_a + w_b;

    let restitution_velocity = (-restitution * pre_solve_normal_vel).min(0.);
    let vel_impulse = n * ((-normal_vel + restitution_velocity) / w_sum);

    vel_a.0 += vel_impulse * w_a;
    vel_b.0 -= vel_impulse * w_b;
}

fn constrain_body_velocity(
    vel_a: &mut Vel,
    pre_solve_vel_a: &PreSolveVel,
    restitution_a: &Restitution,
    restitution_b: &Restitution,
    n: Vec2,
) {
    let pre_solve_normal_vel = Vec2::dot(pre_solve_vel_a.0, n);
    let normal_vel = Vec2::dot(vel_a.0, n);
    let restitution = (restitution_a.0 + restitution_b.0) / 2.;
    vel_a.0 += n * (-normal_vel + (-restitution * pre_solve_normal_vel).min(0.));
}
