use crate::object::{Vertex2D, Vertex3D};

macro_rules! v3 {
    ($x:expr, $y:expr, $z: expr) => {
        Vertex3D {
            position: cgmath::Vector3 {
                x: $x,
                y: $y,
                z: $z,
            }
        }
    };
}

macro_rules! v2 {
    ($x:expr, $y:expr) => {
        Vertex2D {
            position: cgmath::Vector2 {
                x: $x,
                y: $y,
            }
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
    v3!(0.5, 0.0, 0.0),
    v3!(1.0, 1.0, 0.0),
    v3!(0.0, 1.0, 0.0)
];

pub const CUBE: [Vertex3D; 8] = [
    v3!(-0.5, 0.5, 0.5),
    v3!(-0.5, 0.5, -0.5),
    v3!(-0.5, -0.5, 0.5),
    v3!(-0.5, -0.5, -0.5),
    
    v3!(0.5, 0.5, 0.5),
    v3!(0.5, 0.5, -0.5),
    v3!(0.5, -0.5, 0.5),
    v3!(0.5, -0.5, -0.5),
];

pub const CUBE_INDICES: [u32; 6 * 6] = [
    1, 2, 3,
    2, 3, 4,
    5, 6, 7,
    6, 7, 8,
    1, 5, 6,
    1, 6, 2,
    3, 7, 8,
    3, 8, 4,
    1, 5, 7,
    1, 7, 3,
    2, 6, 8,
    2, 8, 4
];