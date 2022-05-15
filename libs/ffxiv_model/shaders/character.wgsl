[[group(0), binding(10)]]
var textureSampler: sampler;
[[group(0), binding(11)]]
var normal_tex: texture_2d<f32>;
[[group(0), binding(12)]]
var color_table_tex: texture_2d<f32>;
[[group(0), binding(13)]]
var mask_tex: texture_2d<f32>;
[[group(0), binding(14)]]
var specular_tex: texture_2d<f32>;

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var normal_map: vec4<f32> = textureSample(normal_tex, textureSampler, in.tex_coord);

    var normal_size: vec2<i32> = textureDimensions(normal_tex, 0);
    var coord: vec2<i32> = vec2<i32>(i32(in.tex_coord.x * f32(normal_size.x)), i32(in.tex_coord.y * f32(normal_size.y)));
    var color_table_key: f32 = textureLoad(normal_tex, coord, 0).a;

    var color_table_size: vec2<i32> = textureDimensions(color_table_tex, 0);
    var color_table_y: i32 = i32(color_table_key * f32(color_table_size.y));
    var diffuse_map: vec4<f32> = textureLoad(color_table_tex, vec2<i32>(0, color_table_y), 0);
    var specular_map: vec4<f32> = textureLoad(color_table_tex, vec2<i32>(1, color_table_y), 0);

    if (normal_map.b <= 0.5) {
        discard;
    }

    var tbn: mat4x4<f32> = mat4x4<f32>(in.tbn0, in.tbn1, in.tbn2, in.tbn3);

    return vec4<f32>(calculate_light(in.world_position, tbn, diffuse_map, normal_map, specular_map, 32.0), 1.0);
}
