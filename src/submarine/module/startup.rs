use bevy::prelude::*;

use super::*;

#[derive(Component)]
pub struct ModuleStartupComponent {
    // TODO: correct naming wps is watt and watt is Wh and current_watt is current Wh
    pub watt_per_second: f32,
    pub watt: f32,
    pub current_watt: Option<f32>,
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

            if usage.current_watt.is_none() {
                usage.current_watt = Some(usage.watt);
            }

            if let Some(power_needed) = usage.current_watt {
                let consumption_max = usage.watt_per_second * dt;
                let capacity_watt_seconds = capacitor.watt_hour * 3600.0;

                if capacity_watt_seconds > consumption_max && power_needed > consumption_max {
                    usage.current_watt = Some(power_needed - consumption_max);
                    capacitor.watt_hour -= consumption_max / 3600.0;
                } else if capacity_watt_seconds < consumption_max && power_needed > capacitor.watt_hour
                {
                    usage.current_watt = Some(power_needed - capacity_watt_seconds);
                    capacitor.watt_hour = 0.0;
                } else {
                    usage.current_watt = Some(0.0);
                    capacitor.watt_hour -= power_needed / 3600.0;
                }

                if power_needed <= 0.1 {
                    usage.current_watt = None;
                    state_component.state.next(ModuleStatus::Active);
                }
            }
        }
    }
}
