#version 450

layout(location = 0) in vec2 v_TexCoord;
layout(location = 0) out vec4 o_Target;
layout(set = 0, binding = 1) uniform sampler Sampler;
layout(set = 0, binding = 2) uniform texture2D Normal;
layout(set = 0, binding = 3) uniform texture2D Diffuse;

void main() {
    vec4 normal = texture(sampler2D(Normal, Sampler), v_TexCoord);
    if(normal.b <= 0.5)
        discard;

    vec4 diffuse = texture(sampler2D(Diffuse, Sampler), v_TexCoord);

    o_Target = diffuse;
}
