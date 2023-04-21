use bevy::prelude::*;

use super::{ModuleStateComponent, ModuleStatus};

pub mod engine_stop;

#[derive(Clone, Component, Debug)]
pub struct ConditionStateComponent {
    pub status: ConditionStatus,
}

impl Default for ConditionStateComponent {
    fn default() -> Self {
        Self {
            status: ConditionStatus::Inactive,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionStatus {
    Active,
    Inactive,
}

pub fn update_engine_stop_condition_by_module_state(
    mut query: Query<
        (&ModuleStateComponent, &mut ConditionStateComponent),
        Changed<ModuleStateComponent>,
    >,
) {
    for (state, mut condition_state) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Passive => condition_state.status = ConditionStatus::Inactive,
            ModuleStatus::StartingUp => condition_state.status = ConditionStatus::Inactive,
            ModuleStatus::Active => condition_state.status = ConditionStatus::Inactive,
            ModuleStatus::ActiveInvalidTrigger => condition_state.status = ConditionStatus::Inactive,
            ModuleStatus::Triggered => condition_state.status = ConditionStatus::Active,
            ModuleStatus::Aftercast => condition_state.status = ConditionStatus::Active,
            ModuleStatus::ShuttingDown => condition_state.status = ConditionStatus::Inactive,
            ModuleStatus::Inactive => condition_state.status = ConditionStatus::Inactive,
        }
    }
}
