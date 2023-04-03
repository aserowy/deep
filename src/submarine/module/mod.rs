use bevy::prelude::*;
use std::fmt::Display;
use uuid::Uuid;

use super::{
    power::{PowerCapacitorComponent, PowerUsageComponent},
    settings::{KeyAction, KeyActionEvent},
};

pub mod action;
pub mod engine;

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
pub struct ModuleShutdownComponent {
    pub spindown_time: f32,
    pub current_spindown_time: Option<f32>,
}

#[derive(Component)]
pub struct ModuleStartupComponent {
    pub spinup_time: f32,
    pub current_spinup_time: Option<f32>,
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
            (ModuleStatus::StartingUp, ModuleStatus::Active) => true,
            (ModuleStatus::Active, ModuleStatus::Triggered) => true,
            (ModuleStatus::Active, ModuleStatus::ShuttingDown) => true,
            (ModuleStatus::Triggered, ModuleStatus::Active) => true,
            (ModuleStatus::Triggered, ModuleStatus::ShuttingDown) => true,
            (ModuleStatus::ShuttingDown, ModuleStatus::Inactive) => true,
            (ModuleStatus::Inactive, ModuleStatus::StartingUp) => true,
            (_, _) => false,
        };

        if next {
            self.status = future;
        } else {
            error!(
                "ModuleState next(future) with {} while in status {} is invalid!",
                future, self.status
            )
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    Passive,
    StartingUp,
    Active,
    Triggered,
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
            ModuleStatus::ShuttingDown => "ShuttingDown",
            ModuleStatus::Inactive => "Inactive",
        })?;

        Ok(())
    }
}

pub fn update_module_shutdown_state_transition_with_shutdown_component(
    time: Res<Time>,
    mut query: Query<(&mut ModuleStateComponent, &mut ModuleShutdownComponent)>,
) {
    let dt = time.delta_seconds();

    for (mut state_component, mut spinup_component) in query.iter_mut() {
        if state_component.state.status() != &ModuleStatus::ShuttingDown {
            continue;
        }

        if let Some(spinup_time) = spinup_component.current_spindown_time {
            let spinup_time = spinup_time - dt;
            if spinup_time > 0.0 {
                spinup_component.current_spindown_time = Some(spinup_time);
            } else {
                spinup_component.current_spindown_time = None;
                state_component.state.next(ModuleStatus::Inactive);
            }
        } else {
            spinup_component.current_spindown_time = Some(spinup_component.spindown_time);
        }
    }
}

pub fn update_module_shutdown_state_transition(
    mut query: Query<&mut ModuleStateComponent, Without<ModuleShutdownComponent>>,
) {
    for mut state_component in query.iter_mut() {
        if state_component.state.status() == &ModuleStatus::ShuttingDown {
            state_component.state.next(ModuleStatus::Inactive);
        }
    }
}

pub fn update_module_startup_state_transition_with_startup_component(
    time: Res<Time>,
    mut query: Query<(&mut ModuleStateComponent, &mut ModuleStartupComponent)>,
) {
    let dt = time.delta_seconds();

    for (mut state_component, mut spinup_component) in query.iter_mut() {
        if state_component.state.status() != &ModuleStatus::StartingUp {
            continue;
        }

        if let Some(spinup_time) = spinup_component.current_spinup_time {
            let spinup_time = spinup_time - dt;
            if spinup_time > 0.0 {
                spinup_component.current_spinup_time = Some(spinup_time);
            } else {
                spinup_component.current_spinup_time = None;
                state_component.state.next(ModuleStatus::Active);
            }
        } else {
            spinup_component.current_spinup_time = Some(spinup_component.spinup_time);
        }
    }
}

pub fn update_module_startup_state_transition(
    mut query: Query<&mut ModuleStateComponent, Without<ModuleStartupComponent>>,
) {
    for mut state_component in query.iter_mut() {
        if state_component.state.status() == &ModuleStatus::StartingUp {
            state_component.state.next(ModuleStatus::Active);
        }
    }
}

pub fn trigger_module_status_triggered_on_key_action_event(
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    query: Query<&Children, With<Camera>>,
    mut child_query: Query<&mut ModuleStateComponent>,
) {
    if let Ok(children) = query.get_single() {
        for key_action_event in key_action_event_reader.iter() {
            let component_index = match &key_action_event.key_map.key_action {
                KeyAction::ModuleActivation01 => Some(0),
                KeyAction::ModuleActivation02 => Some(1),
                _ => None,
            };

            if let Some(index) = component_index {
                let mut current_index = 0;
                let mut child_iter = child_query.iter_many_mut(children);
                while let Some(mut state_component) = child_iter.fetch_next() {
                    if current_index == index {
                        let next_default = match state_component.state.status() {
                            ModuleStatus::Active => Some(ModuleStatus::Triggered),
                            ModuleStatus::Triggered => Some(ModuleStatus::Active),
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

pub fn update_power_capacity_component_by_module_power_usage(
    mut query: Query<(&mut PowerCapacitorComponent, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut PowerUsageComponent)>,
) {
    for (mut capacitor, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state_component, mut usage)) = child_iter.fetch_next() {
            if capacitor.capacity < usage.usage {
                state_component.state.next(ModuleStatus::ShuttingDown);
            } else {
                capacitor.capacity -= usage.usage;
            }

            usage.usage = 0.0;
        }
    }
}
