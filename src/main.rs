// main.rs
#![allow(unused_imports)]
#![allow(dead_code)]

use raylib::prelude::*;
use std::f32::consts::PI;

mod framebuffer;
mod ray_intersect;
mod sphere;
mod material;

use framebuffer::Framebuffer;
use ray_intersect::RayIntersect;
use sphere::Sphere;
use material::Material;

pub fn cast_ray(
    ray_origin: &Vector3,
    ray_direction: &Vector3,
    objects: &[Sphere],
    background_color: Color,
) -> Color {
    let mut closest_distance = f32::INFINITY;
    let mut closest_color = background_color;

    for object in objects {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.hit && intersect.distance < closest_distance {
            closest_distance = intersect.distance;
            closest_color = intersect.diffuse;
        }
    }

    closest_color
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let fov = PI / 3.0;
    let aspect_ratio = width / height;
    let perspective_scale = (fov / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = 1.0 - (2.0 * y as f32) / height;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = Vector3::new(screen_x, screen_y, 1.0).normalize();
            let ray_origin = Vector3::new(0.0, 0.0, 0.0);

            let pixel_color = cast_ray(
                &ray_origin,
                &ray_direction,
                objects,
                framebuffer.background_color,
            );
            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y);
        }
    }
}

/// Coloca un detalle "pegado" a la superficie visible de una esfera padre,
/// en coordenadas locales (u = derecha, v = arriba) relativas a su centro,
/// empujado `protrusion` unidades hacia cámara para evitar z-fighting.
fn detail_on_sphere(
    parent_center: Vector3,
    parent_radius: f32,
    local_u: f32,
    local_v: f32,
    protrusion: f32,
) -> Vector3 {
    let x = parent_center.x + local_u;
    let y = parent_center.y + local_v;

    let dist_sq = local_u * local_u + local_v * local_v;
    let clamped = dist_sq.min(parent_radius * parent_radius * 0.90);
    let dz_surface = (parent_radius * parent_radius - clamped).sqrt();
    let surface_z = parent_center.z - dz_surface;

    Vector3::new(x, y, surface_z - protrusion)
}

fn build_monokuma_scene() -> Vec<Sphere> {
    const BLANCO: Color = Color::new(245, 245, 245, 255);
    const NEGRO: Color = Color::new(25, 25, 25, 255);
    const ROJO: Color = Color::new(200, 30, 30, 255);

    // Toda la figura bajada ~0.42 respecto a la versión anterior para que
    // quepa dentro del frustum visible (a z=5.0, con FOV PI/3, el rango
    // vertical visible es de aprox. ±2.89 unidades).
    let head_center = Vector3::new(0.00, 1.28, 5.0);
    let head_radius = 1.15;
    let body_center = Vector3::new(0.07, -0.12, 5.0);
    let body_radius = 1.00;

    vec![
        // --- Orejas (ahora dentro del frustum: top ≈ 2.52) ---
        Sphere {
            center: Vector3::new(-0.82, 2.22, 5.0),
            radius: 0.30,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: Vector3::new(0.82, 2.22, 5.0),
            radius: 0.30,
            material: Material::split_color(BLANCO, NEGRO),
        },

        // --- Cabeza y cuerpo (base) ---
        Sphere {
            center: head_center,
            radius: head_radius,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: body_center,
            radius: body_radius,
            material: Material::split_color(BLANCO, NEGRO),
        },

        // --- Ojos ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.30, 0.12, 0.18),
            radius: 0.16,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.30, 0.12, 0.18),
            radius: 0.16,
            material: Material::solid(BLANCO),
        },

        // --- Nariz ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.02, -0.12, 0.24),
            radius: 0.09,
            material: Material::solid(NEGRO),
        },

        // --- Boca (protrusion reforzada para que no la tape la curvatura) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.26, -0.33, 0.36),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.13, -0.37, 0.36),
            radius: 0.08,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.02, -0.39, 0.36),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.15, -0.37, 0.36),
            radius: 0.08,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.28, -0.33, 0.36),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },

        // --- Rayo rojo sobre el ojo derecho ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.42, 0.36, 0.30),
            radius: 0.045,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.50, 0.44, 0.30),
            radius: 0.045,
            material: Material::solid(ROJO),
        },

        // --- Panza sobre el cuerpo ---
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.05, 0.05, 0.20),
            radius: 0.38,
            material: Material::solid(BLANCO),
        },

        // --- Marca "X" roja sobre la panza ---
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.13, 0.13, 0.26),
            radius: 0.05,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: detail_on_sphere(body_center, body_radius, 0.10, -0.08, 0.26),
            radius: 0.05,
            material: Material::solid(ROJO),
        },

        // --- Brazos ---
        Sphere {
            center: Vector3::new(0.95, 0.00, 5.0),
            radius: 0.35,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.95, 0.00, 5.0),
            radius: 0.35,
            material: Material::solid(NEGRO),
        },

        // --- Piernas ---
        Sphere {
            center: Vector3::new(0.32, -0.90, 5.0),
            radius: 0.40,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.32, -0.90, 5.0),
            radius: 0.40,
            material: Material::solid(NEGRO),
        },
    ]
}

fn main() {
    let window_width = 1000;
    let window_height = 1000;

    let (mut window, raylib_thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raytracer Example")
        .log_level(TraceLogLevel::LOG_WARNING)
        .build();

    let mut framebuffer = Framebuffer::new(window_width as u32, window_height as u32);

    framebuffer.set_background_color(Color::new(230, 225, 210, 255));
    let objects = build_monokuma_scene();

    while !window.window_should_close() {
        framebuffer.clear();

        render(&mut framebuffer, &objects);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}