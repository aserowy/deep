use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

use submarine::SubmarinePlugin;
use terrain::TerrainPlugin;

mod submarine;
mod terrain;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        // debug
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(EditorPlugin)
        // game
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(TerrainPlugin::default())
        .add_plugin(SubmarinePlugin::default())
        .add_system(bevy::window::close_on_esc)
        .run();
}
