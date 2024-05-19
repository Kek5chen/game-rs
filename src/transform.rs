use cgmath::{Deg, Matrix4, SquareMatrix, Vector3, Zero};

#[repr(C)]
pub struct Transform {
    pos: Vector3<f32>,
    rot: Vector3<f32>,
    scale: Vector3<f32>,
    pos_mat: Matrix4<f32>,
    rot_mat: Matrix4<f32>,
    scale_mat: Matrix4<f32>,
    combined_mat: Matrix4<f32>,
    invert_position: bool,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            pos: Vector3::zero(),
            rot: Vector3::zero(),
            scale: Vector3::new(1.0, 1.0, 1.0),
            pos_mat: Matrix4::identity(),
            rot_mat: Matrix4::identity(),
            scale_mat: Matrix4::identity(),
            combined_mat: Matrix4::identity(),
            invert_position: false,
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
        self.pos_mat = Matrix4::from_translation(pos);
    }

    fn recalculate_rot_matrix(&mut self) {
        self.rot_mat = Matrix4::from_angle_x(Deg(self.rot.x))
            * Matrix4::from_angle_y(Deg(self.rot.y))
            * Matrix4::from_angle_z(Deg(self.rot.z));
    }

    fn recalculate_scale_matrix(&mut self) {
        self.scale_mat = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);
    }

    fn recalculate_combined_matrix(&mut self) {
        self.combined_mat = self.pos_mat * self.rot_mat * self.scale_mat;
    }

    pub fn full_matrix(&self) -> &Matrix4<f32> {
        &self.combined_mat
    }
}