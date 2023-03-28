use bevy::prelude::{shape::Circle, *};

use crate::render::line::{LineMaterial, LineStrip};

use super::PlayerSubmarineResource;

pub fn setup_hud(
    mut commands: Commands,
    player: Res<PlayerSubmarineResource>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
) {
    if !player.enabled {
        return;
    }

    if let Some(entity) = player.entity {
        commands.entity(entity).with_children(|parent| {
            // hud
            parent.spawn(MaterialMeshBundle {
                mesh: meshes.add(LineStrip::from(Circle::new(0.002)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });

            parent.spawn(MaterialMeshBundle {
                // TODO: calculate circle size from options.movement_spot (0.075 fits an 125 spot)
                mesh: meshes.add(LineStrip::from(Circle::new(0.075)).into()),
                material: line_materials.add(LineMaterial {
                    color: Color::WHITE.into(),
                }),
                transform: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            });
        });
    }

    commands.spawn(TextBundle::from_section(
        "Press Spacebar to Toggle Atmospheric Fog.\nPress S to Toggle Directional Light Fog Influence.",
        TextStyle {
            font: asset_server.load("fonts/monofur.ttf"),
            font_size: 15.0,
            color: Color::WHITE,
        },
    )
    .with_style(Style {
        position_type: PositionType::Absolute,
            ..default()
    }),);
}
