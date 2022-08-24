@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var tex_sampler: sampler;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@fragment
fn fragment(input: VertexOutput) -> @location(0) vec4<f32> {
    var output_color = textureSample(texture, tex_sampler, input.uv);
    return output_color;
}