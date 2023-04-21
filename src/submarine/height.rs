use bevy::prelude::*;
use bevy_rapier3d::prelude::{QueryFilter, RapierContext, RigidBody};

#[derive(Clone, Component, Debug, Default)]
pub struct HeightPropertyComponent {
    pub current_height: f32,
}

type TransformHeightPropertyTuple<'a> = (&'a Transform, &'a mut HeightPropertyComponent);

pub fn update_height_property(
    rapier_context: Res<RapierContext>,
    mut query: Query<TransformHeightPropertyTuple, (Changed<Transform>, With<RigidBody>)>,
) {
    let direction = Vec3::new(0.0, -1.0, 0.0);
    let toi_max = 251.0;
    // TODO: rework to collision group for ground
    let filter = QueryFilter::only_fixed();

    for (transform, mut property) in query.iter_mut() {
        let origin = transform.translation;
        if let Some((_entity, toi)) =
            rapier_context.cast_ray(origin, direction, toi_max, true, filter)
        {
            property.current_height = toi.abs();
        }
    }
}
