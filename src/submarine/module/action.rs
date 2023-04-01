use bevy::prelude::*;

use crate::submarine::settings::{KeyAction, KeyActionEvent};

use super::{ModuleStateComponent, ModuleStatus};

#[derive(Clone)]
pub enum ModuleAction {
    MiningMagnatide,
    ResourceScan,
}

#[derive(Clone, Component)]
pub struct ActionComponent {}

pub fn trigger_module_action_on_key_action_event(
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    // mut player: ResMut<PlayerSubmarineResource>,
) {
    for key_action_event in key_action_event_reader.iter() {
        // let mut module: Option<&mut ActionModule> = None;

        match &key_action_event.key_map.key_action {
            KeyAction::ModuleActivation01 => {
                // module = get_module_action_by_position(&mut player.modules, 0);
            }
            KeyAction::ModuleActivation02 => {
                // module = get_module_action_by_position(&mut player.modules, 1);
            }
            _ => (),
        }

        /* if let Some(activated_module) = module {
            trigger_module_activation(activated_module);
        } */
    }
}

pub fn handle_module_state_for_actions(
    mut query: Query<(&mut ModuleStateComponent, &mut ActionComponent)>,
) {
    for (mut state, _action) in query.iter_mut() {
        match state.status {
            ModuleStatus::Startup => (),
            ModuleStatus::Active => (),
            ModuleStatus::Triggered => (),
            ModuleStatus::Shutdown => (),
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
