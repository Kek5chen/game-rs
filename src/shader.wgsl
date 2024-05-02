struct VOutput {
    @location(0) v_color: vec4<f32>,
    @builtin(position) v_pos: vec4<f32>,
}


@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VOutput {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    let pos = vec4<f32>(x, y, 0.0, 1.0);
    var colors = array<vec4<f32>, 3>(
        vec4<f32>(1, 0, 0, 1),
        vec4<f32>(0, 1, 0, 1),
        vec4<f32>(0, 0, 1, 1),
    );

    var out: VOutput;
    out.v_color = colors[in_vertex_index];
    out.v_pos = pos;
    return out;
}

@fragment
fn fs_main(in: VOutput) -> @location(0) vec4<f32> {
    return in.v_color;
}
