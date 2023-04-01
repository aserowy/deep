use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct PowerCapacitorComponent {
    // TODO: add unit to names
    pub capacity: f32,
    pub capacity_max: f32,
}

#[derive(Component)]
pub struct PowerCoreComponent {
    pub production: f32,
}

#[derive(Component, Default)]
pub struct PowerUsageComponent {
    pub usage: f32,
}

pub fn update_power_capacity_component_by_core(
    time: Res<Time>,
    mut query: Query<(&mut PowerCapacitorComponent, &PowerCoreComponent)>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut capacitor, core)) = query.get_single_mut() {
        capacitor.capacity += core.production * dt;

        if capacitor.capacity > capacitor.capacity_max {
            capacitor.capacity = capacitor.capacity_max;
        }
    }
}
