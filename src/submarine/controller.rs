use std::f32::consts::E;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub movement_spot: f32,
    pub key_thrust_positiv: KeyCode,
    pub key_thrust_negative: KeyCode,
    pub key_thrust_zero: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub forward_thrust: f32,
    pub forward_thrust_max: f32,
    pub upward_thrust: f32,
    pub upward_thrust_max: f32,
}

// TODO: split movement options and submarine properties (run_speed, velocity, friction)
impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            movement_spot: 250.0,
            key_thrust_positiv: KeyCode::W,
            key_thrust_negative: KeyCode::S,
            key_thrust_zero: KeyCode::Q,
            key_up: KeyCode::D,
            key_down: KeyCode::A,
            forward_thrust: 0.0,
            forward_thrust_max: 2500.0,
            upward_thrust: 0.0,
            upward_thrust_max: 1000.0,
        }
    }
}

pub fn control_translation(
    time: Res<Time>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut force, transform, mut options)) = query.get_single_mut() {
        if !options.enabled {
            return;
        }

        let current_forward_thrust = options.forward_thrust;
        let current_upward_thrust = options.upward_thrust;

        if key_input.pressed(options.key_thrust_positiv) {
            options.forward_thrust += 1750.0 * dt;
        }

        if key_input.pressed(options.key_thrust_negative) {
            options.forward_thrust -= 1750.0 * dt;
        }

        if options.forward_thrust.abs() > options.forward_thrust_max {
            let coefficient = if options.forward_thrust > 0.0 {
                1.0
            } else {
                -1.0
            };

            options.forward_thrust = options.forward_thrust_max * coefficient;
        }

        if key_input.pressed(options.key_thrust_zero) {
            options.forward_thrust = 0.0;
        }

        if key_input.pressed(options.key_up) {
            options.upward_thrust = options.upward_thrust_max;
        }

        if key_input.pressed(options.key_down) {
            options.upward_thrust = options.upward_thrust_max * -1.0;
        }

        if key_input.just_released(options.key_up) || key_input.just_released(options.key_down) {
            options.upward_thrust = 0.0;
        }

        if options.forward_thrust != current_forward_thrust
            || options.upward_thrust != current_upward_thrust
        {
            force.force =
                get_current_force(&transform, options.forward_thrust, options.upward_thrust);

            info!("{}", options.forward_thrust)
        }
    }
}

fn get_current_force(transform: &Transform, forward_thrust: f32, upward_thrust: f32) -> Vec3 {
    let forward = transform.forward().normalize();
    let upward = transform.up().normalize();

    forward * forward_thrust + upward * upward_thrust
}

pub fn control_axis_rotation(
    time: Res<Time>,
    windows: Query<&Window>,
    mut query: Query<(&mut ExternalForce, &mut Transform, &mut CameraController), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut force, mut transform, options)) = query.get_single_mut() {
        let window = windows.single();

        if let Some(cursor_position) = window.cursor_position() {
            let y_coefficient =
                get_relative_motion(cursor_position.y, window.height(), options.movement_spot);

            let x_coefficient =
                get_relative_motion(cursor_position.x, window.width(), options.movement_spot);

            force.force =
                get_current_force(&transform, options.forward_thrust, options.upward_thrust);

            transform.rotation = transform
                .rotation
                .mul_quat(Quat::from_rotation_x(y_coefficient * dt))
                .mul_quat(Quat::from_rotation_z(x_coefficient * dt));
        }
    }
}

fn get_relative_motion(position: f32, domain: f32, movement_spot: f32) -> f32 {
    let blind_spot = 5.0;

    let relative = domain * 0.5 - position;
    let relative_abs = relative.abs();
    let coefficient = if relative > 0.0 { 1.0 } else { -1.0 };

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
