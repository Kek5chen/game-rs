use crate::asset_management::mesh::{SimpleVertex3D, Vertex3D};

#[allow(dead_code)]
#[rustfmt::skip]
pub const TRIANGLE: [Vertex3D; 3] = [
    SimpleVertex3D { position: [0.0, 1.0, 0.0], normal: [0.0, 0.0, -1.0] }.upgrade(),
    SimpleVertex3D { position: [0.5, 0.0, 0.0], normal: [0.0, 0.0, -1.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, 0.0, 0.0], normal: [0.0, 0.0, -1.0] }.upgrade(),
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const CUBE: [Vertex3D; 24] = [  // 4 vertices * 6 faces = 24 vertices
    // Front face (z = -0.5)
    SimpleVertex3D { position: [-0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5,  0.5, -0.5], normal: [0.0, 0.0, -1.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5, -0.5], normal: [0.0, 0.0, -1.0] }.upgrade(),

    // Back face (z = 0.5)
    SimpleVertex3D { position: [-0.5,  0.5, 0.5], normal: [0.0, 0.0, 1.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5,  0.5, 0.5], normal: [0.0, 0.0, 1.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5, 0.5], normal: [0.0, 0.0, 1.0] }.upgrade(),

    // Top face (y = 0.5)
    SimpleVertex3D { position: [-0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5,  0.5, -0.5], normal: [0.0, 1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5,  0.5,  0.5], normal: [0.0, 1.0, 0.0] }.upgrade(),

    // Bottom face (y = -0.5)
    SimpleVertex3D { position: [-0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5, -0.5], normal: [0.0, -1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5,  0.5], normal: [0.0, -1.0, 0.0] }.upgrade(),

    // Right face (x = 0.5)
    SimpleVertex3D { position: [ 0.5,  0.5, -0.5], normal: [1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5,  0.5,  0.5], normal: [1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5, -0.5], normal: [1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [ 0.5, -0.5,  0.5], normal: [1.0, 0.0, 0.0] }.upgrade(),

    // Left face (x = -0.5)
    SimpleVertex3D { position: [-0.5,  0.5, -0.5], normal: [-1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5,  0.5,  0.5], normal: [-1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, -0.5, -0.5], normal: [-1.0, 0.0, 0.0] }.upgrade(),
    SimpleVertex3D { position: [-0.5, -0.5,  0.5], normal: [-1.0, 0.0, 0.0] }.upgrade(),
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const CUBE_INDICES: [u32; 6 * 6] = [
    // Front face
    0, 1, 2, 1, 3, 2,
    // Back face
    4, 5, 6, 5, 7, 6,
    // Top face
    8, 9, 10, 9, 11, 10,
    // Bottom face
    12, 13, 14, 13, 15, 14,
    // Right face
    16, 17, 18, 17, 19, 18,
    // Left face
    20, 21, 22, 21, 23, 22,
];
