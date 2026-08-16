use raylib::prelude::Vector3;

use crate::material::Material;
use crate::sphere::Sphere;

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Material;
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vector3, ray_direction: &Vector3) -> Material {
        let oc = Vector3::new(
            ray_origin.x - self.center.x,
            ray_origin.y - self.center.y,
            ray_origin.z - self.center.z,
        );

        let a = ray_direction.x * ray_direction.x
            + ray_direction.y * ray_direction.y
            + ray_direction.z * ray_direction.z;
        let b = 2.0
            * (oc.x * ray_direction.x + oc.y * ray_direction.y + oc.z * ray_direction.z);
        let c = oc.x * oc.x + oc.y * oc.y + oc.z * oc.z - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return Material::default();
        }

        let sqrt_discriminant = discriminant.sqrt();
        let denom = 2.0 * a;
        let t1 = (-b - sqrt_discriminant) / denom;
        let t2 = (-b + sqrt_discriminant) / denom;

        let t = if t1 > 0.0 && t2 > 0.0 {
            t1.min(t2)
        } else if t1 > 0.0 {
            t1
        } else if t2 > 0.0 {
            t2
        } else {
            return Material::default();
        };

        let hit_point = Vector3::new(
            ray_origin.x + ray_direction.x * t,
            ray_origin.y + ray_direction.y * t,
            ray_origin.z + ray_direction.z * t,
        );

        if hit_point.z.is_nan() {
            return Material::default();
        }

        self.material.resolved_at(&self.center, &hit_point)
    }
}
