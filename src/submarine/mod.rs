use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use self::{controller::*, hud::*, module::*, settings::*};

mod controller;
mod hud;
mod module;
mod settings;

pub struct SubmarinePlugin {}

impl Default for SubmarinePlugin {
    fn default() -> Self {
        SubmarinePlugin {}
    }
}

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerSubmarineResource>()
            .add_event::<ForwardThrustChangedEvent>()
            .add_event::<KeyActionEvent>()
            .add_system(setup_player_submarine.on_startup())
            .add_system(setup_hud.on_startup())
            .add_systems(
                (
                    handle_key_presses,
                    update_thrust_on_key_action_event,
                    update_axis_rotation,
                    update_velocity_node,
                    update_thrust_node_on_forward_thrust_changed_event,
                    update_modules,
                )
                    .chain(),
            );
    }
}

#[derive(Default, Resource)]
pub struct PlayerSubmarineResource {
    pub enabled: bool,
    pub entity: Option<Entity>,
    pub modules: Vec<Module>,
}

fn setup_player_submarine(mut commands: Commands, mut player: ResMut<PlayerSubmarineResource>) {
    let entity = commands
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
            // hud & controls
            (
                VisibilityBundle {
                    visibility: Visibility::Visible,
                    ..default()
                },
                SettingsComponent::default(),
                ThrustComponent::default(),
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
        .id();

    player.enabled = true;
    player.entity = Some(entity);
    player.modules = vec![
        Module::new_resource_scanner_base(),
        Module::new_mining_base(),
    ];
}
