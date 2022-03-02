use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider};
use ggrs::InputStatus;

use crate::{
    checksum::Checksum,
    menu::win::MatchResult,
    physics::prelude::*,
    round::{prelude::*, resources::Input},
    AppState, AttackerAssets, DefenderAssets, MiscAssets, NUM_PLAYERS,
};

use super::{
    ATTACKER_SIZE, CAKE_SIZE, CROSSHAIR_SPEED, DEFENDER_SIZE, DEF_X_POS, FRAMES_PER_SPRITE, GROUND,
    GROUND_LEVEL, IDLE_THRESH, INPUT_ACT, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP,
    INTERLUDE_LENGTH, JUMP_HEIGHT, JUMP_TIME_TO_PEAK, LAND_FRAMES, MAX_SPEED, NUM_ROUNDS,
    ROUND_LENGTH,
};

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

pub fn spawn_attackers(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    sprites: Res<AttackerAssets>,
    round_data: Res<RoundData>,
) {
    for handle in 0..NUM_PLAYERS {
        // this player will be the defender instead
        if handle == round_data.cur_round as usize {
            continue;
        }
        let x = 0.;
        let y = 0.;
        commands
            .spawn_bundle(SpriteSheetBundle {
                transform: Transform::from_xyz(x, y, (handle + 1) as f32),
                sprite: TextureAtlasSprite::new(0),
                texture_atlas: sprites.janitor_idle.clone(),
                ..Default::default()
            })
            .insert_bundle(DynamicBoxBundle {
                pos: Pos(Vec2::new(x, y)),
                collider: BoxCollider {
                    size: Vec2::new(ATTACKER_SIZE / 2., ATTACKER_SIZE),
                },
                ..Default::default()
            })
            .insert(Attacker { handle })
            .insert(AttackerState::Idle(0))
            .insert(FacingDirection::Right)
            .insert(AttackerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity);
    }
}

pub fn spawn_defender(
    mut commands: Commands,
    mut rip: ResMut<RollbackIdProvider>,
    def_sprites: Res<DefenderAssets>,
    misc_sprites: Res<MiscAssets>,
    round_data: Res<RoundData>,
) {
    let x = DEF_X_POS;
    let y = GROUND_LEVEL + DEFENDER_SIZE / 2.;
    commands
        .spawn_bundle(SpriteSheetBundle {
            transform: Transform::from_xyz(x, y, 1.),
            sprite: TextureAtlasSprite::new(0),
            texture_atlas: def_sprites.fortress_idle.clone(),
            ..Default::default()
        })
        .insert(Defender {
            handle: round_data.cur_round as usize,
        })
        .insert(DefenderState::Idle(0))
        .insert(FacingDirection::Right)
        .insert(DefenderControls::default())
        .insert(Checksum::default())
        .insert(Rollback::new(rip.next_id()))
        .insert(RoundEntity);

    // crosshair
    commands
        .spawn_bundle(SpriteBundle {
            texture: misc_sprites.crosshair.clone(),
            transform: Transform::from_xyz(0., 0., 6.),
            ..Default::default()
        })
        .insert(Crosshair)
        .insert(Checksum::default())
        .insert(Rollback::new(rip.next_id()))
        .insert(RoundEntity);
}

pub fn start_round(mut frame_count: ResMut<FrameCount>, mut state: ResMut<RoundState>) {
    frame_count.frame = 0;
    *state = RoundState::Round;
    //println!("\nROUND START");
}

/*
 * ROUND UPDATE
 */

pub fn update_defender_state(
    mut commands: Commands,
    sprites: Res<MiscAssets>,
    mut rip: ResMut<RollbackIdProvider>,
    gravity: Res<Gravity>,
    mut def_query: Query<(&Transform, &DefenderControls, &mut DefenderState)>,
    crosshair_query: Query<&Transform, With<Crosshair>>,
) {
    let mut should_shoot = false;
    let mut cake_x = 0.;
    let mut cake_y = 0.;

    for (t, contr, mut state) in def_query.iter_mut() {
        match *state {
            DefenderState::Idle(ref mut f) => {
                if contr.fire {
                    *state = DefenderState::Fire(0);
                    continue;
                }
                *f += 1;
            }
            DefenderState::Fire(ref mut f) => {
                // fire anim has 4 frames
                if *f >= FRAMES_PER_SPRITE * 4 {
                    *state = DefenderState::Idle(0);
                    continue;
                }
                // fire the cake after the first two frames of animation have played
                if *f == FRAMES_PER_SPRITE * 2 {
                    should_shoot = true;
                    cake_x = t.translation.x - DEFENDER_SIZE / 2. + 10.;
                    cake_y = t.translation.y + 5.;
                }
                *f += 1;
            }
        }
    }

    for t in crosshair_query.iter() {
        if should_shoot {
            let dist_x = (t.translation.x - cake_x).min(0.);
            let dist_y = (t.translation.y - cake_y).max(0.);
            let cake_vx = 2. * dist_x / JUMP_TIME_TO_PEAK; // TODO: this is not correct if the crosshair is supposed to be the apex of the parabola
            let cake_vy = f32::sqrt(-2. * dist_y * gravity.0.y);
            commands
                .spawn_bundle(SpriteBundle {
                    texture: sprites.cake.clone(),
                    transform: Transform::from_xyz(cake_x, cake_y, 5.),
                    ..Default::default()
                })
                .insert_bundle(DynamicBoxBundle {
                    pos: Pos(Vec2::new(cake_x, cake_y)),
                    collider: BoxCollider {
                        size: Vec2::new(CAKE_SIZE, CAKE_SIZE),
                    },
                    vel: Vel(Vec2::new(cake_vx, cake_vy)),
                    ..Default::default()
                })
                .insert(Cake)
                .insert(Checksum::default())
                .insert(Rollback::new(rip.next_id()))
                .insert(RoundEntity);
        }
    }
}

