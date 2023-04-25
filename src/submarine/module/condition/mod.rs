use bevy::prelude::*;
use uuid::Uuid;

use super::{ModuleStateComponent, ModuleStatus};

pub mod engine_stop;

#[derive(Clone, Component, Debug)]
pub struct ConditionComponent {
    pub id: Uuid,
    pub status: ConditionStatus,
    pub icon: Handle<Image>,
}

impl ConditionComponent {
    fn new(image: Handle<Image>) -> Self {
        Self {
            id: Uuid::new_v4(),
            status: ConditionStatus::Inactive,
            icon: image,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConditionStatus {
    Active,
    Inactive,
}

pub fn update_engine_stop_condition_by_module_state(
    query: Query<(&ModuleStateComponent, &Children), Changed<ModuleStateComponent>>,
    mut condition_query: Query<&mut ConditionComponent>,
) {
    for (state, children) in query.iter() {
        let mut condition_iter = condition_query.iter_many_mut(children);
        while let Some(mut condition_state) = condition_iter.fetch_next() {
            let status = match state.state.status() {
                ModuleStatus::Passive => ConditionStatus::Inactive,
                ModuleStatus::StartingUp => ConditionStatus::Inactive,
                ModuleStatus::Active => ConditionStatus::Inactive,
                ModuleStatus::ActiveInvalidTrigger => ConditionStatus::Inactive,
                ModuleStatus::Triggered => ConditionStatus::Active,
                ModuleStatus::Aftercast => ConditionStatus::Active,
                ModuleStatus::ShuttingDown => ConditionStatus::Inactive,
                ModuleStatus::Inactive => ConditionStatus::Inactive,
            };

            condition_state.status = status;
        }
    }
}
