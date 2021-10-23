struct point_light {
	position: vec4<f32>;
	ambient: vec4<f32>;
	diffuse: vec4<f32>;
	specular: vec4<f32>;

	constant: f32;
	linear: f32;
	quadratic: f32;
};

// lighting codes are from https://learnopengl.com/Lighting/Multiple-lights (CC BY-NC 4.0)

// calculates the color when using a point light.
fn calc_point_light(light: point_light, frag_pos: vec3<f32>, view_dir: vec3<f32>, diffuse: vec3<f32>, normal: vec3<f32>, specular: vec3<f32>, shininess: f32) -> vec3<f32> {
	var light_dir: vec3<f32> = normalize(light.position.xyz - frag_pos);
	// diffuse shading
	var diff: f32 = max(dot(normal, light_dir), 0.0);
	// specular shading
	var reflect_dir: vec3<f32>  = reflect(-light_dir, normal);
	var spec: f32 = pow(max(dot(view_dir, reflect_dir), 0.0), shininess);
	// attenuation
	var dist: f32 = length(light.position.xyz - frag_pos);
	var attenuation: f32 = 1.0 / (light.constant + light.linear * dist + light.quadratic * (dist * dist));
	// combine results
	var result_ambient: vec3<f32> = light.ambient.rgb * diffuse;
	var result_diffuse: vec3<f32> = light.diffuse.rgb * diff * diffuse;
	var result_specular: vec3<f32> = light.specular.rgb * spec * specular;

	return (result_ambient + result_diffuse + result_specular) * attenuation;
}

fn calculate_light(position: vec4<f32>, tbn: mat4x4<f32>, diffuse_map: vec4<f32>, normal_map: vec4<f32>, specular_map: vec4<f32>, shininess: f32) -> vec3<f32> {
    // TODO WIP hardcode
    var key_light: point_light;
	key_light.position = vec4<f32>(-4.0, 4.0, 4.0, 1.0); key_light.ambient = vec4<f32>(0.1, 0.1, 0.1, 1.0); key_light.diffuse = vec4<f32>(1.0, 1.0, 1.0, 1.0); key_light.specular = vec4<f32>(0.0, 0.0, 0.0, 1.0); key_light.constant = 1.0; key_light.linear = 0.026; key_light.quadratic = 0.028;

    var fill_light: point_light;
	fill_light.position = vec4<f32>(2.0, 2.0, 3.0, 1.0); fill_light.ambient = vec4<f32>(0.0, 0.0, 0.0, 1.0); fill_light.diffuse = vec4<f32>(1.0, 1.0, 1.0, 1.0); fill_light.specular = vec4<f32>(0.0, 0.0, 0.0, 1.0); fill_light.constant = 1.0; fill_light.linear = 0.14; fill_light.quadratic = 0.07;

    var back_light: point_light;
	back_light.position = vec4<f32>(0.0, 3.0, -3.0, 1.0); back_light.ambient = vec4<f32>(0.0, 0.0, 0.0, 1.0); back_light.diffuse = vec4<f32>(1.0, 1.0, 1.0, 1.0); back_light.specular = vec4<f32>(0.0, 0.0, 0.0, 1.0); back_light.constant = 1.0; back_light.linear = 0.045; back_light.quadratic = 0.0075;
    var eye_position: vec4<f32> = vec4<f32>(0.0, 0.8, 2.5, 1.0);
    // hardcode end

	var view_dir: vec3<f32> = normalize(eye_position - position).xyz;
	var normal: vec3<f32> = normalize(normalize(normal_map * 2.0 - 1.0) * tbn).xyz;

	var key_light_value: vec3<f32> = calc_point_light(key_light, position.xyz, view_dir, diffuse_map.rgb, normal, specular_map.rgb, shininess);
	var fill_light_value: vec3<f32> = calc_point_light(fill_light, position.xyz, view_dir, diffuse_map.rgb, normal, specular_map.rgb, shininess);
	var back_light_value: vec3<f32> = calc_point_light(back_light, position.xyz, view_dir, diffuse_map.rgb, normal, specular_map.rgb, shininess);

	return key_light_value + fill_light_value + back_light_value;
}
