use std::fmt::Debug;

pub struct Grid<T> {
    pub content: Vec<Vec<T>>,
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(content: Vec<Vec<T>>) -> Self {
        Self { content }
    }
}

impl<T: Debug> Grid<T> {
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if y >= self.content.len() {
            return None;
        }
        if x >= self.content[0].len() {
            return None;
        }
        self.content.get(y).and_then(|row| row.get(x))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if y >= self.content.len() {
            return;
        }
        if x >= self.content[0].len() {
            return;
        }
        if let Some(entry) = self.content.get_mut(y).and_then(|row| row.get_mut(x)) {
            *entry = value;
        }
    }

    pub fn get_adjacent(&self, x: usize, y: usize) -> Vec<&T> {
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
            {
                // eprintln!("{x}, {y}");
                if let Some(row) = self.get(x, y) {
                    result.push(row);
                }
            }
        }
        // eprintln!("Adjacent for {x}, {y}: {result:?}");
        result
    }

    pub fn iterate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.content
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, value)| (x, y, value)))
    }
}
