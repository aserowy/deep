use std::f32::consts::E;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use super::settings::*;

#[derive(Clone, Component)]
pub struct ThrustComponent {
    pub enabled: bool,
    pub initialized: bool,
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
            initialized: false,
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

pub struct ForwardThrustChangedEvent(pub ThrustComponent);

pub fn update_thrust_on_key_action_event(
    time: Res<Time>,
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    mut forward_thrust_event_writer: EventWriter<ForwardThrustChangedEvent>,
    mut query: Query<(&mut ExternalForce, &Transform, &mut ThrustComponent), With<Camera>>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut force, transform, mut thrust)) = query.get_single_mut() {
        if !thrust.enabled {
            return;
        }

        if !thrust.initialized {
            thrust.initialized = true;
            forward_thrust_event_writer.send(ForwardThrustChangedEvent(thrust.clone()));
        }

        for key_action_event in key_action_event_reader.iter() {
            match key_action_event.key_map.key_action {
                KeyAction::ThrustPositiv => {
                    handle_forward_thrust(
                        dt,
                        &mut force,
                        &mut thrust,
                        transform,
                        &mut forward_thrust_event_writer,
                        true,
                    );
                }
                KeyAction::ThrustNegative => {
                    handle_forward_thrust(
                        dt,
                        &mut force,
                        &mut thrust,
                        transform,
                        &mut forward_thrust_event_writer,
                        false,
                    );
                }
                KeyAction::ThrustZero => {
                    handle_forward_stop(
                        &mut force,
                        &mut thrust,
                        transform,
                        &mut forward_thrust_event_writer,
                    );
                }
                KeyAction::ThrustUp => {
                    handle_vertical_thrust(
                        &mut force,
                        &mut thrust,
                        transform,
                        true,
                        &key_action_event.key_press,
                    );
                }
                KeyAction::ThrustDown => {
                    handle_vertical_thrust(
                        &mut force,
                        &mut thrust,
                        transform,
                        false,
                        &key_action_event.key_press,
                    );
                }
                _ => (),
            };
        }
    }
}

fn handle_vertical_thrust(
    force: &mut ExternalForce,
    thrust: &mut ThrustComponent,
    transform: &Transform,
    is_upward: bool,
    key_press: &KeyPress,
) {
    let current_upward_thrust = thrust.upward_thrust;

    match key_press {
        KeyPress::Down() => {
            thrust.upward_thrust += if is_upward {
                thrust.upward_thrust_max
            } else {
                thrust.upward_thrust_max * -1.0
            }
        }
        KeyPress::Release() => {
            thrust.upward_thrust -= if is_upward {
                thrust.upward_thrust_max
            } else {
                thrust.upward_thrust_max * -1.0
            }
        }
        _ => (),
    }

    if thrust.upward_thrust.abs() > thrust.upward_thrust_max {
        let coefficient = if thrust.upward_thrust > 0.0 {
            1.0
        } else {
            -1.0
        };

        thrust.upward_thrust = thrust.upward_thrust_max * coefficient;
    }

    if thrust.upward_thrust != current_upward_thrust {
        force.force = get_current_force(&transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn handle_forward_stop(
    force: &mut ExternalForce,
    thrust: &mut ThrustComponent,
    transform: &Transform,
    event_writer: &mut EventWriter<ForwardThrustChangedEvent>,
) {
    let current_forward_thrust = thrust.forward_thrust;
    thrust.forward_thrust = 0.0;

    if thrust.forward_thrust != current_forward_thrust {
        event_writer.send(ForwardThrustChangedEvent(thrust.clone()));
        force.force = get_current_force(&transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn handle_forward_thrust(
    dt: f32,
    force: &mut ExternalForce,
    thrust: &mut ThrustComponent,
    transform: &Transform,
    event_writer: &mut EventWriter<ForwardThrustChangedEvent>,
    is_forward: bool,
) {
    let current_forward_thrust = thrust.forward_thrust;
    if is_forward {
        thrust.forward_thrust += 2000.0 * dt;
    } else {
        thrust.forward_thrust -= 2000.0 * dt;
    }

    if thrust.forward_thrust.abs() > thrust.forward_thrust_max {
        let coefficient = if thrust.forward_thrust > 0.0 {
            1.0
        } else {
            -1.0
        };

        thrust.forward_thrust = thrust.forward_thrust_max * coefficient;
    }

    if thrust.forward_thrust != current_forward_thrust {
        event_writer.send(ForwardThrustChangedEvent(thrust.clone()));
        force.force = get_current_force(&transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn get_current_force(transform: &Transform, forward_thrust: f32, upward_thrust: f32) -> Vec3 {
    let forward = transform.forward().normalize();
    let upward = transform.up().normalize();

    forward * forward_thrust + upward * upward_thrust
}

pub fn update_axis_rotation(
    windows: Query<&Window>,
    mut query: Query<
        (
            &mut ExternalForce,
            &mut Transform,
            &mut ThrustComponent,
            &SettingsComponent,
        ),
        With<Camera>,
    >,
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
                * get_torque_coefficient(
                    cursor_position.y,
                    window.height(),
                    settings.movement_spot,
                );

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
