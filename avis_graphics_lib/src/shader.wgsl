


struct VertexInput {
	[[location(0)]] position: vec3<f32>;
};

struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(0)]] uv: vec2<f32>;
};

struct Uniforms {
	time: f32;
	audio_buffer_size: f32;
	audio_buffer: array<f32, 1024>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;


[[stage(vertex)]]
fn vertex_shader_main(
	model: VertexInput
) -> VertexOutput {
	var out: VertexOutput;

	out.clip_position = vec4<f32>(model.position, 1.0);
	out.uv = model.position.xy*0.5+0.5;

	return out;
}



[[stage(fragment)]]
fn fragment_shader_main(
	input: VertexOutput
) -> [[location(0)]] vec4<f32> {

	var x = uniforms.audio_buffer[u32(input.uv.x * uniforms.audio_buffer_size + 0.5)];
	// for (var i=-5; i < 6; i=i+1) {
	// 	x = x + abs(uniforms.audio_buffer[u32(round(input.uv.x * uniforms.audio_buffer_size + 0.5))]);
	// 	x = x/2.;
	// }

	// if (pow((xm+x+xn)*0.3, 4.0)*0.25 > input.uv.y) {
	if (x*15.0 > input.uv.y) {
		return vec4<f32>(1.0, 0.0, 0.0, 1.0);
	} else {
		return vec4<f32>(1.0, 0.5, 0.5, 1.0);
	}

}


