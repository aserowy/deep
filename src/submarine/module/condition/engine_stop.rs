use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::submarine::module::engine::EngineComponent;

use super::{ConditionStateComponent, ConditionStatus};

#[derive(Clone, Component, Default)]
pub struct EngineStopConditionComponent {}

pub fn update_engine_by_engine_stop_condition(
    query: Query<(
        &Parent,
        &ConditionStateComponent,
        &EngineStopConditionComponent,
    )>,
    mut children_query: Query<(&mut ExternalForce, &Children)>,
    mut engine_query: Query<&mut EngineComponent>,
) {
    for (parent, state, condition) in query.iter() {
        if state.status == ConditionStatus::Inactive {
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
