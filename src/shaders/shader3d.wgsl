struct VInput {
    @location(0) vpos: vec3<f32>,
    @location(1) vtex: vec2<f32>,
    @location(2) vnorm: vec3<f32>,
    @location(3) vtan: vec3<f32>,
    @location(4) vbitan: vec3<f32>,
}

struct VOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

struct CameraData {
    pos: vec3<f32>,
    rot: vec3<f32>,
    scale: vec3<f32>,
    view_mat: mat4x4<f32>,
    projection_mat: mat4x4<f32>,
    view_proj_mat: mat4x4<f32>,
}

struct ModelData {
    model_mat: mat4x4<f32>,
}

@group(0) @binding(0)
var<uniform> camera: CameraData;

@group(1) @binding(0)
var<uniform> model: ModelData;

@vertex
fn vs_main(in: VInput) -> VOutput {
    var out: VOutput;

    let mvp_matrix = camera.view_proj_mat * model.model_mat;

    out.position = mvp_matrix * vec4<f32>(in.vpos, 1.0);
    out.color = vec4<f32>(in.vnorm, 1.0);

    return out;
}

@fragment
fn fs_main(in: VOutput) -> @location(0) vec4<f32> {
    return in.color;
}
