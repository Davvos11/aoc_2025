use rustc_hash::FxHashMap;
use std::fmt::Debug;

type Content<T> = FxHashMap<(usize, usize), T>;

pub struct GridMap<T> {
    pub content: Content<T>,
}

impl<T> From<Content<T>> for GridMap<T> {
    fn from(content: Content<T>) -> Self {
        Self { content }
    }
}

impl<T: Debug + Copy> GridMap<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        self.content.get(&(x, y)).copied()
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(entry) = self.content.get_mut(&(x, y)) {
            *entry = value;
        }
    }

    pub fn remove(&mut self, x: usize, y: usize) {
        self.content.remove(&(x, y));
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> Vec<T> {
        let mut result = Vec::new();
        let neighbors = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];
        for (x_diff, y_diff) in neighbors {
            if let Some(x) = x.checked_add_signed(x_diff)
                && let Some(y) = y.checked_add_signed(y_diff)
                && let Some(row) = self.get(x, y)
            {
                result.push(row);
            }
        }
        result
    }

    pub fn iterate(&self) -> impl Iterator<Item = (usize, usize, T)> {
        self.content.iter().map(|((x, y), value)| (*x, *y, *value))
    }
}
