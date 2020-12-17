#version 450
#extension GL_EXT_control_flow_attributes : enable

layout(location = 0) in vec4 Position;
layout(location = 1) in ivec4 BoneWeight;
layout(location = 2) in ivec4 BoneIndex;
layout(location = 3) in vec4 Normal;
layout(location = 4) in vec4 TexCoord;
layout(location = 5) in vec4 BiTangent;
layout(location = 6) in vec4 Color;

layout(location = 0) out vec2 FragmentTexCoord;
layout(location = 1) out vec4 FragmentNormal;
layout(location = 2) out vec4 FragmentPosition;
layout(location = 3) out mat4 FragmentTBN;

layout(set = 0, binding = 0) uniform MvpUniform {
    mat4 Mvp;
};
layout(set = 0, binding = 1) uniform BoneTransformsUniform {
	vec4 BoneTransforms[64 * 3];
};

void getPosition(out vec4 position, out vec4 normal) {
	position = vec4(0.0);
	normal = vec4(0.0);

	[[unroll]]
	for(int i = 0; i < 4; i ++)	{
		int index = BoneIndex[i];
		float weight = BoneWeight[i];
		mat4 boneTransform = mat4(BoneTransforms[index * 3], BoneTransforms[index * 3 + 1], BoneTransforms[index * 3 + 2], vec4(0, 0, 0, 1));
		position += Position * boneTransform * (weight / 255.0);
		normal += Normal * boneTransform * (weight / 255.0);
	}
}

void main() {
	vec4 normal, position;

	getPosition(position, normal);

    FragmentTexCoord = TexCoord.xy;
    FragmentNormal = normal;
    FragmentPosition = position;

	vec4 biTangent = normalize((BiTangent * 2.0 / 255.0) - 1.0);
	vec3 tangent = biTangent.a * cross(biTangent.xyz, normal.xyz);

    FragmentTBN = mat4(
		vec4(tangent.x, biTangent.x, normal.x, 0.0),
		vec4(tangent.y, biTangent.y, normal.y, 0.0),
		vec4(tangent.z, biTangent.z, normal.z, 0.0),
		vec4(0.0, 0.0, 0.0, 1.0)
	);

    gl_Position = Mvp * position;
}
