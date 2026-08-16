use raylib::prelude::{Color, Vector3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaterialKind {
    Solid,
    SplitColor {
        color_left: Color,
        color_right: Color,
    },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub diffuse: Color,
    pub hit: bool,
    pub distance: f32,
    kind: MaterialKind,
}

impl Material {
    pub fn solid(diffuse: Color) -> Self {
        Self {
            diffuse,
            hit: false,
            distance: f32::INFINITY,
            kind: MaterialKind::Solid,
        }
    }

    pub fn split_color(color_left: Color, color_right: Color) -> Self {
        Self {
            diffuse: Color::BLACK,
            hit: false,
            distance: f32::INFINITY,
            kind: MaterialKind::SplitColor {
                color_left,
                color_right,
            },
        }
    }

    pub fn resolved_at(&self, center: &Vector3, hit_point: &Vector3, distance: f32) -> Self {
        let diffuse = match self.kind {
            MaterialKind::Solid => self.diffuse,
            MaterialKind::SplitColor {
                color_left,
                color_right,
            } => {
                let local_x = hit_point.x - center.x;
                if local_x < 0.0 {
                    color_left
                } else {
                    color_right
                }
            }
        };

        Self {
            diffuse,
            hit: true,
            distance,
            kind: self.kind,
        }
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::solid(Color::BLACK)
    }
}
