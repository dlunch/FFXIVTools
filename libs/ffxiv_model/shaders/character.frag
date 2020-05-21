#version 450

#include "frag_common.glsl"

layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform sampler Sampler;
layout(set = 0, binding = 2) uniform texture2D Normal;
layout(set = 0, binding = 3) uniform texture2D ColorTable;
layout(set = 0, binding = 4) uniform texture2D Mask;
layout(set = 0, binding = 5) uniform texture2D Specular;

void main() {
    vec4 normal = texture(sampler2D(Normal, Sampler), v_TexCoord);
    if(normal.b <= 0.5)
        discard;

    float key = (normal.a * 15.0 + 0.5) / 16.0;
    vec4 tex = texture(sampler2D(ColorTable, Sampler), vec2(0.125, key));
    o_Target = tex;
}
