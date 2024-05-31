use nalgebra::{Affine3, Rotation3, Scale3, Translation3, Vector3};
use crate::object::GameObjectId;

#[repr(C)]
pub struct Transform {
    pos: Vector3<f32>,
    rot: Vector3<f32>,
    scale: Vector3<f32>,
    pos_mat: Translation3<f32>,
    rot_mat: Rotation3<f32>,
    scale_mat: Scale3<f32>,
    combined_mat: Affine3<f32>,
    invert_position: bool,
    owner: GameObjectId,
}

#[allow(dead_code)]
impl Transform {
    pub fn new(owner: GameObjectId) -> Self {
        Transform {
            pos: Vector3::zeros(),
            rot: Vector3::zeros(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            pos_mat: Translation3::identity(),
            rot_mat: Rotation3::identity(),
            scale_mat: Scale3::identity(),
            combined_mat: Affine3::identity(),
            invert_position: false,
            owner,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        self.pos = position;
        self.recalculate_pos_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.pos
    }

    pub fn set_invert_position(&mut self, invert: bool) {
        self.invert_position = invert;
    }

    pub fn translate(&mut self, other: Vector3<f32>) {
        self.pos += other;
        self.recalculate_pos_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        self.rot = rotation;
        self.recalculate_rot_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn rotation(&self) -> &Vector3<f32> {
        &self.rot
    }

    pub fn rotate(&mut self, rot: Vector3<f32>) {
        self.rot += rot;
        self.recalculate_rot_matrix();
        self.recalculate_combined_matrix();
    }

    pub fn set_nonuniform_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.recalculate_scale_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn set_uniform_scale(&mut self, factor: f32) {
        self.set_nonuniform_scale(Vector3::new(factor, factor, factor));
    }

    pub fn scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn regenerate_matrices(&mut self) {
        self.recalculate_pos_matrix();
        self.recalculate_rot_matrix();
        self.recalculate_scale_matrix();
        self.recalculate_combined_matrix();
    }

    fn recalculate_pos_matrix(&mut self) {
        let pos = if self.invert_position {
            -self.pos
        } else {
            self.pos
        };
        self.pos_mat = Translation3::from(pos);
    }

    fn recalculate_rot_matrix(&mut self) {
        self.rot_mat = Rotation3::from_euler_angles(
            self.rot.x.to_radians(),
            self.rot.y.to_radians(),
            self.rot.z.to_radians(),
        );
    }

    fn recalculate_scale_matrix(&mut self) {
        self.scale_mat = Scale3::from(self.scale);
    }

    fn recalculate_combined_matrix(&mut self) {
        self.combined_mat = Affine3::from_matrix_unchecked(
            self.pos_mat.to_homogeneous()
                * self.rot_mat.to_homogeneous()
                * self.scale_mat.to_homogeneous(),
        );
    }

    pub fn full_matrix(&self) -> &Affine3<f32> {
        &self.combined_mat
    }
}
