use raylib::prelude::Vector3;

use crate::material::Material;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f32,
    pub material: Material,
}
