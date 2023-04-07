use bevy::prelude::*;
use bevy_rapier3d::prelude::AdditionalMassProperties;
use std::fmt::Display;
use uuid::Uuid;

use super::{
    power::{PowerCapacitorComponent, PowerUsageComponent},
    settings::{KeyAction, KeyActionEvent, KeyPress},
};

pub mod action;
pub mod aftercast;
pub mod engine;
pub mod shutdown;
pub mod startup;

#[derive(Bundle)]
pub struct ModuleBundle {
    pub details: ModuleDetailsComponent,
    pub state: ModuleStateComponent,
}

#[derive(Component)]
pub struct ModuleDetailsComponent {
    pub id: Uuid,
    pub icon: String,
}

#[derive(Component)]
pub struct ModuleMassComponent {
    pub initialized: bool,
    pub mass: f32,
}

#[derive(Component)]
pub struct ModuleStateComponent {
    pub state: ModuleState,
}

pub struct ModuleState {
    status: ModuleStatus,
}

impl ModuleState {
    pub fn new() -> Self {
        Self {
            status: ModuleStatus::Inactive,
        }
    }

    pub fn status(&self) -> &ModuleStatus {
        &self.status
    }

    pub fn next(&mut self, future: ModuleStatus) {
        let next = match (&self.status, &future) {
            (ModuleStatus::Passive, ModuleStatus::Passive) => true,
            (ModuleStatus::StartingUp, ModuleStatus::Active) => true,
            (ModuleStatus::Active, ModuleStatus::Triggered) => true,
            (ModuleStatus::Active, ModuleStatus::ShuttingDown) => true,
            (ModuleStatus::Triggered, ModuleStatus::Aftercast) => true,
            (ModuleStatus::Aftercast, ModuleStatus::Active) => true,
            (ModuleStatus::Triggered, ModuleStatus::ShuttingDown) => true,
            (ModuleStatus::ShuttingDown, ModuleStatus::Inactive) => true,
            (ModuleStatus::Inactive, ModuleStatus::StartingUp) => true,
            (_, _) => false,
        };

        if next {
            info!(
                "ModuleState next({}) while in status {} triggered.",
                future, self.status
            );

            self.status = future;
        } else {
            error!(
                "ModuleState next({}) while in status {} is invalid!",
                future, self.status
            );
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    Passive,
    StartingUp,
    Active,
    Triggered,
    Aftercast,
    ShuttingDown,
    Inactive,
}

impl Display for ModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ModuleStatus::Passive => "Passive",
            ModuleStatus::StartingUp => "StartingUp",
            ModuleStatus::Active => "Active",
            ModuleStatus::Triggered => "Triggered",
            ModuleStatus::Aftercast => "Aftercast",
            ModuleStatus::ShuttingDown => "ShuttingDown",
            ModuleStatus::Inactive => "Inactive",
        })?;

        Ok(())
    }
}

pub fn on_key_action_event(
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    query: Query<&Children, With<Camera>>,
    mut child_query: Query<&mut ModuleStateComponent>,
) {
    if let Ok(children) = query.get_single() {
        for key_action_event in key_action_event_reader.iter() {
            if key_action_event.key_press != KeyPress::Down {
                continue;
            }

            let component_index = match &key_action_event.key_map.key_action {
                KeyAction::ModuleActivation01 => Some(0),
                KeyAction::ModuleActivation02 => Some(1),
                KeyAction::ModuleActivation03 => Some(2),
                _ => None,
            };

            if let Some(index) = component_index {
                let mut current_index = 0;
                let mut child_iter = child_query.iter_many_mut(children);
                while let Some(mut state_component) = child_iter.fetch_next() {
                    if current_index == index {
                        let next_default = match state_component.state.status() {
                            ModuleStatus::Active => Some(ModuleStatus::Triggered),
                            ModuleStatus::Triggered => Some(ModuleStatus::Aftercast),
                            ModuleStatus::Inactive => Some(ModuleStatus::StartingUp),
                            _ => None,
                        };

                        if let Some(state) = next_default {
                            state_component.state.next(state);
                            return;
                        }
                    }

                    current_index += 1;
                }
            }
        }
    }
}

pub fn update_mass_by_module_mass(
    mut query: Query<(&mut AdditionalMassProperties, &Children), Changed<AdditionalMassProperties>>,
    mut child_query: Query<&mut ModuleMassComponent>,
) {
    for (mut additional_mass, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some(mut mass_component) = child_iter.fetch_next() {
            if mass_component.initialized {
                continue;
            }

            if let AdditionalMassProperties::Mass(mass) = *additional_mass {
                let mass = mass + mass_component.mass;

                info!("Mass set to {} kg.", mass);

                *additional_mass = AdditionalMassProperties::Mass(mass);
            }

            mass_component.initialized = true;
        }
    }
}

pub fn update_power_capacity_by_module_power_usage(
    mut query: Query<(&mut PowerCapacitorComponent, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut PowerUsageComponent)>,
) {
    for (mut capacitor, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state_component, mut usage)) = child_iter.fetch_next() {
            let watt_hour = usage.watt_per_second / 3600.0;
            if capacitor.watt_hour < watt_hour {
                state_component.state.next(ModuleStatus::ShuttingDown);
            } else {
                capacitor.watt_hour -= watt_hour;
            }

            usage.watt_per_second = 0.0;
        }
    }
}
