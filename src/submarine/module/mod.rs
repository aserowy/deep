use bevy::prelude::*;

use super::{
    power::{PowerCapacitorComponent, PowerUsageComponent},
    settings::*,
    PlayerSubmarineResource,
};

pub mod engine;

#[derive(Bundle)]
pub struct ModuleBundle {
    pub details: ModuleDetailsComponent,
    pub state: ModuleStateComponent,
}

#[derive(Component)]
pub struct ModuleDetailsComponent {
    pub id: String,
    pub icon: String,
    pub slot: u8,
}

#[derive(Component)]
pub struct ModuleStateComponent {
    pub status: ModuleStatus,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    Startup,
    Active,
    Triggered,
    Shutdown,
    Inactive,
}

#[derive(Clone)]
pub struct ActionModule {
    pub id: String,
    pub icon: String,
    pub action: ModuleAction,
}

impl ActionModule {
    pub fn new_mining_base() -> Self {
        ActionModule {
            id: "mining_base".into(),
            icon: "󰜐".into(),
            action: ModuleAction::MiningMagnatide,
        }
    }

    pub fn new_resource_scanner_base() -> Self {
        ActionModule {
            id: "resource_scanner_base".into(),
            icon: "󰐷".into(),
            action: ModuleAction::ResourceScan,
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
    for key_action_event in key_action_event_reader.iter() {
        let mut module: Option<&mut ActionModule> = None;
        match &key_action_event.key_map.key_action {
            KeyAction::ModuleActivation01 => {
                module = get_module_action_by_position(&mut player.modules, 0);
            }
            KeyAction::ModuleActivation02 => {
                module = get_module_action_by_position(&mut player.modules, 1);
            }
            _ => (),
        }
        if let Some(activated_module) = module {
            trigger_module_activation(activated_module);
        }
    }
}

pub fn update_power_capacity_component_by_module_power_usage(
    mut query: Query<(&mut PowerCapacitorComponent, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut PowerUsageComponent)>,
) {
    for (mut capacitor, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state, mut usage)) = child_iter.fetch_next() {
            if capacitor.capacity < usage.usage {
                state.status = ModuleStatus::Shutdown;
            } else {
                capacitor.capacity -= usage.usage;
            }

            usage.usage = 0.0;
        }
    }
}

fn trigger_module_activation(_module: &mut ActionModule) {
    /* if module.current_cooldown.is_some() {
        return;
    }

    module.current_casttime = Some(module.casttime);
    module.current_cooldown = Some(module.cooldown);

    info!(
        "Module {} activated with {} cooldown",
        module.id, module.cooldown
    ); */
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
}
