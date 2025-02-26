use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        prepass::{DepthPrepass, NormalPrepass},
        tonemapping::Tonemapping,
    },
    prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use crate::{render::force_field::ForceFieldMaterial, submarine::height::HeightPropertyComponent};

use self::{
    module::{
        action::{self, ressource_scanner},
        aftercast,
        condition::{self, update_engine_stop_condition_by_module_state},
        engine, requirement, startup,
    },
    power::*,
    settings::*,
};

mod height;
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
            //
            .add_systems(
                (
                    // update properties
                    height::update_height_property,
                    // handle passive effects
                    // TODO: PassiveComponent
                )
                    .in_base_set(CoreSet::PreUpdate),
            )
            .add_systems(
                (
                    // handle ship mass
                    module::update_mass_by_module_mass,
                    // handle automatic module state transitions
                    action::update_module_channeling_state_transition,
                    aftercast::update_module_aftercast_state_transition,
                    aftercast::update_module_aftercast_state_transition_with_aftercast_component,
                    startup::update_module_startup_state_transition,
                    // handle automatic condition state transitions
                    update_engine_stop_condition_by_module_state,
                )
                    .in_base_set(CoreSet::PreUpdate),
            )
            .add_systems(
                (
                    requirement::handle_maximum_height_requirement,
                    requirement::set_module_state_by_requirement_states,
                )
                    .chain()
                    .in_base_set(CoreSet::PreUpdate),
            )
            .add_systems(
                (
                    // handle user input
                    handle_key_presses,
                    engine::on_key_action_event,
                    engine::on_mouse_position_change,
                    module::on_key_action_event,
                    // handle conditions
                    condition::engine_stop::update_engine_by_engine_stop_condition,
                    // calculate power usage
                    action::set_power_usage_for_channels,
                    engine::set_power_usage_for_engines,
                    // handle power management
                    power::update_capacity_by_core,
                    startup::update_power_capacity_by_module_startup,
                    module::update_power_capacity_by_module_power_usage,
                    // handle state
                    action::handle_module_state_for_channels,
                    engine::handle_module_state_for_engines,
                )
                    .chain(),
            )
            .add_systems(
                (
                    // actions
                    ressource_scanner::activate,
                    ressource_scanner::deactivate_on_aftercast,
                )
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_systems(
                (
                    // ui
                    hud::condition::update_condition_row_ui_component,
                    hud::information::update_capacity_node_on_capacitor_componend_changed,
                    hud::information::update_height_node,
                    hud::information::update_thrust_node_on_engine_component_changed,
                    hud::information::update_velocity_node,
                    hud::module::reset_consumption_ui_component,
                    hud::module::reset_cooldown_ui_component,
                    hud::module::update_modules_by_module_state,
                    hud::module::update_modules_consumption_by_module_channeling,
                    hud::module::update_modules_consumption_by_module_startup,
                    hud::module::update_modules_cooldown_by_module_aftercast,
                    hud::module::update_modules_cooldown_by_module_channeling,
                    hud::module::update_modules_requirement_by_state,
                )
                    .chain()
                    .in_base_set(CoreSet::PostUpdate),
            );
    }
}

fn setup_player_submarine(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ForceFieldMaterial>>,
) {
    info!("setup_player_submarine");

    commands
        .spawn((
            (
                Camera3dBundle {
                    camera: Camera {
                        hdr: true,
                        ..default()
                    },
                    tonemapping: Tonemapping::AcesFitted,
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                DepthPrepass,
                NormalPrepass,
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
            ),
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
                PowerCoreComponent {
                    watt_per_second: 400.0 * 1000.0,
                },
                PowerCapacitorComponent {
                    watt_hour: 1.0 * 1000.0,
                    watt_hour_max: 1.0 * 1000.0,
                },
            ),
            // physics
            (
                RigidBody::Dynamic,
                ExternalForce::default(),
                Velocity::default(),
                Damping {
                    linear_damping: 1.0,
                    angular_damping: 1.0,
                },
                GravityScale(0.0),
            ),
            // properties
            (
                AdditionalMassProperties::Mass(0.0),
                HeightPropertyComponent::default(),
            ),
        ))
        .with_children(|builder| {
            builder.spawn((
                Collider::cuboid(2.0, 2.0, 5.0),
                ColliderMassProperties::Mass(6.0 * 1000.0), // kg
            ));

            ressource_scanner::new_basic(&asset_server, builder, &mut meshes, &mut materials);
            engine::new_basic(&asset_server, builder);
        });
}
