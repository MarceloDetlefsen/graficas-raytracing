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

/// Coloca un detalle "pegado" a la superficie visible (más cercana a cámara)
/// de una esfera padre, en coordenadas locales (u = derecha, v = arriba)
/// relativas al centro del padre, empujado `protrusion` unidades hacia cámara.
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
    let clamped = dist_sq.min(parent_radius * parent_radius * 0.98);
    let dz_surface = (parent_radius * parent_radius - clamped).sqrt();
    let surface_z = parent_center.z - dz_surface;

    Vector3::new(x, y, surface_z - protrusion)
}

fn build_monokuma_scene() -> Vec<Sphere> {
    const BLANCO: Color = Color::new(245, 245, 245, 255);
    const NEGRO: Color = Color::new(25, 25, 25, 255);
    const ROJO: Color = Color::new(200, 30, 30, 255);

    let head_center = Vector3::new(0.00, 1.70, 5.0);
    let head_radius = 1.15;
    let body_center = Vector3::new(0.07, 0.30, 5.0);
    let body_radius = 1.00;

    vec![
        // --- Orejas (independientes, no pegadas a la cabeza vía helper) ---
        Sphere {
            center: Vector3::new(-0.82, 2.64, 5.0),
            radius: 0.30,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: Vector3::new(0.82, 2.64, 5.0),
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

        // --- Ojos (capa 1: protrusion 0.18) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.32, 0.10, 0.18),
            radius: 0.17,
            material: Material::solid(NEGRO), // ojo negro sobre lado blanco
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.32, 0.10, 0.18),
            radius: 0.17,
            material: Material::solid(BLANCO), // ojo blanco sobre lado negro
        },

        // --- Nariz (capa 2: protrusion 0.20, debajo de los ojos) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.0, -0.15, 0.20),
            radius: 0.10,
            material: Material::solid(NEGRO),
        },

        // --- Boca (capa 3: protrusion 0.22, debajo de la nariz, arco de sonrisa) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.36, -0.36, 0.22),
            radius: 0.09,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.18, -0.40, 0.22),
            radius: 0.09,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.0, -0.42, 0.22),
            radius: 0.09,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.18, -0.40, 0.22),
            radius: 0.09,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.36, -0.36, 0.22),
            radius: 0.09,
            material: Material::solid(BLANCO),
        },

        // --- Rayo rojo sobre el ojo derecho (capa 4: protrusion 0.24, lejos del ojo) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.48, 0.34, 0.24),
            radius: 0.045,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.58, 0.42, 0.24),
            radius: 0.045,
            material: Material::solid(ROJO),
        },

        // --- Panza sobre el cuerpo (capa 1: protrusion 0.16) ---
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.05, 0.05, 0.16),
            radius: 0.48,
            material: Material::solid(BLANCO),
        },

        // --- Marca "X" roja sobre la panza (capa 2: protrusion 0.20) ---
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.15, 0.15, 0.20),
            radius: 0.06,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: detail_on_sphere(body_center, body_radius, 0.12, -0.10, 0.20),
            radius: 0.06,
            material: Material::solid(ROJO),
        },

        // --- Brazos ---
        Sphere {
            center: Vector3::new(1.05, 0.42, 5.0),
            radius: 0.35,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-1.05, 0.42, 5.0),
            radius: 0.35,
            material: Material::solid(NEGRO),
        },

        // --- Piernas ---
        Sphere {
            center: Vector3::new(0.38, -0.55, 5.0),
            radius: 0.40,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.38, -0.55, 5.0),
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