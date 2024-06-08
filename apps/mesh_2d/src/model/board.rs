use super::entity::Entity;
use std::fmt::Debug;

#[derive(Default, Debug, Clone)]
pub struct Board<DT: Clone + Copy> {
    plane: Vec<Vec<Option<Entity<DT>>>>,
    dimension: (usize, usize),
}

impl<DT: Clone + Copy> Board<DT> {
    pub fn new() -> Self {
        Self {
            plane: Vec::new(),
            dimension: (0, 0),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Entity<DT>> {
        if x >= self.plane.len() || y >= self.plane[x].len() {
            return None;
        }
        self.plane[x][y]
    }

    pub fn set_force(&mut self, value: Entity<DT>, force: bool) {
        let (x, y) = value.coordinates;
        if !force && self.get(x, y).is_some() {
            panic!("Overwriting board entity at ({}, {})", x, y);
        }
        if x >= self.plane.len() {
            self.plane.resize(x + 1, Vec::new());
        }
        if y >= self.plane[x].len() {
            self.plane[x].resize(y + 1, None);
        }
        self.dimension = (
            std::cmp::max(self.dimension.0, self.plane.len()),
            std::cmp::max(self.dimension.1, self.plane[x].len()),
        );
        self.plane[x][y] = Some(value);
    }

    pub fn set(&mut self, value: Entity<DT>) {
        self.set_force(value, false);
    }

    pub fn clear(&mut self, x: usize, y: usize) {
        if x < self.plane.len() && y < self.plane[x].len() {
            self.plane[x][y] = None;
        }
    }

    pub fn dimension(&self) -> (usize, usize) {
        self.dimension
    }
}

impl<DT: Clone + Copy + PartialEq> PartialEq for Board<DT> {
    fn eq(&self, other: &Self) -> bool {
        self.plane == other.plane && self.dimension == other.dimension
    }
}
