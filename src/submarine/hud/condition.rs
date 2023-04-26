use bevy::prelude::*;
use uuid::Uuid;

use crate::{
    color::RED,
    submarine::module::{
        condition::{ConditionComponent, ConditionStatus},
        ModuleDetailsComponent,
    },
};

#[derive(Component, Default)]
pub struct ConditionRowUiComponent {}

pub fn setup(commands: &mut Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                padding: UiRect::top(Val::Px(385.0)),
                size: Size::all(Val::Percent(100.0)),
                gap: Size::all(Val::Px(10.0)),
                ..default()
            },
            ..default()
        },
        ConditionRowUiComponent::default(),
    ));
}

#[derive(Component)]
pub struct ConditionUiComponent(Uuid);

pub fn update_condition_row_ui_component(
    mut commands: Commands,
    children_query: Query<&Children, With<Camera>>,
    module_query: Query<&Children, With<ModuleDetailsComponent>>,
    condition_query: Query<&ConditionComponent>,
    ui_row_query: Query<(Entity, Option<&Children>), With<ConditionRowUiComponent>>,
    ui_entry_query: Query<(Entity, &ConditionUiComponent)>,
) {
    let mut active_conditions: Vec<&ConditionComponent> = vec![];
    if let Ok(children) = children_query.get_single() {
        condition_query
            .iter_many(children)
            .filter(|cndtn| cndtn.status == ConditionStatus::Active)
            .for_each(|cndtn| active_conditions.push(cndtn));

        module_query
            .iter_many(children)
            .flat_map(|mdl| condition_query.iter_many(mdl))
            .filter(|cndtn| cndtn.status == ConditionStatus::Active)
            .for_each(|cndtn| active_conditions.push(cndtn));
    }

    let (parent_entity, children) = ui_row_query.single();
    let mut current_entry_ids: Vec<Uuid> = vec![];
    if let Some(children) = children {
        ui_entry_query
            .iter_many(children)
            .inspect(|(_, ntry)| current_entry_ids.push(ntry.0))
            .filter(|(_, ntry)| active_conditions.iter().all(|cndtn| cndtn.id != ntry.0))
            .for_each(|(entity, _)| {
                commands.entity(parent_entity).remove_children(&[entity]);
                commands.entity(entity).despawn();
            });
    }

    active_conditions
        .iter()
        .filter(|cndtn| current_entry_ids.iter().all(|ntry| ntry != &cndtn.id))
        .for_each(|cndtn| spawn_condition_entry(&mut commands, cndtn, parent_entity));
}

fn spawn_condition_entry(
    commands: &mut Commands,
    condition: &ConditionComponent,
    parent_entity: Entity,
) {
    let child = commands
        .spawn((
            ImageBundle {
                background_color: RED.into(),
                image: UiImage {
                    texture: condition.icon.clone(),
                    ..default()
                },
                style: Style {
                    size: Size::all(Val::Px(33.0)),
                    ..default()
                },
                ..default()
            },
            ConditionUiComponent(condition.id),
        ))
        .id();

    commands.entity(parent_entity).add_child(child);
}
