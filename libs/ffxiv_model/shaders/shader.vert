#version 450

layout(location = 0) in vec4 Position;
layout(location = 1) in vec4 BoneWeight;
layout(location = 2) in vec4 BoneIndex;
layout(location = 3) in vec4 Normal;
layout(location = 4) in vec4 TexCoord;
layout(location = 5) in vec4 Tangent;
layout(location = 6) in vec4 Color;
layout(location = 0) out vec2 v_TexCoord;

layout(set = 0, binding = 0) uniform Locals {
    mat4 u_Transform;
};

void main() {
    v_TexCoord = TexCoord.xy;
    gl_Position = u_Transform * Position;
}
