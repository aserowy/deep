use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;
use render::CustomRenderPlugin;
use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;

mod color;
mod render;
mod submarine;
mod terrain;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            watch_for_changes: true,
            ..default()
        }))
        // debug
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EditorPlugin::default())
        .add_system(bevy::window::close_on_esc)
        // game
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(CustomRenderPlugin::default())
        .add_plugin(TerrainPlugin::default())
        .add_plugin(SubmarinePlugin::default())
        .run();
}
