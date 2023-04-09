#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::prepass_utils

#import deep::worley_noise_3d
#import deep::fresnel

struct ForceFieldMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ForceFieldMaterial;

@fragment
fn fragment(
    @builtin(position) frag_coord: vec4<f32>,
    @builtin(sample_index) sample_index: u32,
    @builtin(front_facing) is_front: bool,
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var result = worley(vec3<f32>(world_normal.xy, globals.time), 0.8);
    var alpha = (result[0] + result[1]) / 20.0;
    let fresnel = fresnel(view.world_position.xyz, world_position.xyz, world_normal, 10.0, 5.0);

    let offset = 0.6;
    let intersection_intensity = 15.0;
    let depth = prepass_depth(frag_coord, sample_index);

    var intersection = 1.0 - ((frag_coord.z - depth) * 100.0) - offset;
    intersection = smoothstep(0.0, 1.0, intersection);
    if is_front{
        intersection *= intersection_intensity;
    } else {
        intersection *= intersection_intensity / 2.0;
    }

    let color = vec3(material.color[0], material.color[1], material.color[2]) * alpha * 5.0;
    if is_front {
        return vec4(color, (fresnel * 0.5 + intersection) * material.color[3]);
    } else {
        return vec4(color, intersection * material.color[3]);
    }
}
