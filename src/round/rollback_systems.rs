use bevy::prelude::*;
use bevy_ggrs::Rollback;
use ggrs::InputStatus;

use crate::{
    physics::prelude::*,
    round::{prelude::*, resources::Input},
};

use super::{INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP, JUMP_HEIGHT, MAX_SPEED};

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
}

/// Needs to happen before input
pub fn ground_check(
    // todo: would maybe be cleaner to just expose one resource, that includes dynamics as well
    // just check against statics (ground) for now
    contacts: ResMut<StaticContacts>,
    mut query: Query<&mut Grounded>,
) {
    // just clear existing state
    for mut grounded in query.iter_mut() {
        grounded.0 = false
    }

    for (dynamic_entity, _, normal) in contacts.0.iter() {
        // The normal points from the dynamic entity to the static entity
        // so a negative y means we're standing on top of the other collider
        if normal.y < 0. {
            if let Ok(mut grounded) = query.get_mut(*dynamic_entity) {
                grounded.0 = true;
            }
        }
    }
}

/// Needs to happen before all systems that use PlatformerControls, or it will desync
pub fn apply_inputs(
    mut query: Query<(&mut PlatformerControls, &Player)>,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (mut c, p) in query.iter_mut() {
        let input = match inputs[p.handle].1 {
            InputStatus::Confirmed => inputs[p.handle].0.inp,
            InputStatus::Predicted => inputs[p.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        c.horizontal = if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            -1. // right positive
        } else if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            1.
        } else {
            0.
        };

        c.accel = if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            -1. // up positive
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
            1.
        } else {
            0.
        };
    }
}

pub fn move_players(
    mut query: Query<(&mut Vel, &Grounded, &PlatformerControls), With<Rollback>>,
    gravity: Res<Gravity>,
) {
    for (mut vel, grounded, controls) in query.iter_mut() {
        // just set horizontal velocity for now
        // this totally overwrites any velocity on the x axis, which might not be ideal...
        vel.0.x = controls.horizontal * MAX_SPEED;

        if controls.accel > 0. && grounded.0 {
            // todo: ground check + only trigger on press
            let v0 = f32::sqrt(-2. * JUMP_HEIGHT * gravity.0.y);
            vel.0.y = controls.accel * v0;
            // vel.0.y = controls.accel * MAX_SPEED;
        }

        // todo: could just be added in the physics inner loop system
        // // constrain cube to plane
        // let bounds = (ARENA_SIZE - CUBE_SIZE) * 0.5;
        // t.translation.x = t.translation.x.clamp(-bounds, bounds);
        // t.translation.y = t.translation.y.clamp(-bounds, bounds);
    }
}
