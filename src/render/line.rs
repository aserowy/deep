use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::{
        shape::{Circle, RegularPolygon},
        Color, Material, Mesh, Vec3,
    },
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, PolygonMode, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError,
        },
    },
};

#[derive(Default, AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "a33a831d-3bac-4d3c-96bf-086bf480c268"]
pub struct LineMaterial {
    #[uniform(0)]
    pub color: Color,
}

impl Material for LineMaterial {
    fn fragment_shader() -> ShaderRef {
        "material/line.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
    pub close: bool,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        let mut vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        if line.close {
            vertices.push(vertices[0].clone());
        }

        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
    pub close: bool,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        let mut points: Vec<_> = line.points.into_iter().collect();
        if line.close {
            points.push(points[0].clone());
        }

        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, points);
        mesh
    }
}

impl From<RegularPolygon> for LineStrip {
    fn from(polygon: RegularPolygon) -> Self {
        let RegularPolygon { radius, sides } = polygon;

        debug_assert!(sides > 2, "RegularPolygon requires at least 3 sides.");

        let mut positions = Vec::with_capacity(sides);

        let step = std::f32::consts::TAU / sides as f32;
        for i in 0..sides {
            let theta = std::f32::consts::FRAC_PI_2 - i as f32 * step;
            let (sin, cos) = theta.sin_cos();

            positions.push([cos * radius, sin * radius, 0.0]);
        }

        Self {
            points: positions
                .iter()
                .map(|p| Vec3::new(p[0], p[1], p[2]))
                .collect(),
            close: true,
        }
    }
}

impl From<Circle> for LineStrip {
    fn from(value: Circle) -> Self {
        LineStrip::from(RegularPolygon::from(value))
    }
}
