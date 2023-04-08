use bevy::{asset::HandleId, prelude::*};

use self::{force_field::ForceFieldMaterial, line::LineMaterial};

pub mod force_field;
pub mod line;

pub const SIMPLEX_NOISE_3D: &str = include_str!("../../assets/shader/simplex_noise_3d.wgsl");
pub const FRESNEL: &str = include_str!("../../assets/shader/fresnel.wgsl");

#[derive(Default)]
pub struct CustomRenderPlugin {}

impl Plugin for CustomRenderPlugin {
    fn build(&self, app: &mut App) {
        app //
            .init_resource::<CustomShaderHandles>()
            .add_plugin(MaterialPlugin::<ForceFieldMaterial> {
                prepass_enabled: false,
                ..default()
            })
            .add_plugin(MaterialPlugin::<LineMaterial> {
                prepass_enabled: false,
                ..default()
            });
    }
}

#[allow(dead_code)]
#[derive(Resource)]
pub struct CustomShaderHandles {
    fresnel: HandleId,
    simplex_noise_3d: HandleId,
}

impl FromWorld for CustomShaderHandles {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();

        CustomShaderHandles {
            fresnel: load_shader(&mut shaders, "fresnel", FRESNEL),
            simplex_noise_3d: load_shader(&mut shaders, "simplex_noise_3d", SIMPLEX_NOISE_3D),
        }
    }
}

fn load_shader(
    shaders: &mut Mut<Assets<Shader>>,
    name: &str,
    shader_str: &'static str,
) -> HandleId {
    let mut shader = Shader::from_wgsl(shader_str);
    shader.set_import_path(format!("deep::{}", name));

    let handle_id = HandleId::random::<Shader>();
    shaders.set_untracked(handle_id, shader);

    handle_id
}
