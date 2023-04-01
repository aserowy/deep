use std::f32::consts::E;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::submarine::{
    power::PowerUsageComponent,
    settings::{KeyAction, KeyActionEvent, KeyPress},
};

use super::{ModuleStateComponent, ModuleStatus};

const MOVEMENT_SPOT: f32 = 125.0;

#[derive(Clone, Component)]
pub struct EngineComponent {
    pub forward_thrust: f32,
    pub forward_thrust_max: f32,
    pub upward_thrust: f32,
    pub upward_thrust_max: f32,
    pub nose_thrust: f32,
    pub nose_thrust_max: f32,
    pub spin_thrust: f32,
    pub spin_thrust_max: f32,
}

pub fn trigger_engine_change_on_key_action_event(
    time: Res<Time>,
    mut key_action_event_reader: EventReader<KeyActionEvent>,
    mut query: Query<(&mut ExternalForce, &Transform, &Children), With<Camera>>,
    mut child_query: Query<(&ModuleStateComponent, &mut EngineComponent)>,
) {
    let dt = time.delta_seconds();

    if let Ok((mut force, transform, children)) = query.get_single_mut() {
        for key_action_event in key_action_event_reader.iter() {
            let mut child_iter = child_query.iter_many_mut(children);

            // NOTE: this handles only one engine currently
            if let Some((state, mut engine)) = child_iter.fetch_next() {
                if state.status != ModuleStatus::Active && state.status != ModuleStatus::Triggered {
                    continue;
                }

                match key_action_event.key_map.key_action {
                    KeyAction::ThrustPositiv => {
                        handle_forward_thrust(dt, &mut force, &mut engine, transform, true);
                    }
                    KeyAction::ThrustNegative => {
                        handle_forward_thrust(dt, &mut force, &mut engine, transform, false);
                    }
                    KeyAction::ThrustZero => {
                        handle_forward_stop(&mut force, &mut engine, transform);
                    }
                    KeyAction::ThrustUp => {
                        handle_vertical_thrust(
                            &mut force,
                            &mut engine,
                            transform,
                            true,
                            &key_action_event.key_press,
                        );
                    }
                    KeyAction::ThrustDown => {
                        handle_vertical_thrust(
                            &mut force,
                            &mut engine,
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
}

fn handle_vertical_thrust(
    force: &mut ExternalForce,
    thrust: &mut EngineComponent,
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
        force.force = get_current_force(transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn handle_forward_stop(
    force: &mut ExternalForce,
    thrust: &mut EngineComponent,
    transform: &Transform,
) {
    let current_forward_thrust = thrust.forward_thrust;
    thrust.forward_thrust = 0.0;

    if thrust.forward_thrust != current_forward_thrust {
        force.force = get_current_force(transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn handle_forward_thrust(
    dt: f32,
    force: &mut ExternalForce,
    thrust: &mut EngineComponent,
    transform: &Transform,
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
        force.force = get_current_force(transform, thrust.forward_thrust, thrust.upward_thrust);
    }
}

fn get_current_force(transform: &Transform, forward_thrust: f32, upward_thrust: f32) -> Vec3 {
    let forward = transform.forward().normalize();
    let upward = transform.up().normalize();

    forward * forward_thrust + upward * upward_thrust
}

pub fn update_axis_rotation(
    windows: Query<&Window>,
    mut query: Query<(&mut ExternalForce, &mut Transform, &Children), With<Camera>>,
    mut child_query: Query<(&ModuleStateComponent, &mut EngineComponent)>,
) {
    if let Ok((mut force, transform, children)) = query.get_single_mut() {
        let mut child_iter = child_query.iter_many_mut(children);

        // NOTE: this handles only one engine currently
        if let Some((state, mut engine)) = child_iter.fetch_next() {
            if state.status != ModuleStatus::Active && state.status != ModuleStatus::Triggered {
                return;
            }

            let window = windows.single();

            if let Some(cursor_position) = window.cursor_position() {
                force.force =
                    get_current_force(&transform, engine.forward_thrust, engine.upward_thrust);

                let current_spin_thrust = engine.spin_thrust;
                let current_nose_thrust = engine.nose_thrust;

                engine.spin_thrust = engine.spin_thrust_max
                    * get_torque_coefficient(cursor_position.x, window.width(), MOVEMENT_SPOT);

                engine.nose_thrust = engine.nose_thrust_max
                    * get_torque_coefficient(cursor_position.y, window.height(), MOVEMENT_SPOT);

                if engine.spin_thrust != current_spin_thrust
                    || engine.nose_thrust != current_nose_thrust
                {
                    force.torque = transform.forward().normalize() * engine.spin_thrust
                        + transform.left().normalize() * engine.nose_thrust;
                }
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

pub fn set_power_usage_for_engines(
    time: Res<Time>,
    mut query: Query<(&EngineComponent, &mut PowerUsageComponent)>,
) {
    let dt = time.delta_seconds();

    if let Ok((engine, mut usage)) = query.get_single_mut() {
        let consumption = (engine.forward_thrust.abs()
            + engine.upward_thrust.abs()
            + engine.nose_thrust.abs()
            + engine.spin_thrust.abs())
            * dt;

        usage.usage = consumption;
    }
}

pub fn handle_module_state_for_engines(
    mut query: Query<(&mut ExternalForce, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut EngineComponent)>,
) {
    for (mut force, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state, mut engine)) = child_iter.fetch_next() {
            match state.status {
                ModuleStatus::Passive => (),
                ModuleStatus::Startup => set_stop(&mut engine, &mut force),
                ModuleStatus::Active => (),
                ModuleStatus::Triggered => state.status = ModuleStatus::Active,
                ModuleStatus::Shutdown => set_stop(&mut engine, &mut force),
                ModuleStatus::Inactive => set_stop(&mut engine, &mut force),
            }
        }
    }
}

fn set_stop(engine: &mut Mut<EngineComponent>, force: &mut Mut<ExternalForce>) {
    engine.forward_thrust = 0.0;
    engine.upward_thrust = 0.0;
    engine.nose_thrust = 0.0;
    engine.spin_thrust = 0.0;

    force.force = Vec3::ZERO;
    force.torque = Vec3::ZERO;
}
