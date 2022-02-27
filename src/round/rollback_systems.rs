use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;

use crate::{
    checksum::Checksum,
    menu::win::MatchResult,
    physics::prelude::*,
    round::{prelude::*, resources::Input},
    AppState, SpriteAssets, NUM_PLAYERS,
};

use super::{
    GROUND, GROUND_LEVEL, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP, JUMP_HEIGHT, MAX_SPEED,
    NUM_ROUNDS, PLAYER_SIZE,
};

const INTERLUDE_LENGTH: u32 = 60;
const ROUND_LENGTH: u32 = 600;

/*
 * INTERLUDE
 */

pub fn setup_interlude(mut state: ResMut<RoundState>) {
    *state = RoundState::Interlude;
    //println!("INTERLUDE_START");
}

pub fn run_interlude(mut frame_count: ResMut<FrameCount>, mut state: ResMut<RoundState>) {
    frame_count.frame += 1;
    if frame_count.frame >= INTERLUDE_LENGTH {
        *state = RoundState::InterludeEnd;
    }
    // println!("INTERLUDE {}", frame_count.frame);
}

pub fn cleanup_interlude(mut frame_count: ResMut<FrameCount>, mut state: ResMut<RoundState>) {
    frame_count.frame = 0;
    *state = RoundState::RoundStart;
    // println!("INTERLUDE_END");
}

/*
 * ROUND START
 */

pub fn spawn_world(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    // todo: could import the body builder from bevy_xpbd to clean this up
    let ground_size = Vec2::new(2000., 2000.); // should just be bigger than the screen
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(ground_size),
                color: GROUND,
                ..Default::default()
            },
            // using transform for now, could probably just as well use custom size
            ..Default::default()
        })
        .insert_bundle(StaticBoxBundle {
            pos: Pos(Vec2::new(0., -ground_size.y / 2. + GROUND_LEVEL)),
            collider: BoxCollider { size: ground_size },
            ..Default::default()
        })
        .insert(Rollback::new(rip.next_id()))
        .insert(RoundEntity);
}

pub fn spawn_players(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    sprites: Res<SpriteAssets>,
) {
    for handle in 0..NUM_PLAYERS {
        let x = (2. * handle as f32 - 1.) * 20.;
        let y = 0.;
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform::from_xyz(x, y, 1.),
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: sprites.janitor_idle.clone(),
                ..Default::default()
            })
            .insert_bundle(DynamicBoxBundle {
                pos: Pos(Vec2::new(x, y)),
                collider: BoxCollider {
                    size: Vec2::new(PLAYER_SIZE / 2., PLAYER_SIZE),
                },
                ..Default::default()
            })
            .insert(Attacker { handle })
            .insert(AttackerState::Idle(0))
            .insert(FacingDirection::Right)
            .insert(PlatformerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity);
    }
}

pub fn start_round(mut frame_count: ResMut<FrameCount>, mut state: ResMut<RoundState>) {
    frame_count.frame = 0;
    *state = RoundState::Round;
    //println!("\nROUND START");
}

/*
 * ROUND UPDATE
 */

const IDLE_THRESH: f32 = 0.01;
const LAND_FRAMES: usize = 3;
/// Needs to happen before input
pub fn update_attacker_state(
    // todo: would maybe be cleaner to just expose one resource, that includes dynamics as well
    // just check against statics (ground) for now
    contacts: ResMut<Contacts>,
    static_contacts: ResMut<StaticContacts>,
    mut query: Query<(Entity, &Vel, &mut AttackerState, &mut FacingDirection)>,
) {
    for (id, vel, mut state, mut face_dir) in query.iter_mut() {
        // update facing direction
        if vel.0.x.is_sign_negative() && vel.0.x.abs() > IDLE_THRESH {
            *face_dir = FacingDirection::Left;
        }

        if vel.0.x.is_sign_positive() && vel.0.x.abs() > IDLE_THRESH {
            *face_dir = FacingDirection::Right;
        }

        //update state
        match *state {
            AttackerState::Idle(ref mut f) => {
                if vel.0.y < -0.01 {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > 0.01 {
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
                if vel.0.y < 0.01 {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Fall(ref mut f) => {
                if static_contacts
                    .0
                    .iter()
                    .any(|(e, _, n)| *e == id && n.y < 0.)
                    || contacts.0.iter().any(|(a, b, n)| {
                        if *a == id {
                            n.y < 0.
                        } else if *b == id {
                            n.y > 0.
                        } else {
                            false
                        }
                    })
                {
                    *state = AttackerState::Land(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Land(ref mut f) => {
                if vel.0.y < -0.01 {
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
                if vel.0.y < -0.01 {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > 0.01 {
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

pub fn check_round_end(mut frame_count: ResMut<FrameCount>, mut round_state: ResMut<RoundState>) {
    frame_count.frame += 1;

    // dummy win condition - game ends after ROUND_LENGTH frames
    if frame_count.frame >= ROUND_LENGTH {
        *round_state = RoundState::RoundEnd;
    }

    //println!("\nROUND {}", frame_count.frame);
}

/*
 * ROUND END
 */

// despawns players and the world
pub fn cleanup_round(
    query: Query<Entity, With<RoundEntity>>,
    mut frame_count: ResMut<FrameCount>,
    mut round_state: ResMut<RoundState>,
    mut round_data: ResMut<RoundData>,
    mut app_state: ResMut<State<AppState>>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }

    frame_count.frame = 0;
    round_data.cur_round += 1; // update round information

    if round_data.cur_round >= NUM_ROUNDS {
        // go to win screen
        match app_state.set(AppState::Win) {
            Ok(_) => commands.insert_resource(MatchResult {
                result: "TODO!".to_owned(), // TODO: should be created from the RoundData
            }),
            Err(e) => warn!("Could not change app state to AppState::Win : {}", e), // this happens when there is a rollback and the change to app win is queued twice
        };
    } else {
        // start another round
        *round_state = RoundState::InterludeStart;
    }
    //println!("\nROUND END {:?}", *round_data);
}
