use rand::seq::SliceRandom;

#[derive(Clone, Debug)]
pub struct Cell {
    // 0 -> 8 means that a cell is nearby a bomb; 255 means it is a bomb
    pub proximity: u8,
    pub hidden: bool,
}

pub struct BoardInstance {
    pub dim: (usize, usize),
    pub cells: Vec<Cell>,
    pub revealed_count: usize,
    pub mine_count: usize,
}

impl BoardInstance {
    /// Creates a new board
    pub fn init(dim: &(usize, usize), mine_count: usize) -> Self {
        let mut cells: Vec<Cell> = vec![
            Cell {
                proximity: 0,
                hidden: true
            };
            dim.0 * dim.1
        ];

        // Place mines randomly
        let mut rng = rand::thread_rng();
        let mut mine_indices: Vec<usize> = (0..cells.len()).collect();
        mine_indices.shuffle(&mut rng);
        for i in 0..mine_count {
            cells[mine_indices[i]].proximity = u8::MAX; // 255 signifies a bomb
        }

        // Update proximity counts for non-mine cells
        for i in 0..cells.len() {
            if cells[i].proximity != u8::MAX {
                let (x, y) = ((i % dim.0), (i / dim.0));

                let mut bombs_nearby: u8 = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;
                        if nx >= 0 && ny >= 0 && (nx as usize) < dim.0 && (ny as usize) < dim.1 {
                            let neighbor_index = (nx as usize) + (ny as usize) * dim.0;
                            if cells[neighbor_index].proximity == u8::MAX {
                                bombs_nearby += 1;
                            }
                        }
                    }
                }
                cells[i].proximity = bombs_nearby;
            }
        }
        Self {
            dim: *dim,
            cells,
            revealed_count: 0,
            mine_count,
        }
    }

    /// Reveals cells and returns what has been revealed
    pub fn reveal(&mut self, index: usize) -> Vec<(usize, u8)> {
        if index >= self.cells.len() {
            return vec![];
        }

        if self.cells[index].hidden {
            if self.cells[index].proximity == u8::MAX {
                return vec![];
            }
            self.cells[index].hidden = false;
            self.revealed_count += 1;
            let (x, y) = self.coord_from_index(index);
            let indices = [
                self.ix(x.saturating_sub(1), y),
                self.ix(x + 1, y),
                self.ix(x, y + 1),
                self.ix(x, y.saturating_sub(1)),
            ];
            let mut res = self.reveal(indices[0]);
            res.extend(self.reveal(indices[1]));
            res.extend(self.reveal(indices[2]));
            res.extend(self.reveal(indices[3]));
            res.push((index, self.cells[index].proximity));
            res
        } else {
            vec![]
        }
    }

    /// Returns a list of indices where a bomb is located
    pub fn get_bomb_positions(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.proximity == u8::MAX)
            .map(|(i, _)| i)
            .collect()
    }

    /// Returns true if all cells have been revealed
    pub fn revealed_all(&self) -> bool {
        self.cells.len() - self.revealed_count == self.mine_count
    }

    /// Given a coord it returns the corresponding cell index
    pub fn ix(&self, i: usize, j: usize) -> usize {
        //assert!(i < self.dim.0 && j < self.dim.1, "Index out of bounds");
        i + j * self.dim.0
    }

    /// Given a cell index, return its coordinates
    pub fn coord_from_index(&self, index: usize) -> (usize, usize) {
        assert!(index < self.cells.len(), "Index out of bounds");
        let x = index % self.dim.0;
        let y = index / self.dim.0;
        (x, y)
    }
}
