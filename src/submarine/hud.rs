use bevy::{
    ecs::query::QueryManyIter,
    pbr::NotShadowCaster,
    prelude::{shape::Circle, *},
};
use bevy_rapier3d::prelude::Velocity;
use std::slice::Iter;
use uuid::Uuid;

use crate::{
    color::*,
    render::line::{LineMaterial, LineStrip},
};

use super::{
    module::{
        action::ChannelingComponent, engine::EngineComponent, shutdown::ModuleShutdownComponent,
        startup::ModuleStartupComponent, *, aftercast::ModuleAftercastComponent,
    },
    power::PowerCapacitorComponent,
};

#[derive(Default, Component)]
pub struct CapacityUiComponent {}

#[derive(Component)]
pub struct ModuleConsumptionUiComponent(Uuid);

#[derive(Component)]
pub struct ModuleCooldownUiComponent(Uuid);

#[derive(Component)]
pub struct ModuleIconUiComponent(Uuid);

#[derive(Default, Component)]
pub struct ThrustUiComponent {}

#[derive(Default, Component)]
pub struct VelocityUiComponent {}

pub fn setup(
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
        let font_bold = asset_server.load("fonts/monofur_bold.ttf");

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
                add_main_screen_column_nodes(builder, child_iter, font, font_bold);
            });
    }
}

fn add_main_screen_column_nodes(
    builder: &mut ChildBuilder,
    mut child_iter: QueryManyIter<&ModuleDetailsComponent, (), Iter<Entity>>,
    font: Handle<Font>,
    font_bold: Handle<Font>,
) {
    builder.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Column,
            size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
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
                add_module_to_module_nodes(builder, details, font.clone(), font_bold.clone());
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
    font_bold: Handle<Font>,
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
            builder.spawn((
                // TODO: icon into const with individual position offsets for center
                TextBundle::from_sections([TextSection::new(
                    details.icon.clone(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                )])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::left(Val::Px(9.0)),
                    ..default()
                }),
                ModuleIconUiComponent(details.id),
            ));

            builder.spawn((
                TextBundle::from_sections([TextSection::new(
                    "0",
                    TextStyle {
                        font: font_bold,
                        font_size: 20.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    },
                )])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Px(6.0)),
                    ..default()
                }),
                ModuleCooldownUiComponent(details.id),
            ));

            builder.spawn((
                TextBundle::from_sections([TextSection::new(
                    "0",
                    TextStyle {
                        font,
                        font_size: 15.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    },
                )])
                .with_style(Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::top(Val::Px(34.0)),
                    ..default()
                }),
                ModuleConsumptionUiComponent(details.id),
            ));
        });
}

pub fn reset_consumption_ui_component(
    mut query: Query<&mut Text, With<ModuleConsumptionUiComponent>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].style.color = TRANSPARENT;
    }
}

pub fn update_modules_consumption_by_module_startup(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<
        (&ModuleDetailsComponent, &ModuleStartupComponent),
        Changed<ModuleStartupComponent>,
    >,
    mut icon_query: Query<(&mut Text, &ModuleConsumptionUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let child_iter = child_query.iter_many(children);
        let mut query_iter = icon_query.iter_mut();
        for (details, startup) in child_iter {
            if let Some((mut text, _)) = query_iter.find(|cmp| cmp.1 .0 == details.id) {
                if let Some(power_needed) = startup.current_watt {
                    text.sections[0].value = format!("{:.0} kW", power_needed / 1000.0);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn update_modules_consumption_by_module_channeling(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<
        (&ModuleDetailsComponent, &ChannelingComponent),
        Changed<ChannelingComponent>,
    >,
    mut consumption_query: Query<(&mut Text, &ModuleConsumptionUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let child_iter = child_query.iter_many(children);
        let mut query_iter = consumption_query.iter_mut();
        for (details, channeling) in child_iter {
            if let Some((mut text, _)) = query_iter.find(|cmp| cmp.1 .0 == details.id) {
                if channeling.current_duration.is_some() {
                    text.sections[0].value = format!("-{:.0} kW", channeling.watt_per_second / 1000.0);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn reset_cooldown_ui_component(
    mut query: Query<&mut Text, With<ModuleCooldownUiComponent>>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].style.color = TRANSPARENT;
    }
}

pub fn update_modules_cooldown_by_module_shutdown(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<
        (&ModuleDetailsComponent, &ModuleShutdownComponent),
        Changed<ModuleShutdownComponent>,
    >,
    mut icon_query: Query<(&mut Text, &ModuleCooldownUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let child_iter = child_query.iter_many(children);
        let mut query_iter = icon_query.iter_mut();
        for (details, shutdown) in child_iter {
            if let Some((mut text, _)) = query_iter.find(|cmp| cmp.1 .0 == details.id) {
                if let Some(cooldown) = shutdown.current_spindown_time {
                    text.sections[0].value = format!("{:.0}", cooldown);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn update_modules_cooldown_by_module_aftercast(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<
        (&ModuleDetailsComponent, &ModuleAftercastComponent),
        Changed<ModuleAftercastComponent>,
    >,
    mut cooldown_query: Query<(&mut Text, &ModuleCooldownUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let child_iter = child_query.iter_many(children);
        let mut query_iter = cooldown_query.iter_mut();
        for (details, aftercast) in child_iter {
            if let Some((mut text, _)) = query_iter.find(|cmp| cmp.1 .0 == details.id) {
                if let Some(duration) = aftercast.current_spindown_time {
                    text.sections[0].value = format!("{:.0}", duration);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn update_modules_cooldown_by_module_channeling(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<
        (&ModuleDetailsComponent, &ChannelingComponent),
        Changed<ChannelingComponent>,
    >,
    mut cooldown_query: Query<(&mut Text, &ModuleCooldownUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let child_iter = child_query.iter_many(children);
        let mut query_iter = cooldown_query.iter_mut();
        for (details, channeling) in child_iter {
            if let Some((mut text, _)) = query_iter.find(|cmp| cmp.1 .0 == details.id) {
                if let Some(duration) = channeling.current_duration {
                    text.sections[0].value = format!("{:.0}", channeling.duration - duration);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn update_modules_by_module_state(
    camera_query: Query<&Children, With<Camera>>,
    child_query: Query<(&ModuleDetailsComponent, &ModuleStateComponent)>,
    mut icon_query: Query<(&mut Text, &ModuleIconUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let mut child_iter = child_query.iter_many(children);
        let icons = icon_query.iter_mut();

        for (mut icon, component) in icons {
            if let Some((_, state)) = child_iter.find(|cmp| cmp.0.id == component.0) {
                icon.sections[0].style.color = match state.state.status() {
                    ModuleStatus::Passive => UNITED_NATIONS_BLUE_25,
                    ModuleStatus::StartingUp => AQUAMARINE_25,
                    ModuleStatus::Active => AQUAMARINE,
                    ModuleStatus::Triggered => SLATE_BLUE,
                    ModuleStatus::Aftercast => SLATE_BLUE_25,
                    ModuleStatus::ShuttingDown => FRENCH_VIOLET_25,
                    ModuleStatus::Inactive => FRENCH_VIOLET,
                }
            }
        }
    }
}
