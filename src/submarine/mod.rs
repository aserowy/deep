use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use self::{
    controller::{
        control_axis_rotation, control_translation, ForwardThrustChangedEvent, SettingsComponent,
        ThrustComponent,
    },
    hud::{setup_hud, update_on_forward_thrust_changed_event, update_velocity},
};

mod controller;
mod hud;

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
            .add_system(setup_player_submarine.on_startup())
            .add_system(setup_hud.on_startup())
            .add_systems(
                (
                    control_axis_rotation,
                    control_translation,
                    update_velocity,
                    update_on_forward_thrust_changed_event,
                )
                    .chain(),
            );
    }
}

#[derive(Default, Resource)]
pub struct PlayerSubmarineResource {
    pub enabled: bool,
    pub entity: Option<Entity>,
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
            VisibilityBundle {
                visibility: Visibility::Visible,
                ..default()
            },
            SettingsComponent::default(),
            ThrustComponent::default(),
            // physics
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
        ))
        .id();

    player.enabled = true;
    player.entity = Some(entity);
}
