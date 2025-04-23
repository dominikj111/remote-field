use bevy::prelude::Color;
use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub struct Entity<DT: Clone + Copy> {
    pub coordinates: (usize, usize),
    pub color: Color,
    pub data: Option<DT>,
}

impl<DT: Clone + Copy> Default for Entity<DT> {
    fn default() -> Self {
        Entity {
            coordinates: (0, 0),
            color: Color::BLACK,
            data: None,
        }
    }
}

impl<DT: Clone + Copy> Entity<DT> {
    pub fn new(coordinates: (usize, usize)) -> Self {
        Self {
            coordinates,
            ..Default::default()
        }
    }

    pub fn new_with_color(coordinates: (usize, usize), color: Color) -> Self {
        Self {
            coordinates,
            color,
            ..Default::default()
        }
    }

    pub fn new_with_data(coordinates: (usize, usize), data: DT) -> Self {
        Self {
            coordinates,
            data: Some(data),
            ..Default::default()
        }
    }
}

impl<DT: Clone + Copy + PartialEq> PartialEq for Entity<DT> {
    fn eq(&self, other: &Self) -> bool {
        self.coordinates == other.coordinates
            && self.color == other.color
            && match (&self.data, &other.data) {
                (Some(d1), Some(d2)) => d1 == d2,
                (None, None) => true,
                _ => false,
            }
    }
}
