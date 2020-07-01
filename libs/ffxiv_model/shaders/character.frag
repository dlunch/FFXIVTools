#version 450

#include "frag_common.glsl"

layout(location = 0) in vec2 FragmentTexCoord;
layout(location = 1) in vec4 FragmentNormal;
layout(location = 2) in vec4 FragmentPosition;
layout(location = 3) in mat4 FragmentTBN;

layout(location = 0) out vec4 OutColor;

layout(set = 0, binding = 10) uniform sampler Sampler;
layout(set = 0, binding = 11) uniform texture2D Normal;
layout(set = 0, binding = 12) uniform texture2D ColorTable;
layout(set = 0, binding = 13) uniform texture2D Mask;
layout(set = 0, binding = 14) uniform texture2D Specular;

void main() {
    vec4 normalMap = texture(sampler2D(Normal, Sampler), FragmentTexCoord);
    if(normalMap.b <= 0.5)
        discard;

    float key = (normalMap.a * 15.0 + 0.5) / 16.0;
    vec4 diffuseMap = texture(sampler2D(ColorTable, Sampler), vec2(0.125, key));
    vec4 specularMap = texture(sampler2D(ColorTable, Sampler), vec2(0.375, key));

    vec3 color = calculateLight(FragmentPosition, FragmentTBN, diffuseMap, normalMap, specularMap, 32.0);
    OutColor = calculateGamma(color);
}
