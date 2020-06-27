#version 450

layout(location = 0) in vec4 Position;
layout(location = 1) in vec4 BoneWeight;
layout(location = 2) in vec4 BoneIndex;
layout(location = 3) in vec4 Normal;
layout(location = 4) in vec4 TexCoord;
layout(location = 5) in vec4 BiTangent;
layout(location = 6) in vec4 Color;

layout(location = 0) out vec2 FragmentTexCoord;
layout(location = 1) out vec4 FragmentNormal;
layout(location = 2) out vec4 FragmentPosition;
layout(location = 3) out mat4 FragmentTBN;

layout(set = 0, binding = 0) uniform Mvp {
    mat4 u_Transform;
};

void main() {
    FragmentTexCoord = TexCoord.xy;
    FragmentNormal = Normal;
    FragmentPosition = Position;

	vec4 biTangent = normalize((BiTangent * 2.0 / 255.0) - 1.0);
	vec3 tangent = biTangent.a * cross(biTangent.xyz, Normal.xyz);

    FragmentTBN = mat4(
		vec4(tangent.x, biTangent.x, Normal.x, 0.0),
		vec4(tangent.y, biTangent.y, Normal.y, 0.0),
		vec4(tangent.z, biTangent.z, Normal.z, 0.0),
		vec4(0.0, 0.0, 0.0, 1.0)
	);

    gl_Position = u_Transform * Position;
}
