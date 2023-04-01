use bevy::prelude::*;

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
    pub status: ModuleStatus,
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
