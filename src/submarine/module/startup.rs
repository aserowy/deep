use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct ModuleStartupComponent {
    pub watt: f32,
    pub watt_hour: f32,
    pub remaining_watt_hour: Option<f32>,
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

            if usage.remaining_watt_hour.is_none() {
                usage.remaining_watt_hour = Some(usage.watt_hour);
            }

            if let Some(remaining_watt_hour) = usage.remaining_watt_hour {
                let consumption_max = usage.watt * dt / 3600.0;

                if capacitor.watt_hour > consumption_max && remaining_watt_hour > consumption_max {
                    usage.remaining_watt_hour = Some(remaining_watt_hour - consumption_max);
                    capacitor.watt_hour -= consumption_max;
                } else if capacitor.watt_hour < consumption_max
                    && remaining_watt_hour > capacitor.watt_hour
                {
                    usage.remaining_watt_hour = Some(remaining_watt_hour - capacitor.watt_hour);
                    capacitor.watt_hour = 0.0;
                } else {
                    usage.remaining_watt_hour = Some(0.0);
                    capacitor.watt_hour -= remaining_watt_hour;
                }

                if remaining_watt_hour <= 0.1 {
                    usage.remaining_watt_hour = None;
                    state_component.state.next(ModuleStatus::Active);
                }
            }
        }
    }
}
