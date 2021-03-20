#version 450

#include "frag_common.glsl"

layout(location = 0) in vec2 FragmentTexCoord;
layout(location = 1) in vec4 FragmentNormal;
layout(location = 2) in vec4 FragmentPosition;
layout(location = 3) in mat4 FragmentTBN;

layout(location = 0) out vec4 OutColor;

layout(set = 0, binding = 10) uniform sampler Sampler;
layout(set = 0, binding = 11) uniform texture2D Normal;
layout(set = 0, binding = 12) uniform texture2D Diffuse;

void main() {
    vec4 normalMap = texture(sampler2D(Normal, Sampler), FragmentTexCoord);
    if(normalMap.b <= 0.5)
        discard;

    vec4 diffuseMap = texture(sampler2D(Diffuse, Sampler), FragmentTexCoord);

    OutColor = vec4(calculateLight(FragmentPosition, FragmentTBN, diffuseMap, normalMap, vec4(0, 0, 0, 1), 4.0), 1.0);
}
