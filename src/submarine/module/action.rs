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
    for (mut state, _action) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Passive => (),
            ModuleStatus::Starting => (),
            ModuleStatus::Active => (),
            ModuleStatus::Triggered => (),
            ModuleStatus::ShuttingDown => (),
            ModuleStatus::Inactive => (),
        }
    }
}
/*
#[derive(Clone)]
pub struct ActionModule {
    pub id: String,
    pub icon: String,
    pub action: ModuleAction,
}

impl ActionModule {
}


fn trigger_module_activation(_module: &mut ActionModule) {
    if module.current_cooldown.is_some() {
        return;
    }

    module.current_casttime = Some(module.casttime);
    module.current_cooldown = Some(module.cooldown);

    info!(
        "Module {} activated with {} cooldown",
        module.id, module.cooldown
    );
}

fn get_module_action_by_position(
    modules: &mut Vec<ActionModule>,
    index: usize,
) -> Option<&mut ActionModule> {
    if index < modules.len() {
        Some(&mut modules[index])
    } else {
        None
    }
} */
