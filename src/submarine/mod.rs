use bevy::prelude::*;
use bevy_atmosphere::prelude::AtmosphereCamera;
use bevy_rapier3d::prelude::*;

use self::controller::{control_axis_rotation, control_translation, CameraController};

mod controller;

pub struct SubmarinePlugin {}

impl Default for SubmarinePlugin {
    fn default() -> Self {
        SubmarinePlugin {}
    }
}

impl Plugin for SubmarinePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_camera.on_startup())
            .add_system(setup_crosshair.on_startup())
            .add_systems((control_axis_rotation, control_translation).chain());
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(5.0, 0.0, 5.0),
            ..default()
        },
        // TODO: add AtmosphereCamera handling to sky.rs
        AtmosphereCamera::default(),
        CameraController::default(),
        FogSettings {
            color: Color::rgb(0.0, 0.36, 0.45),
            falloff: FogFalloff::from_visibility_color(256.0, Color::rgb(0.35, 0.5, 0.66)),
            ..default()
        },
        RigidBody::Dynamic,
        ExternalForce::default(),
        Damping {
            linear_damping: 2.0,
            angular_damping: 1.0,
        },
        GravityScale(0.0),
        Collider::ball(3.0),
        AdditionalMassProperties::Mass(10.0),
    ));
}

fn setup_crosshair(mut commands: Commands, assets: Res<AssetServer>) {
    // TODO: implement programmatic crosshair
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                image: assets.load("submarine/crosshair.png").into(),
                style: Style {
                    size: Size::new(Val::Px(250.0), Val::Px(250.0)),
                    ..default()
                },
                ..default()
            });
        });
}
