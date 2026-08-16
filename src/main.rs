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
    const GRIS_BOTON: Color = Color::new(60, 55, 50, 255);

    let head_center = Vector3::new(0.00, 1.28, 5.0);
    let head_radius = 1.00;
    // x alineado con la cabeza (antes 0.07) para que la línea de partición
    // blanco/negro del torso quede exactamente debajo de la de la cabeza,
    // sin el pequeño "quiebre" que se veía en el cuello.
    let body_center = Vector3::new(0.00, -0.12, 5.0);
    let body_radius = 1.15;

    // La cabeza se achicó (1.15 -> 1.00) y el torso creció (1.00 -> 1.15).
    // Todo lo que "cuelga" de cada uno (orejas/ojos/nariz de la cabeza;
    // panza/brazos/piernas del torso) se reescala en la misma proporción
    // para que guarden el mismo aspecto relativo que antes del cambio.
    let k_head = head_radius / 1.15; // = 0.8696
    let k_body = body_radius / 1.00; // = 1.15

    vec![
        // --- Orejas: base de un solo color + muesca del color opuesto ---
        Sphere {
            center: Vector3::new(-0.82 * k_head, head_center.y + 0.94 * k_head, 5.0),
            radius: 0.32 * k_head,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.68 * k_head, head_center.y + 0.78 * k_head, 5.0 - 0.15 * k_head),
            radius: 0.17 * k_head,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.82 * k_head, head_center.y + 0.94 * k_head, 5.0),
            radius: 0.32 * k_head,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.68 * k_head, head_center.y + 0.78 * k_head, 5.0 - 0.15 * k_head),
            radius: 0.17 * k_head,
            material: Material::solid(BLANCO),
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

        // --- Ojos: izquierdo negro (sobre lado blanco), derecho rojo (sobre lado negro) ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, -0.30 * k_head, 0.14 * k_head, 0.18 * k_head),
            radius: 0.16 * k_head,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.30 * k_head, 0.14 * k_head, 0.18 * k_head),
            radius: 0.16 * k_head,
            material: Material::solid(ROJO),
        },

        // --- Nariz: círculo blanco más grande y más abajo de los ojos,
        //     con el circulito negro también bajado un poco ---
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.02 * k_head, -0.20 * k_head, 0.26 * k_head),
            radius: 0.19 * k_head,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(head_center, head_radius, 0.02 * k_head, -0.18 * k_head, 0.44 * k_head),
            radius: 0.06 * k_head,
            material: Material::solid(NEGRO),
        },

        // --- Panza: borde negro (ligeramente más grande, más al fondo)
        //     detrás del círculo blanco, para que solo se asome como reborde ---
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.05 * k_body, 0.05 * k_body, 0.12 * k_body),
            radius: 0.44 * k_body,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.05 * k_body, 0.05 * k_body, 0.32 * k_body),
            radius: 0.40 * k_body,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: detail_on_sphere(body_center, body_radius, -0.05 * k_body, 0.05 * k_body, 0.38 * k_body),
            radius: 0.07 * k_body,
            material: Material::solid(GRIS_BOTON),
        },

        // --- Brazos: izquierdo blanco, derecho negro (igual que el cuerpo) ---
        Sphere {
            center: Vector3::new(-0.98 * k_body, body_center.y + 0.17 * k_body, 5.0),
            radius: 0.38 * k_body,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.98 * k_body, body_center.y + 0.17 * k_body, 5.0),
            radius: 0.38 * k_body,
            material: Material::solid(NEGRO),
        },

        // --- Piernas: izquierda blanca, derecha negra (igual que el cuerpo) ---
        Sphere {
            center: Vector3::new(-0.32 * k_body, body_center.y - 0.76 * k_body, 5.0),
            radius: 0.42 * k_body,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.32 * k_body, body_center.y - 0.76 * k_body, 5.0),
            radius: 0.42 * k_body,
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