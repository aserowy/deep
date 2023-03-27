use std::f32::consts::E;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

#[derive(Component)]
pub struct SettingsComponent {
    pub enabled: bool,
    pub movement_spot: f32,
    pub key_thrust_positiv: KeyCode,
    pub key_thrust_negative: KeyCode,
    pub key_thrust_zero: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self {
            enabled: true,
            movement_spot: 125.0,
            key_thrust_positiv: KeyCode::W,
            key_thrust_negative: KeyCode::S,
            key_thrust_zero: KeyCode::Q,
            key_up: KeyCode::D,
            key_down: KeyCode::A,
        }
    }
}

#[derive(Component)]
pub struct ThrustComponent {
    pub enabled: bool,
    pub forward_thrust: f32,
    pub forward_thrust_max: f32,
    pub upward_thrust: f32,
    pub upward_thrust_max: f32,
    pub nose_thrust: f32,
    pub nose_thrust_max: f32,
    pub spin_thrust: f32,
    pub spin_thrust_max: f32,
}

impl Default for ThrustComponent {
    fn default() -> Self {
        Self {
            enabled: true,
            forward_thrust: 0.0,
            forward_thrust_max: 2500.0,
            upward_thrust: 0.0,
            upward_thrust_max: 1000.0,
            nose_thrust: 0.0,
            nose_thrust_max: 500.0,
            spin_thrust: 0.0,
            spin_thrust_max: 500.0,
        }
    }
}

pub fn control_translation(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Transform, &mut ThrustComponent, &SettingsComponent), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut force, transform, mut thrust, settings)) = query.get_single_mut() {
        if !thrust.enabled {
            return;
        }

        let current_forward_thrust = thrust.forward_thrust;
        let current_upward_thrust = thrust.upward_thrust;

        if key_input.pressed(settings.key_thrust_positiv) {
            thrust.forward_thrust += 1750.0 * dt;
        }

        if key_input.pressed(settings.key_thrust_negative) {
            thrust.forward_thrust -= 1750.0 * dt;
        }

        if thrust.forward_thrust.abs() > thrust.forward_thrust_max {
            let coefficient = if thrust.forward_thrust > 0.0 {
                1.0
            } else {
                -1.0
            };

            thrust.forward_thrust = thrust.forward_thrust_max * coefficient;
        }

        if key_input.pressed(settings.key_thrust_zero) {
            thrust.forward_thrust = 0.0;
        }

        if key_input.pressed(settings.key_up) {
            thrust.upward_thrust = thrust.upward_thrust_max;
        }

        if key_input.pressed(settings.key_down) {
            thrust.upward_thrust = thrust.upward_thrust_max * -1.0;
        }

        if key_input.just_released(settings.key_up) || key_input.just_released(settings.key_down) {
            thrust.upward_thrust = 0.0;
        }

        if thrust.forward_thrust != current_forward_thrust
            || thrust.upward_thrust != current_upward_thrust
        {
            force.force =
                get_current_force(&transform, thrust.forward_thrust, thrust.upward_thrust);
        }
    }
}

fn get_current_force(transform: &Transform, forward_thrust: f32, upward_thrust: f32) -> Vec3 {
    let forward = transform.forward().normalize();
    let upward = transform.up().normalize();

    forward * forward_thrust + upward * upward_thrust
}

pub fn control_axis_rotation(
    windows: Query<&Window>,
    mut query: Query<(&mut ExternalForce, &mut Transform, &mut ThrustComponent, &SettingsComponent), With<Camera>>,
) {
    if let Ok((mut force, transform, mut thrust, settings)) = query.get_single_mut() {
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            force.force =
                get_current_force(&transform, thrust.forward_thrust, thrust.upward_thrust);

            let current_spin_thrust = thrust.spin_thrust;
            let current_nose_thrust = thrust.nose_thrust;

            thrust.spin_thrust = thrust.spin_thrust_max
                * get_torque_coefficient(cursor_position.x, window.width(), settings.movement_spot);

            thrust.nose_thrust = thrust.nose_thrust_max
                * get_torque_coefficient(cursor_position.y, window.height(), settings.movement_spot);

            if thrust.spin_thrust != current_spin_thrust
                || thrust.nose_thrust != current_nose_thrust
            {
                force.torque = transform.forward().normalize() * thrust.spin_thrust
                    + transform.left().normalize() * thrust.nose_thrust;
            }
        }
    }
}

fn get_torque_coefficient(position: f32, domain: f32, movement_spot: f32) -> f32 {
    let blind_spot = 5.0;

    let relative = domain * 0.5 - position;
    let relative_abs = relative.abs();
    let coefficient = if relative < 0.0 { 1.0 } else { -1.0 };

    if relative_abs >= movement_spot {
        1.0 * coefficient
    } else if relative_abs <= blind_spot {
        0.0
    } else {
        // logistic function
        1.0 / (1.0
            + E.powf(5.0 - 10.0 * (relative_abs - blind_spot) / (movement_spot - blind_spot)))
            * coefficient
    }
}
