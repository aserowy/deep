use bevy::prelude::*;

use super::{ModuleStateComponent, ModuleStatus};

/* #[derive(Clone)]
pub enum Action {
    MiningMagnatide,
    ResourceScan,
} */

#[derive(Clone, Component)]
pub struct ActionComponent {}

pub fn handle_module_state_for_actions(
    mut query: Query<(&mut ModuleStateComponent, &mut ActionComponent)>,
) {
    for (state, _action) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Triggered => (),
            _ => (),
        }
    }
}
