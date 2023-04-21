use bevy::prelude::*;

use crate::submarine::height::HeightPropertyComponent;

use super::{ModuleStateComponent, ModuleStatus};

#[derive(Clone, Component, Debug)]
pub struct RequirementStateComponent {
    pub status: RequirementStatus,
}

impl Default for RequirementStateComponent {
    fn default() -> Self {
        Self {
            status: RequirementStatus::Fulfilled,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RequirementStatus {
    Fulfilled,
    Violated,
}

#[derive(Clone, Component)]
pub struct MaximumHeightRequirementComponent {
    pub maximum_height: f32,
}

pub fn set_module_state_by_requirement_states(
    mut query: Query<(&mut ModuleStateComponent, &Children)>,
    requirement_query: Query<&RequirementStateComponent>,
) {
    for (mut state, children) in query.iter_mut() {
        if state.state.status() != &ModuleStatus::Active
            && state.state.status() != &ModuleStatus::ActiveInvalidTrigger
        {
            continue;
        }

        let mut requirements_met = true;
        for requirement in requirement_query.iter_many(children) {
            if requirement.status == RequirementStatus::Violated {
                requirements_met = false;
            }
        }

        match (requirements_met, state.state.status()) {
            (true, ModuleStatus::ActiveInvalidTrigger) => state.state.next(ModuleStatus::Active),
            (false, ModuleStatus::Active) => state.state.next(ModuleStatus::ActiveInvalidTrigger),
            _ => (),
        }
    }
}

pub fn handle_maximum_height_requirement(
    mut query: Query<(
        &Parent,
        &mut RequirementStateComponent,
        &MaximumHeightRequirementComponent,
    )>,
    parent_query: Query<&Parent>,
    height_property_query: Query<&HeightPropertyComponent>,
) {
    for (parent, mut state, requirement) in query.iter_mut() {
        if let Ok(module_parent) = parent_query.get(parent.get()) {
            match height_property_query
                .get_component::<HeightPropertyComponent>(module_parent.get())
            {
                Ok(height_property) => {
                    if height_property.current_height <= requirement.maximum_height {
                        state.status = RequirementStatus::Fulfilled;
                    } else {
                        state.status = RequirementStatus::Violated;
                    }
                }
                Err(_) => {
                    warn!("module parent does not have HeightPropertyComponent!")
                }
            }
        }
    }
}
