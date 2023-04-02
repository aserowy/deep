use std::slice::Iter;

use bevy::{
    ecs::query::QueryManyIter,
    pbr::NotShadowCaster,
    prelude::{shape::Circle, *},
};
use bevy_rapier3d::prelude::Velocity;

use crate::render::line::{LineMaterial, LineStrip};

use super::{
    module::{engine::EngineComponent, ModuleDetailsComponent},
    power::PowerCapacitorComponent,
};

#[derive(Default, Component)]
pub struct CapacityUiComponent {}

#[derive(Default, Component)]
pub struct ThrustUiComponent {}

#[derive(Default, Component)]
pub struct VelocityUiComponent {}

pub fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut line_materials: ResMut<Assets<LineMaterial>>,
    query: Query<(Entity, &Children), With<Camera>>,
    child_query: Query<&ModuleDetailsComponent>,
) {
    info!("setup_hud");

    if let Ok((entity, children)) = query.get_single() {
        commands.entity(entity).with_children(|parent| {
            // hud
            let line_material = line_materials.add(LineMaterial {
                color: Color::WHITE,
            });

            parent.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(LineStrip::from(Circle::new(0.002)).into()),
                    material: line_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                NotShadowCaster,
            ));

            parent.spawn((
                MaterialMeshBundle {
                    // TODO: calculate circle size from options.movement_spot (0.075 fits an 125 spot)
                    mesh: meshes.add(LineStrip::from(Circle::new(0.075)).into()),
                    material: line_material,
                    transform: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                },
                NotShadowCaster,
            ));
        });

        let child_iter = child_query.iter_many(children);
        let font = asset_server.load("fonts/monofur.ttf");
        commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::all(Val::Percent(100.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                add_main_screen_column_nodes(builder, child_iter, font.clone());
            });
    }
}

fn add_main_screen_column_nodes(
    builder: &mut ChildBuilder,
    mut child_iter: QueryManyIter<&ModuleDetailsComponent, (), Iter<Entity>>,
    font: Handle<Font>,
) {
    builder.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
            ..default()
        },
        ..default()
    });

    builder.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
            ..default()
        },
        ..default()
    });

    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(12.5)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            add_hud_nodes(builder, font.clone());
        });

    builder.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Percent(25.0)),
            ..default()
        },
        ..default()
    });

    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(12.5)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            while let Some(details) = child_iter.fetch_next() {
                add_module_to_module_nodes(builder, details, font.clone());
            }
        });
}

fn add_hud_nodes(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Px(5.0)),
                size: Size::all(Val::Px(100.0)),
                align_content: AlignContent::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            add_velocity_node(builder, font.clone());
            add_thrust_node(builder, font.clone());
        });

    builder.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Px(200.0), Val::Px(100.0)),
            ..default()
        },
        ..default()
    });

    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                gap: Size::height(Val::Px(5.0)),
                size: Size::all(Val::Px(100.0)),
                align_content: AlignContent::FlexEnd,
                justify_content: JustifyContent::FlexEnd,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            add_capacity_node(builder, font);
        });
}

fn add_capacity_node(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "/",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " kw",
                TextStyle {
                    font,
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
        CapacityUiComponent::default(),
    ));
}

pub fn update_capacity_node_on_capacitor_componend_changed(
    query: Query<&PowerCapacitorComponent, (With<Camera>, Changed<PowerCapacitorComponent>)>,
    mut ui_query: Query<&mut Text, With<CapacityUiComponent>>,
) {
    if let Ok(capacitor) = query.get_single() {
        if let Ok(mut text) = ui_query.get_single_mut() {
            text.sections[0].value = format!("{:.0}", capacitor.capacity);
            text.sections[2].value = format!("{:.0}", capacitor.capacity_max);
        }
    }
}

fn add_thrust_node(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "/",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " kwh",
                TextStyle {
                    font,
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            ..default()
        }),
        ThrustUiComponent::default(),
    ));
}

pub fn update_thrust_node_on_engine_component_changed(
    camera_query: Query<&Children, With<Camera>>,
    engine_changed_query: Query<&EngineComponent, Changed<EngineComponent>>,
    engine_query: Query<&EngineComponent>,
    mut ui_query: Query<&mut Text, With<ThrustUiComponent>>,
) {
    if let Ok(children) = camera_query.get_single() {
        if engine_changed_query.iter_many(children).next().is_none() {
            return;
        }

        let mut thrust = 0.0;
        let mut thrust_max = 0.0;

        if let Ok(mut text) = ui_query.get_single_mut() {
            for engine in engine_query.iter_many(children) {
                thrust += engine.forward_thrust;
                thrust_max += engine.forward_thrust_max;
            }

            text.sections[0].value = format!("{:.0}", thrust);
            text.sections[2].value = format!("{:.0}", thrust_max);
        }
    }
}

fn add_velocity_node(builder: &mut ChildBuilder, font: Handle<Font>) {
    builder.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "",
                TextStyle {
                    font: font.clone(),
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::new(
                " m/s",
                TextStyle {
                    font,
                    font_size: 15.0,
                    color: Color::WHITE,
                },
            ),
        ])
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            margin: UiRect::right(Val::Px(12.0)),
            ..default()
        }),
        VelocityUiComponent::default(),
    ));
}

pub fn update_velocity_node(
    mut ui_query: Query<&mut Text, With<VelocityUiComponent>>,
    mut velocity_query: Query<&Velocity, With<Camera>>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        if let Ok(velocity) = velocity_query.get_single_mut() {
            text.sections[0].value = format!("{:.2}", velocity.linvel.length());
        }
    }
}

fn add_module_to_module_nodes(
    builder: &mut ChildBuilder,
    details: &ModuleDetailsComponent,
    font: Handle<Font>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                size: Size::all(Val::Px(50.0)),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((TextBundle::from_sections([TextSection::new(
                details.icon.clone(),
                TextStyle {
                    font: font.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            )])
            .with_style(Style {
                margin: UiRect::right(Val::Px(8.0)),
                ..default()
            }),));
        });
}

pub fn update_modules(
    _query: Query<&Children, With<Camera>>,
    _child_query: Query<&ModuleDetailsComponent>,
) {
}
