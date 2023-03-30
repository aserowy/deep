use bevy::prelude::*;

use super::{settings::*, PlayerSubmarineResource};

#[derive(Component)]
pub struct ModuleComponent;

#[derive(Clone)]
pub struct Module {
    pub id: String,
    pub icon: String,
    pub action: ModuleAction,
    pub cooldown: f32,
    pub current_cooldown: Option<f32>,
}

impl Module {
    pub fn new_mining_base() -> Self {
        Module {
            id: "mining_base".into(),
            icon: "󰜐".into(),
            action: ModuleAction::MiningMagnatide,
            cooldown: 4.0,
            current_cooldown: None,
        }
    }

    pub fn new_resource_scanner_base() -> Self {
        Module {
            id: "resource_scanner_base".into(),
            icon: "󰐷".into(),
            action: ModuleAction::ResourceScan,
            cooldown: 10.0,
            current_cooldown: None,
        }
    }
}

#[derive(Clone)]
pub enum ModuleAction {
    MiningMagnatide,
    ResourceScan,
}

pub fn trigger_module_action_on_key_action_event(
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    mut player: ResMut<PlayerSubmarineResource>,
) {
    let mut module: Option<&mut Module> = None;

    for key_action_event in key_action_event_reader.iter() {
        match &key_action_event.key_map.key_action {
            KeyAction::ModuleActivation01 => {
                module = get_module_action_by_position(&mut player.modules, 0);
            }
            KeyAction::ModuleActivation02 => {
                module = get_module_action_by_position(&mut player.modules, 1);
            }
            _ => (),
        }
    }

    if let Some(activated_module) = module {
        trigger_module_activation(activated_module);
    }
}

fn trigger_module_activation(module: &mut Module) {
    if module.current_cooldown.is_some() {
        return;
    }

    module.current_cooldown = Some(module.cooldown);

    info!(
        "Module {} activated with cooldown {}",
        module.id, module.cooldown
    );
}

fn get_module_action_by_position(modules: &mut Vec<Module>, index: usize) -> Option<&mut Module> {
    if index < modules.len() {
        Some(&mut modules[index])
    } else {
        None
    }
}
