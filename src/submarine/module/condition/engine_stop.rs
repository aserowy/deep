use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::submarine::module::engine::EngineComponent;

use super::{ConditionComponent, ConditionStatus};

#[derive(Clone, Component, Default)]
pub struct EngineStopConditionComponent {}

impl EngineStopConditionComponent {
    pub fn new(asset_server: &Res<AssetServer>, builder: &mut ChildBuilder) {
        builder.spawn((
            ConditionComponent::new(asset_server.load("submarine/condition/stop-sign_33px.png")),
            EngineStopConditionComponent::default(),
        ));
    }
}

pub fn update_engine_by_engine_stop_condition(
    query: Query<(&Parent, &ConditionComponent), With<EngineStopConditionComponent>>,
    parent_query: Query<&Parent>,
    mut force_query: Query<(&mut ExternalForce, &Children)>,
    mut engine_query: Query<&mut EngineComponent>,
) {
    for (parent, state) in query.iter() {
        if state.status == ConditionStatus::Inactive {
            continue;
        }

        set_engine_stop(&mut force_query, &mut engine_query, parent.get());

        if let Ok(parent_parent) = parent_query.get(parent.get()) {
            set_engine_stop(&mut force_query, &mut engine_query, parent_parent.get());
        }
    }
}

fn set_engine_stop(
    force_query: &mut Query<(&mut ExternalForce, &Children)>,
    engine_query: &mut Query<&mut EngineComponent>,
    parent: Entity,
) {
    if let Ok((mut force, children)) = force_query.get_mut(parent) {
        let mut child_iter = engine_query.iter_many_mut(children);
        while let Some(mut engine) = child_iter.fetch_next() {
            engine.set_stop_with_force(&mut force);
        }
    }
}
