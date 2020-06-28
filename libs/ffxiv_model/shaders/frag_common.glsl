struct PointLight {
	vec4 Position;
	vec4 Ambient;
	vec4 Diffuse;
	vec4 Specular;

	float Constant;
	float Linear;
	float Quadratic;
};

// lighting codes are from https://learnopengl.com/Lighting/Multiple-lights (CC BY-NC 4.0)

// calculates the color when using a point light.
vec3 CalcPointLight(in PointLight light, in vec3 fragPos, in vec3 viewDir, in vec3 diffuse, in vec3 normal, in vec3 specular, in float shininess) {
	vec3 lightDir = normalize(light.Position.xyz - fragPos);
	// diffuse shading
	float diff = max(dot(normal, lightDir), 0.0);
	// specular shading
	vec3 reflectDir = reflect(-lightDir, normal);
	float spec = pow(max(dot(viewDir, reflectDir), 0.0), shininess);
	// attenuation
	float distance = length(light.Position.xyz - fragPos);
	float attenuation = 1.0 / (light.Constant + light.Linear * distance + light.Quadratic * (distance * distance));
	// combine results
	vec3 resultAmbient = light.Ambient.rgb * diffuse;
	vec3 resultDiffuse = light.Diffuse.rgb * diff * diffuse;
	vec3 resultSpecular = light.Specular.rgb * spec * specular;
	resultAmbient *= attenuation;
	resultDiffuse *= attenuation;
	resultSpecular *= attenuation;
	return (resultAmbient + resultDiffuse + resultSpecular);
}

vec3 calculateLight(in vec4 fragmentPosition, in mat4 fragmentTBN, in vec4 diffuseMap, in vec4 normalMap, in vec4 specularMap, in float shininess) {
    // TODO WIP hardcode
    PointLight keyLight = PointLight(vec4(-4, 4, 4, 1), vec4(0.1, 0.1, 0.1, 1), vec4(1, 1, 1, 1), vec4(0, 0, 0, 1), 1.0f, 0.026f, 0.028f);
    PointLight fillLight = PointLight(vec4(2, 2, 3, 1), vec4(0, 0, 0, 1), vec4(1, 1, 1, 1), vec4(0, 0, 0, 1), 1.0f, 0.14f, 0.07f);
    PointLight backLight = PointLight(vec4(0, 3, -3, 1), vec4(0, 0, 0, 1), vec4(1, 1, 1, 1), vec4(0, 0, 0, 1), 1.0f, 0.045f, 0.0075f);
    vec4 eyePosition = vec4(0, 0.8, 2.5, 1);
    // hardcode end

	vec3 viewDir = normalize(eyePosition - fragmentPosition).xyz;
	vec3 normal = normalize(normalize(normalMap.xyz * 2.0 - 1.0) * mat3(fragmentTBN));

	vec3 result = vec3(0.0);
    result += CalcPointLight(keyLight, fragmentPosition.xyz, viewDir, diffuseMap.rgb, normal, specularMap.rgb, shininess);
    result += CalcPointLight(fillLight, fragmentPosition.xyz, viewDir, diffuseMap.rgb, normal, specularMap.rgb, shininess);
    result += CalcPointLight(backLight, fragmentPosition.xyz, viewDir, diffuseMap.rgb, normal, specularMap.rgb, shininess);

	return result;
}

vec4 calculateGamma(in vec3 color) {
#define GAMMA 2.2
    return vec4(pow(abs(color), vec3(1.0 / GAMMA)), 1.0);
}