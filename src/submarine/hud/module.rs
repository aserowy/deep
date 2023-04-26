use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    color::*,
    submarine::module::{
        action::ChannelingComponent, aftercast::ModuleAftercastComponent,
        requirement::{RequirementComponent, RequirementStatus}, startup::ModuleStartupComponent, ModuleDetailsComponent,
        ModuleStateComponent, ModuleStatus,
    },
};

pub fn setup(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    query: Query<&Children, With<Camera>>,
    module_query: Query<(&ModuleDetailsComponent, Option<&Children>)>,
    requirements_query: Query<&RequirementComponent>,
) {
    if let Ok(children) = query.get_single() {
        let font = asset_server.load("fonts/monofur.ttf");
        let font_bold = asset_server.load("fonts/monofur_bold.ttf");

        commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    size: Size::all(Val::Percent(100.0)),
                    padding: UiRect::bottom(Val::Px(42.0)),
                    gap: Size::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            })
            .with_children(|builder| {
                info!("Module ui building.");

                for (details, details_children) in module_query.iter_many(children) {
                    info!("Module ui with id {}.", details.id);

                    let requirements = if let Some(details_children) = details_children {
                        requirements_query.iter_many(details_children).collect()
                    } else {
                        vec![]
                    };

                    add_module_to_module_nodes(
                        builder,
                        details,
                        requirements,
                        font.clone(),
                        font_bold.clone(),
                    );
                }
            });
    }
}

#[derive(Component)]
pub struct ModuleConsumptionUiComponent(Uuid);

#[derive(Component)]
pub struct ModuleCooldownUiComponent(Uuid);

#[derive(Component)]
pub struct ModuleIconUiComponent(Uuid);

#[derive(Component)]
pub struct ModuleRequirementUiComponent(Uuid);

fn add_module_to_module_nodes(
    builder: &mut ChildBuilder,
    details: &ModuleDetailsComponent,
    requirements: Vec<&RequirementComponent>,
    font: Handle<Font>,
    font_bold: Handle<Font>,
) {
    builder
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::FlexStart,
                justify_content: JustifyContent::Center,
                size: Size {
                    width: Val::Px(55.0),
                    height: Val::Px(120.0),
                },
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
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
                    margin: UiRect::bottom(Val::Px(8.0)),
                    ..default()
                }),
                ModuleConsumptionUiComponent(details.id),
            ));

            builder
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::all(Val::Px(55.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::bottom(Val::Px(6.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    builder.spawn((
                        ImageBundle {
                            background_color: FRENCH_VIOLET.into(),
                            image: UiImage {
                                texture: details.icon.clone(),
                                ..default()
                            },
                            style: Style {
                                size: Size::all(Val::Percent(100.0)),
                                position_type: PositionType::Absolute,
                                ..default()
                            },
                            ..default()
                        },
                        ModuleIconUiComponent(details.id),
                    ));

                    builder.spawn((
                        TextBundle::from_sections([TextSection::new(
                            "0",
                            TextStyle {
                                font: font_bold,
                                font_size: 28.0,
                                color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                            },
                        )]),
                        ModuleCooldownUiComponent(details.id),
                    ));
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::height(Val::Px(18.0)),
                        margin: UiRect::bottom(Val::Px(8.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|builder| {
                    info!("RequirementComponents building.");

                    for requirement in requirements {
                        info!("RequirementComponent with id {}.", requirement.id);

                        builder.spawn((
                            ImageBundle {
                                background_color: FRENCH_VIOLET.into(),
                                image: UiImage {
                                    texture: requirement.icon.clone(),
                                    ..default()
                                },
                                style: Style {
                                    size: Size::all(Val::Px(18.0)),
                                    ..default()
                                },
                                ..default()
                            },
                            ModuleRequirementUiComponent(requirement.id),
                        ));
                    }
                });
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
                if let Some(power_needed) = startup.remaining_watt_hour {
                    text.sections[0].value = format!("{:.0} Wh", power_needed);
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
                    text.sections[0].value =
                        format!("-{:.0} kW", channeling.watt_per_second / 1000.0);
                    text.sections[0].style.color = Color::WHITE;
                }
            }
        }
    }
}

pub fn reset_cooldown_ui_component(mut query: Query<&mut Text, With<ModuleCooldownUiComponent>>) {
    for mut text in query.iter_mut() {
        text.sections[0].style.color = TRANSPARENT;
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
    mut bg_color_query: Query<(&mut BackgroundColor, &ModuleIconUiComponent)>,
) {
    if let Ok(children) = camera_query.get_single() {
        let mut child_iter = child_query.iter_many(children);
        for (mut background_color, component) in bg_color_query.iter_mut() {
            if let Some((_, state)) = child_iter.find(|cmp| cmp.0.id == component.0) {
                let color = match state.state.status() {
                    ModuleStatus::Passive => Color::WHITE,
                    ModuleStatus::StartingUp => TIFFANY_BLUE,
                    ModuleStatus::Active => GREEN,
                    ModuleStatus::ActiveInvalidTrigger => RED,
                    ModuleStatus::Triggered => AQUAMARINE,
                    ModuleStatus::Aftercast => AQUAMARINE_50,
                    ModuleStatus::ShuttingDown => TIFFANY_BLUE_50,
                    ModuleStatus::Inactive => FRENCH_VIOLET_50,
                };

                *background_color = color.into();
            }
        }
    }
}

pub fn update_modules_requirement_by_state(
    requirements_query: Query<&RequirementComponent>,
    mut ui_query: Query<(&mut BackgroundColor, &ModuleRequirementUiComponent)>,
) {
    let requirements: Vec<&RequirementComponent> = requirements_query.iter().collect();
    for (mut background_color, component) in ui_query.iter_mut() {
        if let Some(requirement) = requirements.iter().find(|rqrmnt| component.0 == rqrmnt.id) {
                let color = match requirement.status{
                    RequirementStatus::Fulfilled => GREEN,
                    RequirementStatus::Violated => RED,
                };

                *background_color = color.into();
        }
    }
}
