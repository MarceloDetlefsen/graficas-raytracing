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
) -> Color {
    for object in objects {
        let material = object.ray_intersect(ray_origin, ray_direction);

        if material.hit {
            return material.diffuse;
        }
    }
    Color::new(4, 12, 36, 255)
}


pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let fov = PI/3.0;
    let aspect_ratio = width / height;
    let perspective_scale = (fov / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;   // 0 .. 1
            let screen_y = 1.0 - (2.0 * y as f32) / height;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;
            
            let ray_direction = Vector3::new(screen_x, screen_y, 1.0).normalize();
            let ray_origin = Vector3::new(0.0, 0.0, 0.0);

            //
            let pixel_color = cast_ray(&ray_origin, &ray_direction, objects);
            framebuffer.set_current_color(pixel_color);
            framebuffer.set_pixel(x, y)
        }
    }
        
}

fn build_monokuma_scene() -> Vec<Sphere> {
    const BLANCO: Color = Color::new(245, 245, 245, 255);
    const NEGRO: Color = Color::new(25, 25, 25, 255);
    const ROJO: Color = Color::new(200, 30, 30, 255);

    vec![
        Sphere {
            center: Vector3::new(-0.32, 1.28, 4.7),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.16, 1.20, 4.7),
            radius: 0.08,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.0, 1.16, 4.7),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.16, 1.20, 4.7),
            radius: 0.08,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.32, 1.28, 4.7),
            radius: 0.08,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.40, 1.94, 4.7),
            radius: 0.06,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: Vector3::new(0.52, 1.80, 4.7),
            radius: 0.06,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: Vector3::new(-0.32, 1.78, 4.85),
            radius: 0.12,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.32, 1.78, 4.85),
            radius: 0.12,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.0, 1.56, 4.85),
            radius: 0.10,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.05, 0.04, 4.8),
            radius: 0.45,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(0.16, 0.18, 4.65),
            radius: 0.06,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: Vector3::new(-0.06, -0.10, 4.65),
            radius: 0.06,
            material: Material::solid(ROJO),
        },
        Sphere {
            center: Vector3::new(0.85, 0.15, 5.0),
            radius: 0.32,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.85, 0.15, 5.0),
            radius: 0.32,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(0.35, -0.85, 5.0),
            radius: 0.38,
            material: Material::solid(BLANCO),
        },
        Sphere {
            center: Vector3::new(-0.35, -0.85, 5.0),
            radius: 0.38,
            material: Material::solid(NEGRO),
        },
        Sphere {
            center: Vector3::new(-0.63, 2.48, 5.0),
            radius: 0.35,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: Vector3::new(0.63, 2.50, 5.0),
            radius: 0.35,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: Vector3::new(0.0, 1.70, 5.0),
            radius: 1.0,
            material: Material::split_color(BLANCO, NEGRO),
        },
        Sphere {
            center: Vector3::new(0.0, 0.0, 5.0),
            radius: 0.9,
            material: Material::split_color(BLANCO, NEGRO),
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

    framebuffer.set_background_color(Color::new(80, 80, 200, 255));
    let objects = build_monokuma_scene();

    while !window.window_should_close() {
        framebuffer.clear();

        render(&mut framebuffer, &objects);

        framebuffer.swap_buffers(&mut window, &raylib_thread);
    }
}
