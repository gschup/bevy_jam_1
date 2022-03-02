mod checksum;
mod menu;
mod physics;
mod round;

use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_ecs_ldtk::prelude::*;
use bevy_ggrs::GGRSPlugin;
use checksum::{checksum_attackers, checksum_cakes, checksum_crosshair, checksum_splat, Checksum};
use ggrs::Config;
use menu::{
    connect::{create_matchbox_socket, update_matchbox_socket},
    online::{update_lobby_btn, update_lobby_id, update_lobby_id_display},
};
use physics::{components::*, create_physics_stage, prelude::*};
use round::prelude::*;

const ROLLBACK_SYSTEMS: &str = "rollback_systems";
const CHECKSUM_UPDATE: &str = "checksum_update";
const PHYSICS_UPDATE: &str = "physics_update";

const NUM_PLAYERS: usize = 2;
const FPS: usize = 60;
const MAX_PREDICTION: usize = 12;
const INPUT_DELAY: usize = 2;
const CHECK_DISTANCE: usize = 2;
const SCREEN_X: f32 = 1280.;
const SCREEN_Y: f32 = 720.;

const DISABLED_BUTTON: Color = Color::rgb(0.8, 0.5, 0.5);
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const BUTTON_TEXT: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    AssetLoading,
    MenuMain,
    MenuOnline,
    MenuConnect,
    RoundLocal,
    RoundOnline,
    Win,
}

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
enum SystemLabel {
    UpdateState,
    Input,
    Move,
    End,
}

#[derive(AssetCollection)]
pub struct MiscAssets {
    #[asset(path = "sprites/misc/title.png")]
    pub game_title: Handle<Image>,
    #[asset(path = "sprites/misc/background.png")]
    pub background: Handle<Image>,
    #[asset(path = "sprites/misc/cake.png")]
    pub cake: Handle<Image>,
    #[asset(path = "sprites/misc/crosshair.png")]
    pub crosshair: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub default_font: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AttackerAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 2, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_idle_white.png")]
    janitor_idle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 2, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_walk_white.png")]
    janitor_walk: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 2, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_fall_white.png")]
    janitor_fall: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 1, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_jump_white.png")]
    janitor_jump: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 1, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_land_white.png")]
    janitor_land: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 26., tile_size_y = 26., columns = 1, rows = 1))]
    #[asset(path = "sprites/janitor/janitor_hit_white1.png")]
    janitor_hit: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct DefenderAssets {
    // if the sheet would have padding, we could set that with `padding_x` and `padding_y`
    #[asset(texture_atlas(tile_size_x = 168., tile_size_y = 168., columns = 2, rows = 1))]
    #[asset(path = "sprites/fort/idle_animation_fort.png")]
    fortress_idle: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 168., tile_size_y = 168., columns = 4, rows = 1))]
    #[asset(path = "sprites/fort/fire_animation_fort.png")]
    fortress_fire: Handle<TextureAtlas>,
}

#[derive(Debug)]
pub struct GGRSConfig;
impl Config for GGRSConfig {
    type Input = round::resources::Input;
    type State = u8;
    type Address = String;
}

