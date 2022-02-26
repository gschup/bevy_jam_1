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

const IDLE_THRESH: f32 = 0.01;
const LAND_FRAMES: u16 = 3;
/// Needs to happen before input
pub fn update_attacker_state(
    // todo: would maybe be cleaner to just expose one resource, that includes dynamics as well
    // just check against statics (ground) for now
    contacts: ResMut<StaticContacts>,
    mut query: Query<(Entity, &Vel, &mut AttackerState)>,
) {
    for (id, vel, mut state) in query.iter_mut() {
        match *state {
            AttackerState::Idle(ref mut f) => {
                if vel.0.y < 0. {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > 0. {
                    *state = AttackerState::Jump(0);
                    continue;
                }
                if vel.0.x.abs() > IDLE_THRESH {
                    *state = AttackerState::Walk(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Jump(ref mut f) => {
                if vel.0.y < 0. {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Fall(ref mut f) => {
                if let Some(_) = contacts.0.iter().find(|(e, _, n)| *e == id && n.y < 0.) {
                    *state = AttackerState::Land(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Land(ref mut f) => {
                if vel.0.y < 0. {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if *f > LAND_FRAMES {
                    *state = AttackerState::Idle(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Walk(ref mut f) => {
                if vel.0.y < 0. {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > 0. {
                    *state = AttackerState::Jump(0);
                    continue;
                }
                if vel.0.x.abs() < IDLE_THRESH {
                    *state = AttackerState::Idle(0);
                    continue;
                }
                *f += 1;
            }
        };
    }
}

/// Needs to happen before all systems that use PlatformerControls, or it will desync
pub fn apply_inputs(
    mut query: Query<(&mut PlatformerControls, &Attacker)>,
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
    mut query: Query<(&mut Vel, &AttackerState, &PlatformerControls), With<Rollback>>,
    gravity: Res<Gravity>,
) {
    for (mut vel, state, controls) in query.iter_mut() {
        // just set horizontal velocity for now
        // this totally overwrites any velocity on the x axis, which might not be ideal...
        vel.0.x = controls.horizontal * MAX_SPEED;

        if controls.accel > 0. && state.can_jump() {
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
