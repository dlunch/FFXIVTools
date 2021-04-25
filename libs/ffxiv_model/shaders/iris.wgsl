[[group(0), binding(10)]]
var sampler: sampler;
[[group(0), binding(11)]]
var normal_tex: texture_2d<f32>;
[[group(0), binding(12)]]
var diffuse_tex: texture_2d<f32>;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var normal_map: vec4<f32> = textureSample(normal_tex, sampler, in.tex_coord);
    if (normal_map.a <= 0.5) {
        discard;
    }

    var diffuse_map: vec4<f32> = textureSample(diffuse_tex, sampler, in.tex_coord);

    return vec4<f32>(calculate_light(in.world_position, in.tbn, diffuse_map, normal_map, vec4<f32>(0.0, 0.0, 0.0, 1.0), 4.0), 1.0);
}
