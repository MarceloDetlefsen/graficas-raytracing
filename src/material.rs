use raylib::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Material {
    pub diffuse: Color,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            diffuse: Color::BLACK,
        }
    }
}
