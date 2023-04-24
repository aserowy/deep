use bevy::prelude::*;
use bevy_rapier3d::prelude::Velocity;

use crate::submarine::{
    height::HeightPropertyComponent, module::engine::EngineComponent,
    power::PowerCapacitorComponent,
};

#[derive(Default, Component)]
pub struct CapacityUiComponent {}

#[derive(Default, Component)]
pub struct HeightUiComponent {}

#[derive(Default, Component)]
pub struct ThrustUiComponent {}

#[derive(Default, Component)]
pub struct VelocityUiComponent {}

pub fn setup(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let font = asset_server.load("fonts/monofur.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                size: Size::all(Val::Percent(100.0)),
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        gap: Size::height(Val::Px(5.0)),
                        size: Size::all(Val::Px(100.0)),
                        align_content: AlignContent::FlexEnd,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    add_velocity_node(builder, font.clone());
                    add_thrust_node(builder, font.clone());
                    add_height_node(builder, font.clone());
                });

            builder.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    size: Size::new(Val::Px(350.0), Val::Px(100.0)),
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
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    add_capacity_node(builder, font);
                });
        });
}

fn add_height_node(builder: &mut ChildBuilder, font: Handle<Font>) {
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
                " m",
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
        HeightUiComponent::default(),
    ));
}

pub fn update_height_node(
    mut ui_query: Query<&mut Text, With<HeightUiComponent>>,
    query: Query<&HeightPropertyComponent, With<Camera>>,
) {
    if let Ok(property) = query.get_single() {
        if let Ok(mut text) = ui_query.get_single_mut() {
            text.sections[0].value = format!("{:.2}", property.current_height);
        }
    }
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
                " Wh",
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
            text.sections[0].value = format!("{:.0}", capacitor.watt_hour);
            text.sections[2].value = format!("{:.0}", capacitor.watt_hour_max);
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
                " kN m/s",
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

        let mut force = 0.0;
        let mut force_max = 0.0;

        if let Ok(mut text) = ui_query.get_single_mut() {
            for engine in engine_query.iter_many(children) {
                force += engine.forward_force;
                force_max += engine.forward_force_max;
            }

            text.sections[0].value = format!("{:.0}", force / 1000.0);
            text.sections[2].value = format!("{:.0}", force_max / 1000.0);
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
