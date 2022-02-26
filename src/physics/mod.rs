//! simplified version of bevy_xpbd

use bevy::prelude::*;
use systems::*;

use resources::*;

mod bundle;
pub mod components;
mod contact;
mod resources;
mod systems;
mod utils;

pub struct PhysicsPlugin;

pub const PIXELS_PER_METER: f32 = 24.0 / 1.8; // assuming janitor is 1.80 tall and 24 pixels tall

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gravity>()
            // .init_resource::<LoopState>() // todo: for substepping
            // These resources are cleared at the start of every physics frame, so they should be rollback safe
            // i.e. they do not need to be added as rollback resources.
            .init_resource::<CollisionPairs>()
            .init_resource::<Contacts>()
            .init_resource::<StaticContacts>();

        // Normally, we would add the stage here, but since we're doing rollback, we will just do it in main instead
    }
}

/// re-exports of things needed to to use the physics module
pub mod prelude {
    pub use super::{
        bundle::*,
        components::{BoxCollider, Pos, Vel},
        resources::Gravity,
        PhysicsPlugin,
    };
}

pub const DELTA_TIME: f32 = 1. / 60.;
pub const NUM_SUBSTEPS: u32 = 1; // todo
pub const SUB_DT: f32 = DELTA_TIME / NUM_SUBSTEPS as f32;
/// Safety margin bigger than DELTA_TIME added to AABBs to account for sudden accelerations
const COLLISION_PAIR_VEL_MARGIN_FACTOR: f32 = 2. * DELTA_TIME;

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
enum Step {
    ComputeAabbs,
    CollectCollisionPairs,
    Integrate,
    SolvePositions,
    UpdateVelocities,
    SolveVelocities,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub struct PhysicsUpdateStage;

pub fn create_physics_stage() -> SystemStage {
    SystemStage::parallel()
        // todo: needed if we implement substepping
        // .with_run_criteria(run_criteria)
        .with_system_set(
            SystemSet::new()
                .label(Step::ComputeAabbs)
                .before(Step::CollectCollisionPairs)
                .with_system(update_aabb_box)
                .with_system(update_aabb_ball),
        )
        .with_system(
            collect_collision_pairs
                // .with_run_criteria(first_substep) // todo: for substepping
                .label(Step::CollectCollisionPairs)
                .before(Step::Integrate),
        )
        .with_system_set(
            SystemSet::new()
                .label(Step::Integrate)
                .with_system(integrate),
            // .with_system(integrate_rot), // todo: for rotation
        )
        .with_system(clear_contacts.before(Step::SolvePositions))
        .with_system_set(
            SystemSet::new()
                .label(Step::SolvePositions)
                .after(Step::Integrate)
                .with_system(solve_pos_ball_ball)
                .with_system(solve_pos_box_box)
                .with_system(solve_pos_static_ball_ball)
                .with_system(solve_pos_static_box_ball)
                .with_system(solve_pos_static_box_box),
        )
        .with_system_set(
            SystemSet::new()
                .label(Step::UpdateVelocities)
                .after(Step::SolvePositions)
                .with_system(update_vel),
            // .with_system(update_ang_vel), todo: for rotation
        )
        .with_system_set(
            SystemSet::new()
                .label(Step::SolveVelocities)
                .after(Step::UpdateVelocities)
                .with_system(solve_vel)
                .with_system(solve_vel_statics),
        )
        .with_system(
            sync_transforms
                // .with_run_criteria(last_substep) // todo: substepping
                .after(Step::SolveVelocities),
        )
}

// Stuff for substepping, if we enable that:

// #[derive(Debug, Default)]
// struct LoopState {
//     has_added_time: bool,
//     accumulator: f32,
//     substepping: bool,
//     current_substep: u32,
// }

// fn run_criteria(time: Res<Time>, mut state: ResMut<LoopState>) -> ShouldRun {
//     if !state.has_added_time {
//         state.has_added_time = true;
//         state.accumulator += time.delta_seconds();
//     }

//     if state.substepping {
//         state.current_substep += 1;

//         if state.current_substep < NUM_SUBSTEPS {
//             return ShouldRun::YesAndCheckAgain;
//         } else {
//             // We finished a whole step
//             state.accumulator -= DELTA_TIME;
//             state.current_substep = 0;
//             state.substepping = false;
//         }
//     }

//     if state.accumulator >= DELTA_TIME {
//         state.substepping = true;
//         state.current_substep = 0;
//         ShouldRun::YesAndCheckAgain
//     } else {
//         state.has_added_time = false;
//         ShouldRun::No
//     }
// }

// fn first_substep(state: Res<LoopState>) -> ShouldRun {
//     if state.current_substep == 0 {
//         ShouldRun::Yes
//     } else {
//         ShouldRun::No
//     }
// }

// fn last_substep(state: Res<LoopState>) -> ShouldRun {
//     if state.current_substep == NUM_SUBSTEPS - 1 {
//         ShouldRun::Yes
//     } else {
//         ShouldRun::No
//     }
// }
