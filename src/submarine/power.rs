use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct PowerCapacitorComponent {
    // TODO: add unit to names
    pub enabled: bool,
    pub initialized: bool,
    pub capacity: f32,
    pub capacity_max: f32,
}

#[derive(Component)]
pub struct PowerCoreComponent {
    pub production: f32,
}

pub struct PowerCapacitorChangedEvent(pub PowerCapacitorComponent);

// TODO: calculate current consumption
pub struct PowerConsumptionChangedEvent(pub f32);

pub fn update_power_capacity_component_by_core(
    time: Res<Time>,
    mut power_capacitor_event_writer: EventWriter<PowerCapacitorChangedEvent>,
    mut query: Query<(&mut PowerCapacitorComponent, &PowerCoreComponent), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut capacitor, core)) = query.get_single_mut() {
        if !capacitor.enabled {
            return;
        }

        if !capacitor.initialized {
            capacitor.initialized = true;
            power_capacitor_event_writer.send(PowerCapacitorChangedEvent(capacitor.clone()));
        }

        let current_capacity = capacitor.capacity;

        capacitor.capacity += core.production * dt;

        if capacitor.capacity > capacitor.capacity_max {
            capacitor.capacity = capacitor.capacity_max;
        }

        // NOTE: this brings some blur into current capacity, because the shown value is set before
        // consumption through engine and modules [maybe add this to 'calculate current consumption']
        if current_capacity != capacitor.capacity {
            power_capacitor_event_writer.send(PowerCapacitorChangedEvent(capacitor.clone()));
        }
    }
}
