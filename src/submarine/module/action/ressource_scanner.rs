use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::{
    color,
    submarine::module::{
        aftercast::ModuleAftercastComponent, shutdown::ModuleShutdownComponent,
        startup::ModuleStartupComponent, *,
    },
};

use super::ChannelingComponent;

#[derive(Clone, Component)]
pub struct RessourceScannerComponent {
    pub material: Handle<StandardMaterial>,
    pub expanse_max: f32,
    pub cleanup_in_seconds: f32,
}

pub fn new_basic(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> (
    ModuleBundle,
    ModuleMassComponent,
    PbrBundle,
    NotShadowCaster,
    RessourceScannerComponent,
    ModuleStartupComponent,
    ChannelingComponent,
    ModuleAftercastComponent,
    ModuleShutdownComponent,
    PowerUsageComponent,
) {
    let material = materials.add(StandardMaterial {
        base_color: color::TURQUOISE_25,
        emissive: color::SKY_BLUE_25,
        alpha_mode: AlphaMode::Blend,
        double_sided: true,
        cull_mode: None,
        ..default()
    });

    (
        ModuleBundle {
            details: ModuleDetailsComponent {
                id: Uuid::new_v4(),
                icon: "󰐷".into(),
            },
            state: ModuleStateComponent {
                state: ModuleState::new(),
            },
        },
        ModuleMassComponent {
            initialized: false,
            mass: 2.5 * 1000.0,
        },
        PbrBundle {
            mesh: meshes.add(shape::UVSphere::default().into()),
            material: material.clone(),
            transform: Transform::from_scale(Vec3::ZERO),
            ..default()
        },
        NotShadowCaster,
        RessourceScannerComponent {
            material,
            expanse_max: 42.0,
            cleanup_in_seconds: 4.0,
        },
        ModuleStartupComponent {
            watt_per_second: 1500.0 * 1000.0,
            watt: 5000.0 * 1000.0,
            current_watt: None,
        },
        ChannelingComponent {
            current_duration: None,
            duration: 8.0,
            watt_per_second: 450.0 * 1000.0,
        },
        ModuleAftercastComponent::default(),
        ModuleShutdownComponent::default(),
        PowerUsageComponent::default(),
    )
}

pub fn activate(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<
        (
            &RessourceScannerComponent,
            &ChannelingComponent,
            &mut Transform,
        ),
        Changed<ChannelingComponent>,
    >,
) {
    for (scanner, channel, mut transform) in query.iter_mut() {
        if let Some(span) = channel.current_duration {
            transform.scale = Vec3::splat(span * scanner.expanse_max / channel.duration);
            if let Some(material) = materials.get_mut(&scanner.material) {
                material.base_color.set_a(0.5);
            }
        }
    }
}

// TODO: refactor logic of aftercast and shutdown
pub fn deactivate_on_aftercast(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &ModuleStateComponent,
        &RessourceScannerComponent,
        &mut ModuleAftercastComponent,
        &mut Transform,
    )>,
) {
    for (state, scanner, mut aftercast, mut transform) in query.iter_mut() {
        if state.state.status() != &ModuleStatus::Aftercast {
            aftercast.current_spindown_time = None;
            aftercast.spindown_time = None;

            continue;
        }

        if let (Some(spindown_time), Some(current_spindown_time)) =
            (aftercast.spindown_time, aftercast.current_spindown_time)
        {
            let current_scale = transform.scale.x;

            if current_scale > 0.1 {
                let scale = scanner.expanse_max * current_spindown_time / spindown_time;
                transform.scale = Vec3::splat(scale);
            } else {
                transform.scale = Vec3::ZERO;
                aftercast.current_spindown_time = Some(0.0);

                if let Some(material) = materials.get_mut(&scanner.material) {
                    material.base_color.set_a(0.0);
                }
            }
        } else {
            let current = scanner.cleanup_in_seconds * transform.scale.x / scanner.expanse_max;
            aftercast.current_spindown_time = Some(current);
            aftercast.spindown_time = Some(scanner.cleanup_in_seconds);
        }
    }
}

pub fn deactivate_on_shutdown(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(
        &ModuleStateComponent,
        &RessourceScannerComponent,
        &mut ModuleShutdownComponent,
        &mut Transform,
    )>,
) {
    for (state, scanner, mut shutdown, mut transform) in query.iter_mut() {
        if state.state.status() != &ModuleStatus::ShuttingDown {
            shutdown.current_spindown_time = None;
            shutdown.spindown_time = None;

            continue;
        }

        if let (Some(spindown_time), Some(current_spindown_time)) =
            (shutdown.spindown_time, shutdown.current_spindown_time)
        {
            let current_scale = transform.scale.x;

            if current_scale > 0.1 {
                let scale = scanner.expanse_max * current_spindown_time / spindown_time;
                transform.scale = Vec3::splat(scale);
            } else {
                transform.scale = Vec3::ZERO;
                shutdown.current_spindown_time = Some(0.0);

                if let Some(material) = materials.get_mut(&scanner.material) {
                    material.base_color.set_a(0.0);
                }
            }
        } else {
            let current = scanner.cleanup_in_seconds * transform.scale.x / scanner.expanse_max;
            shutdown.current_spindown_time = Some(current);
            shutdown.spindown_time = Some(scanner.cleanup_in_seconds);
        }
    }
}
