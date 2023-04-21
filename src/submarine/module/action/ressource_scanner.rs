use std::f32::consts::E;

use bevy::{pbr::NotShadowCaster, prelude::*};

use crate::{
    color,
    render::force_field::ForceFieldMaterial,
    submarine::module::{
        aftercast::ModuleAftercastComponent,
        condition::{engine_stop::EngineStopConditionComponent, ConditionStateComponent},
        requirement::{MaximumHeightRequirementComponent, RequirementStateComponent},
        startup::ModuleStartupComponent,
        *,
    },
};

use super::ChannelingComponent;

#[derive(Clone, Component)]
pub struct ExpandingSphereEffectComponent {
    pub expanse_max: f32,
    pub cleanup_in_seconds: f32,
}

pub fn new_basic(
    builder: &mut ChildBuilder,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ForceFieldMaterial>>,
) {
    builder
        .spawn((
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
                mass: 2.5 * 1000.0,
                ..default()
            },
            MaterialMeshBundle {
                mesh: meshes.add(shape::UVSphere::default().into()),
                material: materials.add(ForceFieldMaterial {
                    color: color::GRAPE,
                    alpha_mode: AlphaMode::Blend,
                }),
                transform: Transform::from_scale(Vec3::ZERO),
                ..default()
            },
            NotShadowCaster,
            ExpandingSphereEffectComponent {
                expanse_max: 42.0,
                cleanup_in_seconds: 4.0,
            },
            ModuleStartupComponent {
                watt: 1500.0 * 1000.0,
                watt_hour: 1500.0,
                ..default()
            },
            ChannelingComponent {
                duration: 8.0,
                watt_per_second: 450.0 * 1000.0,
                ..default()
            },
            ModuleAftercastComponent::default(),
            PowerUsageComponent::default(),
            ConditionStateComponent::default(),
            EngineStopConditionComponent::default(),
        ))
        .with_children(|child_builer| {
            child_builer.spawn((
                RequirementStateComponent::default(),
                MaximumHeightRequirementComponent {
                    maximum_height: 5.0,
                },
            ));
        });
}

pub fn activate(
    mut query: Query<
        (
            &ExpandingSphereEffectComponent,
            &ChannelingComponent,
            &mut Transform,
        ),
        Changed<ChannelingComponent>,
    >,
) {
    for (scanner, channel, mut transform) in query.iter_mut() {
        if let Some(span) = channel.current_duration {
            let scale = scanner.expanse_max * get_coefficient(span, channel.duration);
            transform.scale = Vec3::splat(scale);
        }
    }
}

pub fn deactivate_on_aftercast(
    mut query: Query<(
        &ModuleStateComponent,
        &ExpandingSphereEffectComponent,
        &mut ModuleAftercastComponent,
        &mut Transform,
    )>,
) {
    for (state, scanner, mut aftercast, mut transform) in query.iter_mut() {
        match state.state.status() {
            ModuleStatus::Triggered => continue,
            ModuleStatus::Aftercast => cleanup_effect(scanner, &mut aftercast, &mut transform),
            ModuleStatus::ShuttingDown => cleanup_effect(scanner, &mut aftercast, &mut transform),
            _ => reset_module(&mut aftercast, &mut transform),
        }
    }
}

fn cleanup_effect(
    scanner: &ExpandingSphereEffectComponent,
    aftercast: &mut ModuleAftercastComponent,
    transform: &mut Transform,
) {
    if let (Some(spindown_base), Some(spindown_time), Some(current_spindown_time)) = (
        aftercast.spindown_base,
        aftercast.spindown_time,
        aftercast.current_spindown_time,
    ) {
        let scale = spindown_base * get_coefficient(current_spindown_time, spindown_time);
        transform.scale = Vec3::splat(scale);
    } else {
        let current = scanner.cleanup_in_seconds * transform.scale.x / scanner.expanse_max;
        let base = transform.scale.x / get_coefficient(current, current);

        aftercast.spindown_base = Some(base);

        aftercast.current_spindown_time = Some(current);
        aftercast.spindown_time = Some(current);
    }
}

fn reset_module(aftercast: &mut Mut<ModuleAftercastComponent>, transform: &mut Mut<Transform>) {
    aftercast.current_spindown_time = None;
    aftercast.spindown_time = None;

    transform.scale = Vec3::ZERO;
}

fn get_coefficient(current: f32, max: f32) -> f32 {
    // logistic function
    1.0 / (1.0 + E.powf(5.0 - 10.0 * current / max))
}
