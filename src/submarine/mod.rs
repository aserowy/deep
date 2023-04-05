use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use self::{
    module::{action, engine, shutdown, startup},
    power::*,
    settings::*,
};

mod hud;
mod module;
mod power;
mod settings;

#[derive(Default)]
pub struct SubmarinePlugin {}

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app //
            .add_event::<KeyActionEvent>()
            .add_system(setup_player_submarine.on_startup())
            .add_system(hud::setup.on_startup().in_base_set(StartupSet::PostStartup))
            // handle passive effects
            // TODO: PassiveComponent
            //
            .add_systems(
                (
                    // handle automatic state transitions
                    action::update_module_channeling_state_transition,
                    startup::update_module_startup_state_transition,
                    shutdown::update_module_shutdown_state_transition,
                    shutdown::update_module_shutdown_state_transition_with_shutdown_component,
                )
                    .in_base_set(CoreSet::PreUpdate),
            )
            .add_systems((
                // handle user input
                handle_key_presses,
                engine::trigger_engine_change_on_key_action_event,
                engine::update_axis_rotation,
                module::trigger_module_status_triggered_on_key_action_event,
                // calculate power usage
                action::set_power_usage_for_channels,
                engine::set_power_usage_for_engines,
                // handle power management
                power::update_power_capacity_component_by_core,
                startup::update_power_capacity_by_module_startup,
                module::update_power_capacity_component_by_module_power_usage,
                // handle state
                action::handle_module_state_for_channels,
                engine::handle_module_state_for_engines,
            ))
            .add_systems(
                (
                    // ui
                    hud::update_capacity_node_on_capacitor_componend_changed,
                    hud::update_modules_by_module_shutdown,
                    hud::update_modules_by_module_startup,
                    hud::update_modules_by_module_state,
                    hud::update_modules_cooldown_by_module_channeling,
                    hud::update_modules_consumption_by_module_channeling,
                    hud::update_thrust_node_on_engine_component_changed,
                    hud::update_velocity_node,
                )
                    .in_base_set(CoreSet::PostUpdate),
            );
    }
}

fn setup_player_submarine(mut commands: Commands) {
    info!("setup_player_submarine");

    commands
        .spawn((
            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    ..default()
                },
                tonemapping: Tonemapping::AcesFitted,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            BloomSettings {
                intensity: 0.25,
                ..default()
            },
            FogSettings {
                color: Color::rgb(0.0, 0.36, 0.45),
                falloff: FogFalloff::from_visibility_color(256.0, Color::rgb(0.35, 0.5, 0.66)),
                ..default()
            },
            AtmosphereCamera::default(),
            // hud & settings
            (
                VisibilityBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                SettingsComponent::default(),
                KeyMapComponent {
                    key_actions: vec![
                        KeyActionMap {
                            key_code: KeyCode::W,
                            key_action: KeyAction::ThrustPositiv,
                        },
                        KeyActionMap {
                            key_code: KeyCode::S,
                            key_action: KeyAction::ThrustNegative,
                        },
                        KeyActionMap {
                            key_code: KeyCode::Q,
                            key_action: KeyAction::ThrustZero,
                        },
                        KeyActionMap {
                            key_code: KeyCode::D,
                            key_action: KeyAction::ThrustUp,
                        },
                        KeyActionMap {
                            key_code: KeyCode::A,
                            key_action: KeyAction::ThrustDown,
                        },
                        KeyActionMap {
                            key_code: KeyCode::Key1,
                            key_action: KeyAction::ModuleActivation01,
                        },
                        KeyActionMap {
                            key_code: KeyCode::Key2,
                            key_action: KeyAction::ModuleActivation02,
                        },
                        KeyActionMap {
                            key_code: KeyCode::Key3,
                            key_action: KeyAction::ModuleActivation03,
                        },
                    ],
                },
            ),
            // power management
            (
                PowerCoreComponent { production: 2000.0 },
                PowerCapacitorComponent {
                    capacity: 10000.0,
                    capacity_max: 10000.0,
                },
            ),
            // physics
            (
                RigidBody::Dynamic,
                ExternalForce::default(),
                Velocity::default(),
                Damping {
                    linear_damping: 2.0,
                    angular_damping: 1.0,
                },
                GravityScale(0.0),
                Collider::ball(3.0),
                AdditionalMassProperties::Mass(10.0),
            ),
        ))
        .with_children(|builder| {
            builder.spawn(action::new_resource_scanner_basic());
            builder.spawn(engine::new_thruster_basic());
        });
}
