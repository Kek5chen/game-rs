use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;

use cgmath::{Vector2, Vector3, Zero};
use itertools::izip;
use russimp::node::Node;
use russimp::scene::{PostProcess, Scene};
use russimp::Vector3D;

use crate::object::{GameObject, Object3D, Vertex3D};
use crate::world::World;

pub struct SceneLoader {}

impl SceneLoader {
    pub(crate) fn load(
        world: &mut World,
        path: &str,
    ) -> Result<Rc<RefCell<GameObject>>, Box<dyn Error>> {
        let scene = Scene::from_file(
            path,
            vec![
                PostProcess::CalculateTangentSpace,
                PostProcess::Triangulate,
                PostProcess::SortByPrimitiveType,
                PostProcess::JoinIdenticalVertices,
                PostProcess::GenerateUVCoords,
                PostProcess::GenerateNormals,
                PostProcess::ForceGenerateNormals,
            ],
        )?;

        let root = match &scene.root {
            Some(node) => node.clone(),
            None => return Ok(world.new_object("EmptyLoadedObject")),
        };

        let root_object = world.new_object(&root.name);
        Self::load_rec(world, &scene, &root, root_object.clone());
        Ok(root_object)
    }

    fn load_rec(
        world: &mut World,
        scene: &Scene,
        node: &Rc<Node>,
        node_obj: Rc<RefCell<GameObject>>,
    ) {
        Self::load_data(scene, &node, node_obj.clone());
        for child in node.children.borrow().iter() {
            let obj = world.new_object(&child.name);
            node_obj.borrow_mut().add_child(obj.clone());
            Self::load_rec(world, scene, child, obj);
        }
    }

    fn extract_data<T, O, F>(indices: &[u32], source: &[T], converter: F) -> Vec<O>
    where
        F: Fn(&T) -> O,
    {
        indices
            .iter()
            .filter_map(|&idx| source.get(idx as usize).map(&converter))
            .collect()
    }

    fn extend_data<T, O, F>(extendable: &mut Vec<T>, indices: &[u32], source: &[O], converter: F)
    where
        F: Fn(&O) -> T,
    {
        extendable.extend(Self::extract_data(indices, source, converter));
    }

    // hehe, idk either
    fn normalize_data<S, X, Y, Z, A>(
        scalar: &mut [S],
        x: &mut Vec<X>,
        y: &mut Vec<Y>,
        z: &mut Vec<Z>,
        a: &mut Vec<A>,
    ) where
        S: Zero + Clone,
        X: Zero + Clone,
        Y: Zero + Clone,
        Z: Zero + Clone,
        A: Zero + Clone,
    {
        if x.len() != scalar.len() {
            x.resize(scalar.len(), X::zero());
        }

        if y.len() != scalar.len() {
            y.resize(scalar.len(), Y::zero());
        }

        if z.len() != scalar.len() {
            z.resize(scalar.len(), Z::zero());
        }

        if a.len() != scalar.len() {
            a.resize(scalar.len(), A::zero());
        }
    }

    fn load_data(scene: &Scene, node: &Rc<Node>, node_obj: Rc<RefCell<GameObject>>) {
        let mut positions: Vec<Vector3<f32>> = Vec::new();
        let mut tex_coords: Vec<Vector2<f32>> = Vec::new();
        let mut normals: Vec<Vector3<f32>> = Vec::new();
        let mut tangents: Vec<Vector3<f32>> = Vec::new();
        let mut bitangents: Vec<Vector3<f32>> = Vec::new();

        const VEC3_FROM_VEC3D: fn(&Vector3D) -> Vector3<f32> =
            |v: &Vector3D| Vector3::new(v.x, v.y, v.z);
        const VEC2_FROM_VEC3D: fn(&Vector3D) -> Vector2<f32> =
            |v: &Vector3D| Vector2::new(v.x, v.y);

        for (_, mesh) in (0..)
            .zip(scene.meshes.iter())
            .filter(|(i, _)| node.meshes.contains(i))
        {
            for face in &mesh.faces {
                if face.0.len() != 3 {
                    continue; // ignore line and point primitives
                }
                let face_indices = &face.0;
                Self::extend_data(
                    &mut positions,
                    face_indices,
                    &mesh.vertices,
                    VEC3_FROM_VEC3D,
                );
                if let Some(Some(dif_tex_coords)) = mesh.texture_coords.first() {
                    Self::extend_data(
                        &mut tex_coords,
                        face_indices,
                        dif_tex_coords,
                        VEC2_FROM_VEC3D,
                    );
                }
                Self::extend_data(&mut normals, face_indices, &mesh.normals, VEC3_FROM_VEC3D);
                Self::extend_data(&mut tangents, face_indices, &mesh.tangents, VEC3_FROM_VEC3D);
                Self::extend_data(
                    &mut bitangents,
                    face_indices,
                    &mesh.bitangents,
                    VEC3_FROM_VEC3D,
                );
            }
        }

        // it does work tho
        Self::normalize_data(
            &mut positions,
            &mut tex_coords,
            &mut normals,
            &mut tangents,
            &mut bitangents,
        );

        let vertices = izip!(positions, tex_coords, normals, tangents, bitangents)
            .map(
                |(position, tex_coord, normal, tangent, bitangent)| Vertex3D {
                    position,
                    tex_coord,
                    normal,
                    tangent,
                    bitangent,
                },
            )
            .collect();

        node_obj.borrow_mut().drawable = Some(Object3D::new(vertices, None));
    }
}
