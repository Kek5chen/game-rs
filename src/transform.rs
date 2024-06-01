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
    compound_mat: Affine3<f32>,
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
            compound_mat: Affine3::identity(),
            invert_position: false,
            owner,
        }
    }

    pub fn set_position(&mut self, position: Vector3<f32>) {
        let mat = self.get_global_transform_matrix();
        self.set_local_position(mat.inverse_transform_vector(&position));
    }

    pub fn position(&self) -> Vector3<f32> {
        let mat = self.get_global_transform_matrix_ext(false);

        mat * self.pos
    }

    fn get_parent_list(&self) -> Vec<GameObjectId> {
        let mut parents = vec![];
        let mut parent_opt = Some(self.owner);

        while let Some(parent) = parent_opt {
            parents.push(parent);
            parent_opt = parent.parent;
        }
        parents.reverse();

        parents
    }

    pub fn get_global_transform_matrix_ext(&self, include_self: bool) -> Affine3<f32> {
        let mut mat = Affine3::identity();
        let mut parents = self.get_parent_list();

        if !include_self {
            parents.pop();
        }

        for parent in parents {
            mat *= parent.transform.compound_mat;
        }
        mat
    }

    pub fn get_global_transform_matrix(&self) -> Affine3<f32> {
        self.get_global_transform_matrix_ext(true)
    }

    pub fn get_global_rotation_matrix_ext(&self, include_self: bool) -> Rotation3<f32> {
        let mut mat = Rotation3::identity();
        let mut parents = self.get_parent_list();

        if !include_self {
            parents.pop();
        }

        for parent in parents {
            mat *= parent.transform.rot_mat;
        }
        mat
    }

    pub fn get_global_rotation_matrix(&self) -> Rotation3<f32> {
        self.get_global_rotation_matrix_ext(true)
    }

    pub fn get_global_scale_matrix_ext(&self, include_self: bool) -> Scale3<f32> {
        let mut mat = Scale3::identity();
        let mut parents = self.get_parent_list();

        if !include_self {
            parents.pop();
        }

        for parent in parents {
            mat *= parent.transform.scale_mat;
        }
        mat
    }

    pub fn get_global_scale_matrix(&self) -> Scale3<f32> {
        self.get_global_scale_matrix_ext(true)
    }

    pub fn set_local_position(&mut self, position: Vector3<f32>) {
        self.pos = position;
        self.recalculate_pos_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn local_position(&self) -> &Vector3<f32> {
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

    pub fn set_local_rotation(&mut self, rotation: Vector3<f32>) {
        self.rot = rotation;
        self.recalculate_rot_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn local_rotation(&self) -> &Vector3<f32> {
        &self.rot
    }

    pub fn set_rotation(&mut self, rotation: Vector3<f32>) {
        let desired_global_rotation =
            Rotation3::from_euler_angles(rotation.x.to_radians(), rotation.y.to_radians(), rotation.z.to_radians());

        let parent_global_rotation = self.get_global_rotation_matrix_ext(false);
        let local_rotation_change = parent_global_rotation.rotation_to(&desired_global_rotation);

        let (new_local_rotation_x, new_local_rotation_y, new_local_rotation_z) = local_rotation_change.euler_angles();

        self.set_local_rotation(Vector3::new(
            new_local_rotation_x.to_degrees(),
            new_local_rotation_y.to_degrees(),
            new_local_rotation_z.to_degrees(),
        ));
    }

    pub fn rotation(&self) -> Vector3<f32> {
        let global_rotation = self.get_global_rotation_matrix();
        let angles = global_rotation.euler_angles();
        println!("{:?}", &angles.1.to_degrees());
        Vector3::new(angles.0.to_degrees(), angles.1.to_degrees(), angles.2.to_degrees())
    }

    pub fn rotate(&mut self, rot: Vector3<f32>) {
        self.rot += rot;
        self.recalculate_rot_matrix();
        self.recalculate_combined_matrix();
    }

    pub fn set_nonuniform_local_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.recalculate_scale_matrix();
        self.recalculate_combined_matrix()
    }

    pub fn set_uniform_local_scale(&mut self, factor: f32) {
        self.set_nonuniform_local_scale(Vector3::new(factor, factor, factor));
    }

    pub fn local_scale(&self) -> &Vector3<f32> {
        &self.scale
    }

    pub fn set_nonuniform_scale(&mut self, scale: Vector3<f32>) {
        let global_scale = self.scale();
        let scale_delta = scale.component_div(&global_scale);
        let new_local_scale = self.scale.component_mul(&scale_delta);

        self.set_nonuniform_local_scale(new_local_scale);
    }

    pub fn set_uniform_scale(&mut self, factor: f32) {
        self.set_nonuniform_scale(Vector3::new(factor, factor, factor));
    }

    pub fn scale(&self) -> Vector3<f32> {
        let global_scale = self.get_global_scale_matrix();
        global_scale.vector
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
        self.compound_mat = Affine3::from_matrix_unchecked(
            self.pos_mat.to_homogeneous()
                * self.rot_mat.to_homogeneous()
                * self.scale_mat.to_homogeneous(),
        );
    }

    pub fn full_matrix(&self) -> &Affine3<f32> {
        &self.compound_mat
    }
}
