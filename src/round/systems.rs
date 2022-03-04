use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_ggrs::SessionType;
use ggrs::{P2PSession, PlayerHandle};

use crate::{
    menu::connect::LocalHandles, AttackerAssets, DefenderAssets, FontAssets, GGRSConfig,
    MiscAssets, BUTTON_TEXT, NUM_PLAYERS, SCREEN_X, SCREEN_Y,
};

use super::{
    prelude::*, FRAMES_PER_SPRITE, GROUND_LEVEL, INPUT_ACT, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT,
    INPUT_UP, ROUND_LENGTH,
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
        if keyboard_input.pressed(KeyCode::Space) {
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
        if keyboard_input.pressed(KeyCode::RShift) {
            inp |= INPUT_ACT;
        }
    }

    super::resources::Input { inp }
}

pub fn on_interlude_start(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::InterludeStart => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn on_interlude(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::Interlude => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn on_interlude_end(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::InterludeEnd => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn on_round_start(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::RoundStart => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn on_round(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::Round => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn on_round_end(state: Res<RoundState>) -> ShouldRun {
    match *state {
        RoundState::RoundEnd => ShouldRun::Yes,
        _ => ShouldRun::No,
    }
}

pub fn setup_game(mut commands: Commands, misc_sprites: Res<MiscAssets>) {
    commands.insert_resource(RoundState::InterludeStart);
    commands.insert_resource(FrameCount::default());
    commands.insert_resource(RoundData::default());
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scale = 1. / 2.; // Asset pixels are 2 times bigger than "device points"
    commands.spawn_bundle(cam).insert(GameEntity);

    commands
        .spawn_bundle(SpriteBundle {
            texture: misc_sprites.background.clone(),
            ..Default::default()
        })
        .insert(GameEntity);
}

pub fn setup_network_stats_ui(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.insert_resource(ConnectionInfo {
        status: ConnectionStatus::Synchronizing,
        ping: 0,
    });
    commands
        .spawn_bundle(Text2dBundle {
            transform: Transform::from_xyz(
                -SCREEN_X / 4.,
                -SCREEN_Y / 4. - GROUND_LEVEL / 2.,
                100.,
            ),
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Ping: -\n".to_owned(),
                        style: TextStyle {
                            font: font_assets.default_font.clone(),
                            font_size: 20.0,
                            color: BUTTON_TEXT,
                        },
                    },
                    TextSection {
                        value: "Status: -".to_owned(),
                        style: TextStyle {
                            font: font_assets.default_font.clone(),
                            font_size: 20.0,
                            color: BUTTON_TEXT,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(NetworkStatsUi)
        .insert(GameEntity);
}

pub fn handle_p2p_events(
    mut con_info: ResMut<ConnectionInfo>,
    mut session: ResMut<P2PSession<GGRSConfig>>,
) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
        match event {
            ggrs::GGRSEvent::Synchronized { .. } => con_info.status = ConnectionStatus::Running,
            ggrs::GGRSEvent::Disconnected { .. } => {
                con_info.status = ConnectionStatus::Disconnected
            }
            ggrs::GGRSEvent::NetworkInterrupted { .. } => {
                con_info.status = ConnectionStatus::Interrupted
            }
            ggrs::GGRSEvent::NetworkResumed { .. } => con_info.status = ConnectionStatus::Running,
            _ => (),
        }
    }
}

// TODO: for now, this assumes only a single remote player
pub fn update_connection_info(
    mut con_info: ResMut<ConnectionInfo>,
    session: ResMut<P2PSession<GGRSConfig>>,
    local_handles: Res<LocalHandles>,
) {
    for handle in 0..NUM_PLAYERS {
        if local_handles.handles.contains(&handle) {
            continue;
        }
        if let Ok(stats) = session.network_stats(handle) {
            con_info.ping = stats.ping;
        }
    }
}

pub fn update_connection_display(
    con_info: Res<ConnectionInfo>,
    mut text_query: Query<&mut Text, With<NetworkStatsUi>>,
) {
    for mut text in text_query.iter_mut() {
        text.sections[0].value = format!("Ping: {}\n", con_info.ping);
        text.sections[1].value = format!("Status: {}", con_info.status);
    }
}

pub fn update_screen_timer(
    frame_count: Res<FrameCount>,
    mut timer: Query<&mut Text, With<ScreenTimer>>,
) {
    let remaining_secs = (ROUND_LENGTH - frame_count.frame) / 60;

    for mut text in timer.iter_mut() {
        text.sections[0].value = remaining_secs.to_string();
    }
}

pub fn cleanup_game(query: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    commands.remove_resource::<RoundData>();
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
    commands.remove_resource::<SessionType>();
    commands.remove_resource::<ConnectionInfo>();

    for e in query.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub fn update_attacker_sprite(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &FacingDirection,
        &AttackerState,
    )>,
    sprites: Res<AttackerAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut sprite, mut atlas_handle, face_dir, state) in query.iter_mut() {
        match *state {
            AttackerState::Idle(_) => *atlas_handle = sprites.janitor_idle.clone(),
            AttackerState::Jump(_) => *atlas_handle = sprites.janitor_jump.clone(),
            AttackerState::Fall(_) => *atlas_handle = sprites.janitor_fall.clone(),
            AttackerState::Land(_) => *atlas_handle = sprites.janitor_land.clone(),
            AttackerState::Walk(_) => *atlas_handle = sprites.janitor_walk.clone(),
            AttackerState::Hit(_) => *atlas_handle = sprites.janitor_hit.clone(),
        }

        let texture_atlas = texture_atlases
            .get(atlas_handle.as_ref())
            .expect("TextureAtlas not found.");
        sprite.index = (state.get_frame() / FRAMES_PER_SPRITE) % texture_atlas.textures.len();
        sprite.flip_x = match *face_dir {
            FacingDirection::Left => true,
            FacingDirection::Right => false,
        }
    }
}

pub fn update_defender_sprite(
    mut query: Query<(
        &mut TextureAtlasSprite,
        &mut Handle<TextureAtlas>,
        &FacingDirection,
        &DefenderState,
    )>,
    sprites: Res<DefenderAssets>,
    texture_atlases: Res<Assets<TextureAtlas>>,
) {
    for (mut sprite, mut atlas_handle, face_dir, state) in query.iter_mut() {
        match *state {
            DefenderState::Idle(_) => *atlas_handle = sprites.fortress_idle.clone(),
            DefenderState::Fire(_) => *atlas_handle = sprites.fortress_fire.clone(),
        }

        let texture_atlas = texture_atlases
            .get(atlas_handle.as_ref())
            .expect("TextureAtlas not found.");
        sprite.index = (state.get_frame() / FRAMES_PER_SPRITE) % texture_atlas.textures.len();
        sprite.flip_x = match *face_dir {
            FacingDirection::Left => true,
            FacingDirection::Right => false,
        }
    }
}
