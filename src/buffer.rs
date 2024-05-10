use crate::object::{Vertex2D, Vertex3D};

macro_rules! v3 {
    ([$x:expr, $y:expr, $z: expr], [$r: expr, $g: expr, $b: expr], [$nx: expr, $ny: expr, $nz: expr]) => {
        Vertex3D {
            position: cgmath::vec3($x, $y, $z),
            color: cgmath::vec3($r, $g, $b),
            normal: cgmath::vec3($nx, $ny, $nz),
        }
    };
}

macro_rules! v2 {
    ($x:expr, $y:expr) => {
        Vertex2D {
            position: cgmath::Vector2 { x: $x, y: $y },
        }
    };
}

#[rustfmt::skip]
pub const TRIANGLE2D: [Vertex2D; 3] = [
    v2!(0.0, 1.0),
    v2!(1.0, -1.0),
    v2!(-1.0, -1.0)
];

#[rustfmt::skip]
pub const TRIANGLE: [Vertex3D; 3] = [
    Vertex3D { position: [0.0, 1.0, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex3D { position: [0.5, 0.0, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex3D { position: [-0.5, 0.0, 0.0], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
];

#[rustfmt::skip]
pub const CUBE: [Vertex3D; 24] = [  // 4 vertices * 6 faces = 24 vertices
    // Front face (z = -0.5)
    Vertex3D { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex3D { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex3D { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
    Vertex3D { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },

    // Back face (z = 0.5)
    Vertex3D { position: [-0.5,  0.5, 0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex3D { position: [ 0.5,  0.5, 0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex3D { position: [-0.5, -0.5, 0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },
    Vertex3D { position: [ 0.5, -0.5, 0.5], color: [0.0, 1.0, 0.0], normal: [0.0, 0.0, 1.0] },

    // Top face (y = 0.5)
    Vertex3D { position: [-0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex3D { position: [ 0.5,  0.5, -0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex3D { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },
    Vertex3D { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], normal: [0.0, 1.0, 0.0] },

    // Bottom face (y = -0.5)
    Vertex3D { position: [-0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },
    Vertex3D { position: [ 0.5, -0.5, -0.5], color: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },
    Vertex3D { position: [-0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },
    Vertex3D { position: [ 0.5, -0.5,  0.5], color: [1.0, 1.0, 0.0], normal: [0.0, -1.0, 0.0] },

    // Right face (x = 0.5)
    Vertex3D { position: [ 0.5,  0.5, -0.5], color: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
    Vertex3D { position: [ 0.5,  0.5,  0.5], color: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
    Vertex3D { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },
    Vertex3D { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0], normal: [1.0, 0.0, 0.0] },

    // Left face (x = -0.5)
    Vertex3D { position: [-0.5,  0.5, -0.5], color: [1.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex3D { position: [-0.5,  0.5,  0.5], color: [1.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex3D { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
    Vertex3D { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0], normal: [-1.0, 0.0, 0.0] },
];

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
