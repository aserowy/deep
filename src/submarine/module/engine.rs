use std::f32::consts::E;

use bevy::prelude::*;
use bevy_rapier3d::prelude::ExternalForce;

use crate::submarine::{
    power::PowerUsageComponent,
    settings::{KeyAction, KeyActionEvent, KeyPress},
};

use super::{startup::ModuleStartupComponent, *};

const MOVEMENT_SPOT: f32 = 125.0;

// INFO: force in N m/s
#[derive(Clone, Component, Debug, Default, PartialEq)]
pub struct EngineComponent {
    pub forward_force: f32,
    pub forward_force_max: f32,
    pub upward_force: f32,
    pub downward_force: f32,
    pub upward_force_max: f32,
    pub nose_force: f32,
    pub nose_force_max: f32,
    pub spin_force: f32,
    pub spin_force_max: f32,
}

impl EngineComponent {
    pub fn set_stop(&mut self) {
        self.forward_force = 0.0;
        self.upward_force = 0.0;
        self.downward_force = 0.0;
        self.nose_force = 0.0;
        self.spin_force = 0.0;
    }

    pub fn set_stop_with_force(&mut self, force: &mut Mut<ExternalForce>) {
        self.set_stop();

        force.force = Vec3::ZERO;
        force.torque = Vec3::ZERO;
    }
}

pub fn new_basic(asset_server: &Res<AssetServer>, builder: &mut ChildBuilder) {
    builder.spawn((
        ModuleBundle {
            details: ModuleDetailsComponent {
                id: Uuid::new_v4(),
                icon: asset_server.load("submarine/module/afterburn_55px.png"),
            },
            state: ModuleStateComponent {
                state: ModuleState::new(),
            },
        },
        ModuleMassComponent {
            mass: 1.5 * 1000.0,
            ..default()
        },
        EngineComponent {
            forward_force_max: 250.0 * 1000.0,
            upward_force_max: 100.0 * 1000.0,
            nose_force_max: 40.0 * 1000.0,
            spin_force_max: 50.0 * 1000.0,
            ..default()
        },
        ModuleStartupComponent {
            watt: 1000.0 * 1000.0,
            watt_hour: 1500.0,
            ..default()
        },
        PowerUsageComponent::default(),
    ));
}

