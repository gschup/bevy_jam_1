use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::SessionType;
use ggrs::{P2PSession, PlayerHandle};

use crate::{menu::connect::LocalHandles, AttackerAssets, DefenderAssets, GGRSConfig};

use super::{
    prelude::*, FRAMES_PER_SPRITE, INPUT_ACT, INPUT_DOWN, INPUT_LEFT, INPUT_RIGHT, INPUT_UP,
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

pub fn setup_game(mut commands: Commands) {
    commands.insert_resource(RoundState::InterludeStart);
    commands.insert_resource(FrameCount::default());
    commands.insert_resource(RoundData::default());
    let mut cam = OrthographicCameraBundle::new_2d();
    cam.orthographic_projection.scale = 1. / 2.; // Asset pixels are 2 times bigger than "device points"
    commands.spawn_bundle(cam).insert(GameEntity);
}

pub fn print_p2p_events(mut session: ResMut<P2PSession<GGRSConfig>>) {
    for event in session.events() {
        info!("GGRS Event: {:?}", event);
    }
}

pub fn cleanup_game(query: Query<Entity, With<GameEntity>>, mut commands: Commands) {
    commands.remove_resource::<RoundData>();
    commands.remove_resource::<FrameCount>();
    commands.remove_resource::<LocalHandles>();
    commands.remove_resource::<P2PSession<GGRSConfig>>();
    commands.remove_resource::<SessionType>();

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

pub fn load_ldtk_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    // todo: disable in release builds?
    asset_server.watch_for_changes().unwrap();

    let ldtk_handle = asset_server.load("levels/level.ldtk");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}
