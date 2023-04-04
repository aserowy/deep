use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct ModuleStartupComponent {
    pub power_consumption_max: f32,
    pub power_needed: f32,
    pub current_power_needed: Option<f32>,
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

pub fn update_power_capacity_by_module_startup(
    time: Res<Time>,
    mut query: Query<(&mut PowerCapacitorComponent, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut ModuleStartupComponent)>,
) {
    let dt = time.delta_seconds();

    // INFO: maybe flatten consumption over all startups?
    for (mut capacitor, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state_component, mut usage)) = child_iter.fetch_next() {
            if state_component.state.status() != &ModuleStatus::StartingUp {
                continue;
            }

            if usage.current_power_needed.is_none() {
                usage.current_power_needed = Some(usage.power_needed);
            }

            if let Some(power_needed) = usage.current_power_needed {
                let consumption_max = usage.power_consumption_max * dt;

                if capacitor.capacity > consumption_max && power_needed > consumption_max {
                    usage.current_power_needed = Some(power_needed - consumption_max);
                    capacitor.capacity -= consumption_max;
                } else if capacitor.capacity < consumption_max && power_needed > capacitor.capacity
                {
                    usage.current_power_needed = Some(power_needed - capacitor.capacity);
                    capacitor.capacity = 0.0;
                } else {
                    usage.current_power_needed = Some(0.0);
                    capacitor.capacity -= power_needed;
                }

                if power_needed <= 0.1 {
                    usage.current_power_needed = None;
                    state_component.state.next(ModuleStatus::Active);
                }
            }
        }
    }
}
