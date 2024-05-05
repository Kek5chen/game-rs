@vertex
fn vs_main(@location(0) in_vpos: vec3<f32>) -> @builtin(position) vec4<f32> {
  return vec4<f32>(in_vpos, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
  return vec4<f32>(1.0, 0.0, 0.0, 1.0); // RED
}
