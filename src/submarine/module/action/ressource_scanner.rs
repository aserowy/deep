use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::{
    color,
    submarine::module::{startup::ModuleStartupComponent, *},
};

use super::ChannelingComponent;

#[derive(Clone, Component)]
pub struct RessourceScannerComponent {
    pub material: Handle<StandardMaterial>,
    pub expanse_max: f32,
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
            transform.scale = Vec3::splat(span * scanner.expanse_max);

            if let Some(material) = materials.get_mut(&scanner.material) {
                material.base_color.set_a(0.5);
            }
        } else {
            // TODO: deactivate effect (shrink, fade)
            transform.scale = Vec3::ZERO;

            if let Some(material) = materials.get_mut(&scanner.material) {
                material.base_color.set_a(0.0);
            }
        }
    }
}
