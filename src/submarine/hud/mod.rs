use bevy::prelude::*;

use super::module::{requirement::RequirementComponent, *};

pub mod condition;
mod crosshair;
pub mod information;
pub mod module;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Children, With<Camera>>,
    module_query: Query<(&ModuleDetailsComponent, Option<&Children>)>,
    requirements_query: Query<&RequirementComponent>,
) {
    info!("setup_hud");

    condition::setup(&mut commands);
    crosshair::setup(&mut commands, &asset_server);
    information::setup(&mut commands, &asset_server);
    module::setup(
        &mut commands,
        &asset_server,
        query,
        module_query,
        requirements_query,
    );
}
