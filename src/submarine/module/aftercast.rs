use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct ModuleAftercastComponent {
    pub spindown_time: f32,
    pub current_spindown_time: Option<f32>,
}

pub fn update_module_aftercast_state_transition_with_aftercast_component(
    time: Res<Time>,
    mut query: Query<(&mut ModuleStateComponent, &mut ModuleAftercastComponent)>,
) {
    let dt = time.delta_seconds();

    for (mut state_component, mut spinup_component) in query.iter_mut() {
        if state_component.state.status() != &ModuleStatus::Aftercast {
            continue;
        }

        if let Some(spinup_time) = spinup_component.current_spindown_time {
            let spinup_time = spinup_time - dt;
            if spinup_time > 0.0 {
                spinup_component.current_spindown_time = Some(spinup_time);
            } else {
                spinup_component.current_spindown_time = None;
                state_component.state.next(ModuleStatus::Active);
            }
        } else {
            spinup_component.current_spindown_time = Some(spinup_component.spindown_time);
        }
    }
}

pub fn update_module_aftercast_state_transition(
    mut query: Query<&mut ModuleStateComponent, Without<ModuleAftercastComponent>>,
) {
    for mut state_component in query.iter_mut() {
        if state_component.state.status() == &ModuleStatus::Aftercast {
            state_component.state.next(ModuleStatus::Active);
        }
    }
}
