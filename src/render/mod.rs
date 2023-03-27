use bevy::prelude::{App, MaterialPlugin, Plugin};

use self::line::LineMaterial;

pub mod line;

pub struct CustomRenderPlugin {}

impl Default for CustomRenderPlugin {
    fn default() -> Self {
        CustomRenderPlugin {}
    }
}

impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<LineMaterial>::default());
    }
}
