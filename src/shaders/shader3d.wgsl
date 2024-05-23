struct VInput {
    @location(0) vpos: vec3<f32>,
    @location(1) vtex: vec2<f32>,
    @location(2) vnorm: vec3<f32>,
    @location(3) vtan: vec3<f32>,
    @location(4) vbitan: vec3<f32>,
}

struct VOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) vnorm: vec3<f32>,
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

struct Material {
    diffuse: vec3<f32>,
    _padding1: u32,
    use_diffuse_texture: u32,
    use_normal_texture: u32,
    shininess: f32,
    opacity: f32,
}

@group(0) @binding(0)
var<uniform> camera: CameraData;

@group(1) @binding(0)
var<uniform> model: ModelData;

@group(2) @binding(0)
var<uniform> material: Material;

@group(2) @binding(1)
var t_diffuse: texture_2d<f32>;

@group(2) @binding(2)
var s_diffuse: sampler;

@vertex
fn vs_main(in: VInput) -> VOutput {
    var out: VOutput;

    let mvp_matrix = camera.view_proj_mat * model.model_mat;

    out.position = mvp_matrix * vec4<f32>(in.vpos, 1.0);
    out.tex_coords = vec2<f32>(in.vtex.x, 1 - in.vtex.y);
    out.vnorm = in.vnorm;

    return out;
}

@fragment
fn fs_main(in: VOutput) -> @location(0) vec4<f32> {
    var diffuse: vec4<f32>;

    if material.use_diffuse_texture != 0 {
        diffuse = textureSample(t_diffuse, s_diffuse, in.tex_coords);
    } else {
        diffuse = vec4<f32>(material.diffuse, 1.0);
    }

    if diffuse.w < 0.8 {
        discard;
    }

    return diffuse;
}
