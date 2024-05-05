@vertex
fn vs_main(@location(0) in_vpos: vec2<f32>) -> @builtin(position) {
  return vec4<f32>(in_vpos, 0.0, 1.0);
}

@fragment
fn fs_main() -> vec4<f32>() {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0); // RED
}