pub fn on_key_action_event(
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
                if state.state.status() != &ModuleStatus::Active
                    && state.state.status() != &ModuleStatus::ActiveInvalidTrigger
                    && state.state.status() != &ModuleStatus::Triggered
                    && state.state.status() != &ModuleStatus::Aftercast
                {
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
    engine: &mut EngineComponent,
    transform: &Transform,
    is_upward: bool,
    key_press: &KeyPress,
) {
    let current_force = engine.upward_force - engine.downward_force;

    match key_press {
        KeyPress::Down => {
            if is_upward {
                engine.upward_force = engine.upward_force_max;
            } else {
                engine.downward_force = engine.upward_force_max;
            }
        }
        KeyPress::Release => {
            if is_upward {
                engine.upward_force = 0.0;
            } else {
                engine.downward_force = 0.0;
            }
        }
        _ => (),
    }

    if current_force != (engine.upward_force - engine.downward_force) {
        force.force = get_current_force(
            transform,
            engine.forward_force,
            engine.upward_force,
            engine.downward_force,
        );
    }
}

fn handle_forward_stop(
    force: &mut ExternalForce,
    thrust: &mut EngineComponent,
    transform: &Transform,
) {
    let current_forward_thrust = thrust.forward_force;
    thrust.forward_force = 0.0;

    if thrust.forward_force != current_forward_thrust {
        force.force = get_current_force(
            transform,
            thrust.forward_force,
            thrust.upward_force,
            thrust.downward_force,
        );
    }
}

fn handle_forward_thrust(
    dt: f32,
    force: &mut ExternalForce,
    thrust: &mut EngineComponent,
    transform: &Transform,
    is_forward: bool,
) {
    let current_forward_thrust = thrust.forward_force;
    let current_step = thrust.forward_force_max * 0.4 * dt;

    if is_forward {
        thrust.forward_force += current_step;
    } else {
        thrust.forward_force -= current_step;
    }

    if thrust.forward_force.abs() > thrust.forward_force_max {
        let coefficient = if thrust.forward_force > 0.0 {
            1.0
        } else {
            -1.0
        };

        thrust.forward_force = thrust.forward_force_max * coefficient;
    }

    if thrust.forward_force != current_forward_thrust {
        force.force = get_current_force(
            transform,
            thrust.forward_force,
            thrust.upward_force,
            thrust.downward_force,
        );
    }
}

fn get_current_force(
    transform: &Transform,
    forward_thrust: f32,
    upward_thrust: f32,
    downward_force: f32,
) -> Vec3 {
    let forward = transform.forward().normalize();
    let upward = transform.up().normalize();

    forward * forward_thrust + upward * (upward_thrust - downward_force)
}

pub fn on_mouse_position_change(
    windows: Query<&Window>,
    mut query: Query<(&mut ExternalForce, &mut Transform, &Children), With<Camera>>,
    mut child_query: Query<(&ModuleStateComponent, &mut EngineComponent)>,
) {
    if let Ok((mut force, transform, children)) = query.get_single_mut() {
        let mut child_iter = child_query.iter_many_mut(children);

        // NOTE: this handles only one engine currently
        if let Some((state, mut engine)) = child_iter.fetch_next() {
            if state.state.status() != &ModuleStatus::Active
                && state.state.status() != &ModuleStatus::ActiveInvalidTrigger
                && state.state.status() != &ModuleStatus::Triggered
                && state.state.status() != &ModuleStatus::Aftercast
            {
                return;
            }

            let window = windows.single();

            if let Some(cursor_position) = window.cursor_position() {
                force.force = get_current_force(
                    &transform,
                    engine.forward_force,
                    engine.upward_force,
                    engine.downward_force,
                );

                let current_spin_thrust = engine.spin_force;
                let current_nose_thrust = engine.nose_force;

                engine.spin_force = engine.spin_force_max
                    * get_torque_coefficient(cursor_position.x, window.width(), MOVEMENT_SPOT);

                engine.nose_force = engine.nose_force_max
                    * get_torque_coefficient(cursor_position.y, window.height(), MOVEMENT_SPOT);

                if engine.spin_force != current_spin_thrust
                    || engine.nose_force != current_nose_thrust
                {
                    force.torque = transform.forward().normalize() * engine.spin_force
                        + transform.left().normalize() * engine.nose_force;
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

    for (engine, mut usage) in query.iter_mut() {
        let consumption = (engine.forward_force.abs()
            + (engine.upward_force - engine.downward_force).abs()
            + engine.nose_force.abs()
            + engine.spin_force.abs())
            * dt;

        usage.watt_per_second = consumption; // 1 N m/s == 1 W
    }
}

pub fn handle_module_state_for_engines(
    mut query: Query<(&mut ExternalForce, &Children)>,
    mut child_query: Query<(&mut ModuleStateComponent, &mut EngineComponent)>,
) {
    for (mut force, children) in query.iter_mut() {
        let mut child_iter = child_query.iter_many_mut(children);
        while let Some((mut state, mut engine)) = child_iter.fetch_next() {
            match state.state.status() {
                ModuleStatus::Passive => (),
                ModuleStatus::StartingUp => engine.set_stop_with_force(&mut force),
                ModuleStatus::Active => (),
                ModuleStatus::ActiveInvalidTrigger => state.state.next(ModuleStatus::Active),
                ModuleStatus::Triggered => state.state.next(ModuleStatus::Aftercast),
                ModuleStatus::Aftercast => state.state.next(ModuleStatus::Active),
                ModuleStatus::ShuttingDown => engine.set_stop_with_force(&mut force),
                ModuleStatus::Inactive => engine.set_stop_with_force(&mut force),
            }
        }
    }
}
