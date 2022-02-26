use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use bytemuck::{Pod, Zeroable};
use ggrs::{InputStatus, P2PSession, PlayerHandle};

use crate::{
    checksum::Checksum,
    menu::{connect::LocalHandles, win::MatchData},
    physics::prelude::*,
    AppState, GGRSConfig, NUM_PLAYERS,
};

const INPUT_UP: u8 = 0b0001;
const INPUT_DOWN: u8 = 0b0010;
const INPUT_LEFT: u8 = 0b0100;
const INPUT_RIGHT: u8 = 0b1000;

const GROUND: Color = Color::rgb(0.5, 0.5, 0.5);
const BLUE: Color = Color::rgb(0.8, 0.6, 0.2);
const ORANGE: Color = Color::rgb(0., 0.35, 0.8);
const MAGENTA: Color = Color::rgb(0.9, 0.2, 0.2);
const GREEN: Color = Color::rgb(0.35, 0.7, 0.35);
const PLAYER_COLORS: [Color; 4] = [BLUE, ORANGE, MAGENTA, GREEN];

const PLAYER_SIZE: f32 = 24.;
const MAX_SPEED: f32 = 50.;
const JUMP_HEIGHT: f32 = 24.;
const ARENA_SIZE: f32 = 720.0;
const GROUND_LEVEL: f32 = -100.;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Pod, Zeroable)]
pub struct Input {
    pub inp: u8,
}

#[derive(Default, Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component)]
pub struct RoundEntity;

#[derive(Default, Reflect, Component)]
pub struct PlatformerControls {
    accel: f32,
    horizontal: f32,
}

#[derive(Default, Reflect, Hash, Component)]
#[reflect(Hash)]
pub struct FrameCount {
    pub frame: u32,
}

pub fn input(
    handle: In<PlayerHandle>,
    keyboard_input: Res<bevy::input::Input<KeyCode>>,
    local_handles: Res<LocalHandles>,
) -> Input {
    let mut inp: u8 = 0;

    if handle.0 == local_handles.handles[0] {
        if keyboard_input.pressed(KeyCode::W) {
            inp |= INPUT_UP;
        }
        if keyboard_input.pressed(KeyCode::A) {
            inp |= INPUT_LEFT;
        }
        if keyboard_input.pressed(KeyCode::S) {
            inp |= INPUT_DOWN;
        }
        if keyboard_input.pressed(KeyCode::D) {
            inp |= INPUT_RIGHT;
        }
    } else {
        if keyboard_input.pressed(KeyCode::Up) {
            inp |= INPUT_UP;
        }
        if keyboard_input.pressed(KeyCode::Left) {
            inp |= INPUT_LEFT;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            inp |= INPUT_DOWN;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            inp |= INPUT_RIGHT;
        }
    }

    Input { inp }
}

pub fn setup_round(mut commands: Commands) {
    commands.insert_resource(FrameCount::default());
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scale = 1. / 2.; // Asset pixels are 2 times bigger than "device points"
    commands.spawn_bundle(cam).insert(RoundEntity);

    // commands
    //     .spawn_bundle(SpriteBundle {
    //         transform: Transform::from_xyz(0., 0., 0.),
    //         sprite: Sprite {
    //             color: Color::BLACK,
    //             custom_size: Some(Vec2::new(ARENA_SIZE, ARENA_SIZE)),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert(RoundEntity);

    // level geometry

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
        });

    // // Add a falling box to see that physics are working
    // let box_size = Vec2::new(24., 24.);
    // commands
    //     .spawn_bundle(SpriteBundle {
    //         sprite: Sprite {
    //             color: BLUE,
    //             custom_size: Some(box_size),
    //             ..Default::default()
    //         },
    //         ..Default::default()
    //     })
    //     .insert_bundle(DynamicBoxBundle {
    //         pos: Pos(Vec2::new(0., 0.)),
    //         collider: BoxCollider { size: box_size },
    //         ..Default::default()
    //     });
    //     // todo: add rollback entity
}

pub fn spawn_players(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    let r = ARENA_SIZE / 4.;

    for handle in 0..NUM_PLAYERS {
        let rot = handle as f32 / NUM_PLAYERS as f32 * 2. * std::f32::consts::PI;
        let x = r * rot.cos();
        let y = r * rot.sin();

        let mut transform = Transform::from_translation(Vec3::new(x, y, 1.));
        transform.rotate(Quat::from_rotation_z(rot));

        let player_size = Vec2::new(PLAYER_SIZE / 2., PLAYER_SIZE);

        commands
            .spawn_bundle(SpriteBundle {
                transform,
                sprite: Sprite {
                    color: PLAYER_COLORS[handle],
                    custom_size: Some(player_size),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert_bundle(DynamicBoxBundle {
                pos: Pos(Vec2::new(0., 0.)),
                collider: BoxCollider { size: player_size },
                ..Default::default()
            })
            .insert(Player { handle })
            .insert(PlatformerControls::default())
            .insert(Checksum::default())
            .insert(Rollback::new(rip.next_id()))
            .insert(RoundEntity);
    }
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn check_win(mut state: ResMut<State<AppState>>, mut commands: Commands) {
    let condition = false;
    let confirmed = false;

    if condition && confirmed {
        state.set(AppState::Win).expect("Could not change state.");
        commands.insert_resource(MatchData {
            result: "Orange won!".to_owned(),
        });
    }
}

pub fn cleanup(query: Query<Entity, With<RoundEntity>>, mut commands: Commands) {
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
    commands.remove_resource::<SessionType>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

/*
 * ROLLBACK SYSTEMS
 */

pub fn increase_frame_count(mut frame_count: ResMut<FrameCount>) {
    frame_count.frame += 1;
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
    mut query: Query<(&mut Vel, &PlatformerControls), With<Rollback>>,
    gravity: Res<Gravity>,
) {
    for (mut vel, controls) in query.iter_mut() {
        // just set horizontal velocity for now
        // this totally overwrites any velocity on the x axis, which might not be ideal...
        vel.0.x = controls.horizontal * MAX_SPEED;

        if controls.accel > 0. {
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
