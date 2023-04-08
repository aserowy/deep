use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::{AlphaMode, Color, Material},
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "ea2fa4ce-3adc-49e9-a476-ffabab248f8d"]
pub struct ForceFieldMaterial {
    #[uniform(0)]
    pub color: Color,
    pub alpha_mode: AlphaMode,
}

impl Material for ForceFieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "material/force_field.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;

        Ok(())
    }
}
