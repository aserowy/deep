use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use self::{
    hud::*,
    module::{action::*, engine::*, *},
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
            .add_system(setup_hud.on_startup().in_base_set(StartupSet::PostStartup))
            .add_systems(
                (
                    // handle automatic state transitions
                    // TODO: startup -> active, shutdown -> inactive
                    //
                    // handle passive effects
                    // TODO: PassiveComponent
                    //
                    // handle user input
                    handle_key_presses,
                    trigger_engine_change_on_key_action_event,
                    trigger_module_status_triggered_on_key_action_event,
                    update_axis_rotation,
                    // calculate power usage
                    set_power_usage_for_engines,
                    // handle power management
                    update_power_capacity_component_by_core,
                    update_power_capacity_component_by_module_power_usage,
                    // handle state
                    handle_module_state_for_engines,
                    handle_module_state_for_actions,
                    // ui
                    update_modules,
                    update_capacity_node_on_capacitor_componend_changed,
                    update_thrust_node_on_engine_component_changed,
                    update_velocity_node,
                )
                    .chain(),
            );
    }
}

#[derive(Default, Component)]
pub struct PlayerSubmarineComponent {
    pub enabled: bool,
    pub entity: Option<Entity>,
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
            builder.spawn((
                ModuleBundle {
                    details: ModuleDetailsComponent {
                        id: "engine".into(),
                        icon: "󰇺".into(),
                    },
                    state: ModuleStateComponent {
                        state: ModuleState::new(),
                    },
                },
                EngineComponent {
                    forward_thrust: 0.0,
                    forward_thrust_max: 2500.0,
                    upward_thrust: 0.0,
                    upward_thrust_max: 1000.0,
                    nose_thrust: 0.0,
                    nose_thrust_max: 500.0,
                    spin_thrust: 0.0,
                    spin_thrust_max: 500.0,
                },
                PowerUsageComponent::default(),
            ));

            builder.spawn((
                ModuleBundle {
                    details: ModuleDetailsComponent {
                        id: "resource_scanner_base".into(),
                        icon: "󰐷".into(),
                        // action: ModuleAction::ResourceScan,
                    },
                    state: ModuleStateComponent {
                        state: ModuleState::new(),
                    },
                },
                ActionComponent {},
                PowerUsageComponent::default(),
            ));

            builder.spawn((
                ModuleBundle {
                    details: ModuleDetailsComponent {
                        id: "mining_base".into(),
                        icon: "󰜐".into(),
                        // action: ModuleAction::MiningMagnatide,
                    },
                    state: ModuleStateComponent {
                        state: ModuleState::new(),
                    },
                },
                ActionComponent {},
                PowerUsageComponent::default(),
            ));
        });
}
