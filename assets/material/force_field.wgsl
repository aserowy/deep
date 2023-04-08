#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::prepass_utils

#import deep::simplex_noise_3d
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
    var noise = simplexNoise3(vec3<f32>(world_normal.xy * 4.2, globals.time));
    var alpha = (noise + 1.0) / 2.0;
    let fresnel = fresnel(view.world_position.xyz, world_position.xyz, world_normal, 2.0, 1.0);

    let offset = 0.82;
    let intersection_intensity = 10.0;

    let depth = prepass_depth(frag_coord, sample_index);

    var intersection = 1.0 - ((frag_coord.z - depth) * 100.0) - offset;
    intersection = smoothstep(0.0, 1.0, intersection);
    if is_front{
        intersection *= intersection_intensity;
    } else {
        intersection *= intersection_intensity / 2.0;
    }

    let color = mix(vec3(1.00, 0.455, 0.827), vec3(1.00, 0.555, 0.927), 1.0) * (alpha + 0.5) * 5.0;
    if is_front {
        return vec4(color, fresnel * 0.4 + intersection);
    } else {
        return vec4(color, intersection);
    }
}