/// Needs to happen before input
pub fn update_attacker_state(
    // todo: would maybe be cleaner to just expose one resource, that includes dynamics as well
    // just check against statics (ground) for now
    contacts: ResMut<Contacts>,
    static_contacts: ResMut<StaticContacts>,
    mut query: Query<(
        Entity,
        &Vel,
        &AttackerControls,
        &mut AttackerState,
        &mut FacingDirection,
    )>,
) {
    for (id, vel, contr, mut state, mut face_dir) in query.iter_mut() {
        // update facing direction
        if contr.horizontal < -IDLE_THRESH {
            *face_dir = FacingDirection::Left;
        }

        if contr.horizontal > IDLE_THRESH {
            *face_dir = FacingDirection::Right;
        }

        //update state
        match *state {
            AttackerState::Idle(ref mut f) => {
                if vel.0.y < -IDLE_THRESH {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > IDLE_THRESH {
                    *state = AttackerState::Jump(0);
                    continue;
                }
                if contr.horizontal.abs() > IDLE_THRESH {
                    *state = AttackerState::Walk(0);
                    continue;
                }
                *f += 1;
            }
            AttackerState::Jump(ref mut f) => {
                if vel.0.y < IDLE_THRESH {
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
                if vel.0.y < -IDLE_THRESH {
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
                if vel.0.y < -IDLE_THRESH {
                    *state = AttackerState::Fall(0);
                    continue;
                }
                if vel.0.y > IDLE_THRESH {
                    *state = AttackerState::Jump(0);
                    continue;
                }
                if contr.horizontal.abs() < IDLE_THRESH {
                    *state = AttackerState::Idle(0);
                    continue;
                }
                *f += 1;
            }
        };
    }
}

pub fn apply_attacker_inputs(
    mut query: Query<(&mut AttackerControls, &Attacker)>,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (mut controls, attacker) in query.iter_mut() {
        let input = match inputs[attacker.handle].1 {
            InputStatus::Confirmed => inputs[attacker.handle].0.inp,
            InputStatus::Predicted => inputs[attacker.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        controls.horizontal = if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            -1. // right positive
        } else if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            1.
        } else {
            0.
        };

        controls.vertical = if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            -1. // up positive
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
            1.
        } else {
            0.
        };
    }
}

pub fn apply_defender_inputs(
    mut query: Query<(&mut DefenderControls, &Defender)>,
    inputs: Res<Vec<(Input, InputStatus)>>,
) {
    for (mut controls, def) in query.iter_mut() {
        let input = match inputs[def.handle].1 {
            InputStatus::Confirmed => inputs[def.handle].0.inp,
            InputStatus::Predicted => inputs[def.handle].0.inp,
            InputStatus::Disconnected => 0, // disconnected players do nothing
        };

        controls.horizontal = if input & INPUT_LEFT != 0 && input & INPUT_RIGHT == 0 {
            -1. // right positive
        } else if input & INPUT_LEFT == 0 && input & INPUT_RIGHT != 0 {
            1.
        } else {
            0.
        };

        controls.vertical = if input & INPUT_DOWN != 0 && input & INPUT_UP == 0 {
            -1. // up positive
        } else if input & INPUT_DOWN == 0 && input & INPUT_UP != 0 {
            1.
        } else {
            0.
        };

        controls.fire = input & INPUT_ACT != 0;
    }
}

pub fn move_attackers(
    mut query: Query<(&mut Vel, &AttackerState, &AttackerControls), With<Rollback>>,
    gravity: Res<Gravity>,
) {
    for (mut vel, state, controls) in query.iter_mut() {
        // just set horizontal velocity for now
        // this totally overwrites any velocity on the x axis, which might not be ideal...
        vel.0.x = controls.horizontal * MAX_SPEED;

        if controls.vertical > 0. && state.can_jump() {
            let v0 = f32::sqrt(-2. * JUMP_HEIGHT * gravity.0.y);
            vel.0.y = controls.vertical * v0;
            // vel.0.y = controls.accel * MAX_SPEED;
        }

        // todo: could just be added in the physics inner loop system
        // // constrain cube to plane
        // let bounds = (ARENA_SIZE - CUBE_SIZE) * 0.5;
        // t.translation.x = t.translation.x.clamp(-bounds, bounds);
        // t.translation.y = t.translation.y.clamp(-bounds, bounds);
    }
}

pub fn move_crosshair(
    input_query: Query<&DefenderControls>,
    mut crosshair_query: Query<&mut Transform, With<Crosshair>>,
) {
    let mut hor = 0.;
    let mut vert = 0.;

    for c in input_query.iter() {
        hor = c.horizontal;
        vert = c.vertical;
    }

    for mut t in crosshair_query.iter_mut() {
        t.translation.x += hor * CROSSHAIR_SPEED;
        t.translation.y += vert * CROSSHAIR_SPEED;
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
