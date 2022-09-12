// bevy_pbr-0.8.1 > src > material.rs : 290
// bevy_pbr-0.8.1 > src > render > mesh.rs : 771
// bevy_render-0.8.1 > src > view > mod.rs : 96
struct View {
    view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    width: f32,
    height: f32,
}

struct Mesh {
    model: mat4x4<f32>,
    inverse_transpose_model: mat4x4<f32>,
    // 'flags' is a bit field indicating various options. u32 is 32 bits so we have up to 32 options.
    flags: u32,
};

@group(0) @binding(0)
var<uniform> view: View;
@group(2) @binding(0)
var<uniform> mesh: Mesh;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let mod_view = mat4x4<f32>(
        vec4<f32>(view.inverse_view[0].xyzw),
        vec4<f32>(view.inverse_view[1].xyzw),
        vec4<f32>(view.inverse_view[2].xyzw),
        vec4<f32>(0.0, 0.0, 0.0, 1.0)
    );

    // Demonstration purposes
    // let mod_view = mat4x4<f32>(
    //     vec4<f32>(view.view[0].xyzw),
    //     vec4<f32>(view.view[1].xyzw),
    //     vec4<f32>(view.view[2].xyzw),
    //     vec4<f32>(0.0, 0.0, 0.0, 1.0)
    // );

    out.world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    out.clip_position = view.projection * mod_view *  out.world_position;
    
    // Demonstration purposes
    //out.clip_position = view.view_proj * out.world_position;

    out.clip_position = vec4<f32>(out.clip_position.x, out.clip_position.y, 0.0000000000000001, out.clip_position.w);
    out.uv = vertex.uv;
    return out;
}

