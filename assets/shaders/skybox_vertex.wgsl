
// struct View {
//     view_proj: mat4x4<f32>;
//     world_position: vec3<f32>;
// };


struct View {
    view_proj: mat4x4<f32>;
    view: mat4x4<f32>;
    inverse_view: mat4x4<f32>;
    projection: mat4x4<f32>;
    world_position: vec3<f32>;
    near: f32;
    far: f32;
    width: f32;
    height: f32;
};

[[group(0), binding(0)]]
var<uniform> view: View;


struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let mod_view = mat4x4<f32>(
        vec4<f32>(view.inverse_view[0].xyz, 0.0),
        vec4<f32>(view.inverse_view[1].xyz, 0.0),
        vec4<f32>(view.inverse_view[2].xyz, 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    out.clip_position = view.projection * mod_view *  vec4<f32>(vertex.position, 1.0);  
    //out.clip_position = vec4<f32>((view.view_proj * vec4<f32>(vertex.position, 1.0)).xyz + view.world_position, 1.0);
    out.clip_position = vec4<f32>(out.clip_position.x, out.clip_position.y, 0.0000000000000001, out.clip_position.w);
    out.uv = vertex.uv;
    return out;
}

