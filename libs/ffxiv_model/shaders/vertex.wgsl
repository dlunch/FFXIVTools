struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coord: vec2<f32>,
	@location(1) normal: vec4<f32>,
	@location(2) world_position: vec4<f32>,

	// naga bug? we can't interpolate mat4x4
	@location(3) tbn0: vec4<f32>,
	@location(4) tbn1: vec4<f32>,
	@location(5) tbn2: vec4<f32>,
	@location(6) tbn3: vec4<f32>,
};

struct transform {
    model: mat4x4<f32>,
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};
struct bone_transform {
    transforms: array<vec4<f32>, 192u>,
};

@group(0) @binding(0)
var<uniform> transform: transform;
@group(0) @binding(1)
var<uniform> bone_transform: bone_transform;

@vertex
fn vs_main(
	@location(0) position: vec4<f32>,
	@location(1) bone_weight: vec4<u32>,
	@location(2) bone_index: vec4<u32>,
	@location(3) normal: vec4<f32>,
	@location(4) tex_coord: vec4<f32>,
	@location(5) bi_tangent: vec4<u32>,
	@location(6) color: vec4<u32>
) -> VertexOutput {
	var skinned_position: vec4<f32> = vec4<f32>(0.0);
	var skinned_normal: vec4<f32> = vec4<f32>(0.0);

	for (var i: u32 = 0u; i < 4u; i = i + 1u) {
		var index: u32 = bone_index[i];
		var weight: f32 = f32(bone_weight[i]) / 255.0;
		var boneTransform: mat4x4<f32> = mat4x4<f32>(
			bone_transform.transforms[index * 3u],
			bone_transform.transforms[index * 3u + 1u],
			bone_transform.transforms[index * 3u + 2u],
			vec4<f32>(0.0, 0.0, 0.0, 1.0)
		);
		skinned_position = skinned_position + position * boneTransform * weight;
		skinned_normal = skinned_normal + normal * boneTransform * weight;
	}

	var normalized_bi_tangent: vec4<f32> = normalize((vec4<f32>(bi_tangent) * 2.0 / 255.0) - 1.0);
	var tangent: vec3<f32> = normalized_bi_tangent.a * cross(normalized_bi_tangent.xyz, skinned_normal.xyz);

	var out: VertexOutput;
	out.tex_coord = tex_coord.xy;
	out.normal = skinned_normal;
	out.world_position = skinned_position;
	out.tbn0 = vec4<f32>(tangent.x, normalized_bi_tangent.x, skinned_normal.x, 0.0);
	out.tbn1 = vec4<f32>(tangent.y, normalized_bi_tangent.y, skinned_normal.y, 0.0);
	out.tbn2 = vec4<f32>(tangent.z, normalized_bi_tangent.z, skinned_normal.z, 0.0);
	out.tbn3 = vec4<f32>(0.0, 0.0, 0.0, 1.0);
	out.position = transform.projection * transform.view * transform.model * skinned_position;

	return out;
}
