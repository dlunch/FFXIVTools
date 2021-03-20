#version 450
#extension GL_EXT_samplerless_texture_functions : enable

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

    ivec2 normalSize = textureSize(Normal, 0);
    ivec2 coord = {int(FragmentTexCoord.x * normalSize.x), int(FragmentTexCoord.y * normalSize.y)};
    float colorTableKey = texelFetch(Normal, coord, 0).a;

    ivec2 colorTableSize = textureSize(ColorTable, 0);
    int colorTabley = int(colorTableKey * colorTableSize.y);
    vec4 diffuseMap = texelFetch(ColorTable, ivec2(0, colorTabley), 0);
    vec4 specularMap = texelFetch(ColorTable, ivec2(1, colorTabley), 0);

    OutColor = vec4(calculateLight(FragmentPosition, FragmentTBN, diffuseMap, normalMap, specularMap, 32.0), 1.0);
}
