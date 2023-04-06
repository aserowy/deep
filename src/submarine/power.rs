use bevy::prelude::*;

#[derive(Clone, Component)]
pub struct PowerCapacitorComponent {
    // TODO: add unit to names
    pub watt_hour: f32,
    pub watt_hour_max: f32,
}

#[derive(Component)]
pub struct PowerCoreComponent {
    pub watt_per_second: f32,
}

#[derive(Component, Default)]
pub struct PowerUsageComponent {
    pub watt_per_second: f32,
}

pub fn update_capacity_by_core(
    time: Res<Time>,
    mut query: Query<(&mut PowerCapacitorComponent, &PowerCoreComponent)>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut capacitor, core)) = query.get_single_mut() {
        // INFO: conversion of watt to watt hour
        capacitor.watt_hour += core.watt_per_second * dt / 3600.0;

        if capacitor.watt_hour > capacitor.watt_hour_max {
            capacitor.watt_hour = capacitor.watt_hour_max;
        }
    }
}
