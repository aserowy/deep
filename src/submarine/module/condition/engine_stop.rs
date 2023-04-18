use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::submarine::module::{engine::EngineComponent, ModuleStateComponent, ModuleStatus};

#[derive(Clone, Component, Default)]
pub struct EngineStopConditionComponent {
    pub is_active: bool,
}

pub fn update_engine_stop_condition_by_module_state(
    mut query: Query<
        (&ModuleStateComponent, &mut EngineStopConditionComponent),
        Changed<ModuleStateComponent>,
    >,
) {
    for (state, mut condition) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Passive => condition.is_active = false,
            ModuleStatus::StartingUp => condition.is_active = false,
            ModuleStatus::Active => condition.is_active = false,
            ModuleStatus::Triggered => condition.is_active = true,
            ModuleStatus::Aftercast => condition.is_active = true,
            ModuleStatus::ShuttingDown => condition.is_active = false,
            ModuleStatus::Inactive => condition.is_active = false,
        }
    }
}

pub fn update_engine_by_engine_stop_condition(
    query: Query<(&Parent, &EngineStopConditionComponent)>,
    mut children_query: Query<(&mut ExternalForce, &Children)>,
    mut engine_query: Query<&mut EngineComponent>,
) {
    for (parent, condition) in query.iter() {
        if !condition.is_active {
            continue;
        }

        if let Ok((mut force, children)) = children_query.get_mut(parent.get()) {
            let mut child_iter = engine_query.iter_many_mut(children);
            while let Some(mut engine) = child_iter.fetch_next() {
                engine.set_stop_with_force(&mut force);
            }
        }
    }
}
