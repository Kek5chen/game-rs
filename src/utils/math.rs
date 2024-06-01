use std::f32::consts::PI;

use nalgebra::{Matrix3, Matrix4, Vector3};

pub trait ExtraMatrixMath {
    fn decompose(self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>);
}

pub fn matrix_to_euler(matrix: Matrix3<f32>) -> Vector3<f32> {
    let sy = -matrix[(2, 0)];

    if sy.abs() > 1.0 - 1e-6 {
        // Gimbal lock detected, handle the singularity
        let x = 0.0f32;
        let y = PI / 2.0 * sy.signum();
        let z = y.atan2(-matrix[(1, 2)]);
        Vector3::new(x.to_degrees(), y.to_degrees(), z.to_degrees())
    } else {
        let x = matrix[(2, 1)].atan2(matrix[(2, 2)]);
        let y = sy.asin();
        let z = matrix[(1, 0)].atan2(matrix[(0, 0)]);
        Vector3::new(x.to_degrees(), y.to_degrees(), z.to_degrees())
    }
}

fn decompose_mat3(matrix: Matrix4<f32>) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
    let translation = matrix.column(3).xyz();

    let scale_x = matrix.column(0).xyz().norm();
    let scale_y = matrix.column(1).xyz().norm();
    let scale_z = matrix.column(2).xyz().norm();
    let scale = Vector3::new(scale_x, scale_y, scale_z);

    let rotation_matrix = Matrix3::from_columns(&[
        matrix.column(0).xyz() / scale_x,
        matrix.column(1).xyz() / scale_y,
        matrix.column(2).xyz() / scale_z,
    ]);

    let rotation = matrix_to_euler(rotation_matrix);

    (translation, rotation, scale)
}

impl ExtraMatrixMath for Matrix4<f32> {
    fn decompose(self) -> (Vector3<f32>, Vector3<f32>, Vector3<f32>) {
        decompose_mat3(self)
    }
}
