use bevy::prelude::Color;
use rand::Rng;

pub fn random_color() -> Color {
    let mut rng = rand::rng();
    let r = rng.random_range(0.0..=1.0);
    let g = rng.random_range(0.0..=1.0);
    let b = rng.random_range(0.0..=1.0);
    Color::linear_rgba(r, g, b, 1.)
}
