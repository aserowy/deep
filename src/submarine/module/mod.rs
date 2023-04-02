use bevy::prelude::*;
use std::fmt::Display;

use super::power::{PowerCapacitorComponent, PowerUsageComponent};

pub mod action;
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
            (ModuleStatus::Startup, ModuleStatus::Active) => true,
            (ModuleStatus::Active, ModuleStatus::Triggered) => true,
            (ModuleStatus::Active, ModuleStatus::Shutdown) => true,
            (ModuleStatus::Triggered, ModuleStatus::Active) => true,
            (ModuleStatus::Triggered, ModuleStatus::Shutdown) => true,
            (ModuleStatus::Shutdown, ModuleStatus::Inactive) => true,
            (ModuleStatus::Inactive, ModuleStatus::Startup) => true,
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
    Startup,
    Active,
    Triggered,
    Shutdown,
    Inactive,
}

impl Display for ModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            ModuleStatus::Passive => "Passive",
            ModuleStatus::Startup => "Startup",
            ModuleStatus::Active => "Active",
            ModuleStatus::Triggered => "Triggered",
            ModuleStatus::Shutdown => "Shutdown",
            ModuleStatus::Inactive => "Inactive",
        })?;

        Ok(())
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
                state_component.state.next(ModuleStatus::Shutdown);
            } else {
                capacitor.capacity -= usage.usage;
            }

            usage.usage = 0.0;
        }
    }
}
