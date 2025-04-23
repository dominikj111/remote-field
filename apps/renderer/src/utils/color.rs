use bevy::prelude::Color;
use rand::Rng;

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0.0..=1.0);
    let g = rng.gen_range(0.0..=1.0);
    let b = rng.gen_range(0.0..=1.0);
    Color::rgba_from_array([r, g, b, 1.])
}
