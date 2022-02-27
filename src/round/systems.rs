use bevy::prelude::*;
use bevy_ggrs::{Rollback, RollbackIdProvider, SessionType};
use ggrs::{P2PSession, PlayerHandle};

use crate::{
    checksum::Checksum,
    menu::{connect::LocalHandles, win::MatchData},
    physics::prelude::*,
    AppState, GGRSConfig, NUM_PLAYERS,
};

use super::{
    prelude::*, ARENA_SIZE, GROUND, GROUND_LEVEL, INPUT_ACT, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT,
    INPUT_UP, PLAYER_COLORS, PLAYER_SIZE,
};

pub fn input(
    handle: In<PlayerHandle>,
    keyboard_input: Res<bevy::input::Input<KeyCode>>,
    local_handles: Res<LocalHandles>,
) -> super::resources::Input {
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
        if keyboard_input.just_pressed(KeyCode::Space) {
            inp |= INPUT_ACT;
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
        if keyboard_input.just_pressed(KeyCode::RShift) {
            inp |= INPUT_ACT;
        }
    }

    super::resources::Input { inp }
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
            .insert(Attacker { handle })
            .insert(AttackerState::Idle(0))
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
