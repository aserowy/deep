use bevy::prelude::*;

use super::module::*;

mod crosshair;
pub mod information;
pub mod module;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&Children, With<Camera>>,
    child_query: Query<&ModuleDetailsComponent>,
) {
    info!("setup_hud");

    crosshair::setup(&mut commands, &asset_server);
    information::setup(&mut commands, &asset_server);
    module::setup(&mut commands, &asset_server, query, child_query);
}