fn main() {
    let mut app = App::new();

    AssetLoader::new(AppState::AssetLoading)
        .continue_to_state(AppState::MenuMain)
        .with_collection::<MiscAssets>()
        .with_collection::<FontAssets>()
        .with_collection::<AttackerAssets>()
        .with_collection::<DefenderAssets>()
        .build(&mut app);

    GGRSPlugin::<GGRSConfig>::new()
        .with_update_frequency(FPS)
        .with_input_system(input)
        .register_rollback_type::<Attacker>()
        .register_rollback_type::<Defender>()
        .register_rollback_type::<RoundEntity>()
        .register_rollback_type::<AttackerState>()
        .register_rollback_type::<DefenderState>()
        .register_rollback_type::<AttackerControls>()
        .register_rollback_type::<DefenderControls>()
        .register_rollback_type::<FrameCount>()
        .register_rollback_type::<Checksum>()
        .register_rollback_type::<RoundState>()
        .register_rollback_type::<RoundData>()
        .register_rollback_type::<Transform>()
        .register_rollback_type::<FacingDirection>()
        .register_rollback_type::<Cake>()
        .register_rollback_type::<Splat>()
        .register_rollback_type::<Crosshair>()
        // physics types
        .register_rollback_type::<Pos>()
        .register_rollback_type::<Vel>()
        .register_rollback_type::<PrevPos>()
        .register_rollback_type::<PreSolveVel>()
        .register_rollback_type::<Restitution>()
        .register_rollback_type::<BoxCollider>()
        .register_rollback_type::<Mass>()
        .register_rollback_type::<Aabb>()
        .register_rollback_type::<StaticContacts>()
        .register_rollback_type::<Contacts>()
        .with_rollback_schedule(
            Schedule::default()
                // adding physics in a separate stage for now,
                // could perhaps merge with the stage below for increased parallelism...
                // but this is a web jam game, so we don't *really* care about that now...
                .with_stage(PHYSICS_UPDATE, create_physics_stage())
                .with_stage_after(
                    PHYSICS_UPDATE,
                    ROLLBACK_SYSTEMS,
                    SystemStage::parallel()
                        // interlude start
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_interlude_start)
                                .with_system(setup_interlude),
                        )
                        // interlude
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_interlude)
                                .with_system(run_interlude),
                        )
                        // interlude end
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_interlude_end)
                                .with_system(cleanup_interlude),
                        )
                        // round start
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_round_start)
                                .with_system(spawn_attackers)
                                .with_system(spawn_defender)
                                .with_system(spawn_world)
                                .with_system(start_round),
                        )
                        // round
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_round)
                                .with_system(update_attacker_state)
                                .with_system(update_defender_state)
                                .label(SystemLabel::UpdateState),
                        )
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_round)
                                .with_system(apply_attacker_inputs)
                                .with_system(apply_defender_inputs)
                                .label(SystemLabel::Input)
                                .after(SystemLabel::UpdateState),
                        )
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_round)
                                .with_system(move_attackers)
                                .with_system(move_crosshair)
                                .with_system(cake_collision)
                                .with_system(splat_cleaning)
                                .label(SystemLabel::Move)
                                .after(SystemLabel::Input),
                        )
                        .with_system_set(
                            SystemSet::new()
                                .with_run_criteria(on_round)
                                .with_system(check_round_end)
                                .label(SystemLabel::End)
                                .after(SystemLabel::Move),
                        )
                        // round end
                        .with_system_set(
                            SystemSet::new()
                                .after(SystemLabel::End)
                                .with_run_criteria(on_round_end)
                                .with_system(cleanup_round),
                        ),
                )
                .with_stage_after(
                    ROLLBACK_SYSTEMS,
                    CHECKSUM_UPDATE,
                    SystemStage::parallel()
                        .with_system(checksum_attackers)
                        .with_system(checksum_cakes)
                        .with_system(checksum_crosshair)
                        .with_system(checksum_splat),
                ),
        )
        .build(&mut app);

    app.insert_resource(WindowDescriptor {
        width: SCREEN_X,
        height: SCREEN_Y,
        title: "A janitor's Nightmare".to_owned(),
        resizable: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_state(AppState::AssetLoading)
    .insert_resource(ClearColor(Color::BLACK))
    // physics
    .add_plugin(PhysicsPlugin)
    .add_plugin(LdtkPlugin)
    .insert_resource(LevelSelection::Index(0))
    // main menu
    .add_system_set(SystemSet::on_enter(AppState::MenuMain).with_system(menu::main::setup_ui))
    .add_system_set(
        SystemSet::on_update(AppState::MenuMain)
            .with_system(menu::main::btn_visuals)
            .with_system(menu::main::btn_listeners),
    )
    .add_system_set(SystemSet::on_exit(AppState::MenuMain).with_system(menu::main::cleanup_ui))
    //online menu
    .add_system_set(SystemSet::on_enter(AppState::MenuOnline).with_system(menu::online::setup_ui))
    .add_system_set(
        SystemSet::on_update(AppState::MenuOnline)
            .with_system(update_lobby_id)
            .with_system(update_lobby_id_display)
            .with_system(update_lobby_btn)
            .with_system(menu::online::btn_visuals)
            .with_system(menu::online::btn_listeners),
    )
    .add_system_set(SystemSet::on_exit(AppState::MenuOnline).with_system(menu::online::cleanup_ui))
    // connect menu
    .add_system_set(
        SystemSet::on_enter(AppState::MenuConnect)
            .with_system(create_matchbox_socket)
            .with_system(menu::connect::setup_ui),
    )
    .add_system_set(
        SystemSet::on_update(AppState::MenuConnect)
            .with_system(update_matchbox_socket)
            .with_system(menu::connect::btn_visuals)
            .with_system(menu::connect::btn_listeners),
    )
    .add_system_set(
        SystemSet::on_exit(AppState::MenuConnect)
            .with_system(menu::connect::cleanup)
            .with_system(menu::connect::cleanup_ui),
    )
    // win menu
    .add_system_set(SystemSet::on_enter(AppState::Win).with_system(menu::win::setup_ui))
    .add_system_set(
        SystemSet::on_update(AppState::Win)
            .with_system(menu::win::btn_visuals)
            .with_system(menu::win::btn_listeners),
    )
    .add_system_set(SystemSet::on_exit(AppState::Win).with_system(menu::win::cleanup_ui))
    // local round
    .add_system_set(SystemSet::on_enter(AppState::RoundLocal).with_system(setup_game))
    .add_system_set(
        SystemSet::on_update(AppState::RoundLocal)
            .with_system(update_attacker_sprite)
            .with_system(update_defender_sprite),
    )
    .add_system_set(SystemSet::on_exit(AppState::RoundLocal).with_system(cleanup_game))
    // online round
    .add_system_set(SystemSet::on_enter(AppState::RoundOnline).with_system(setup_game))
    .add_system_set(
        SystemSet::on_update(AppState::RoundOnline)
            .with_system(update_attacker_sprite)
            .with_system(update_defender_sprite)
            .with_system(print_p2p_events),
    )
    .add_system_set(SystemSet::on_exit(AppState::RoundOnline).with_system(cleanup_game));
    // ldtk loading TODO: move to assetLoader plugin?
    //.add_startup_system(load_ldtk_level);

    #[cfg(target_arch = "wasm32")]
    {
        app.add_system(bevy_web_resizer::web_resize_system);
    }

    app.run();
}
