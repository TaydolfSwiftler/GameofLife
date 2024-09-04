// Grid Wraps on the Edges
// Use Vec for storing state
// index(row, column, universe) = row * width(universe) + column
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

pub struct Universe {
    height: u32,
    width: u32,
    iteration: u32,
    cells: Vec<Cell>,
}

impl Universe {
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    pub fn tick(&mut self) {
        self.iteration += 1;
        let mut next = self.cells.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                if self.live_neighbor_count(row, col) < 2 {
                    next[idx] = Cell::Dead;
                } else if self.live_neighbor_count(row, col) == 3 {
                    next[idx] = Cell::Alive;
                } else if self.live_neighbor_count(row, col) > 4 {
                    next[idx] = Cell::Dead;
                }
            }
        }
        self.cells = next;
    }
}

impl Universe {
    pub fn display(&self) {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                print!("{}", symbol);
            }
            println!();
        }
    }

    pub fn new(height: u32, width: u32, iteration: u32, cells: Vec<Cell>) -> Universe {
        Universe {
            height,
            width,
            iteration,
            cells,
        }
    }
}

#[cfg(test)]
#[test]
fn test_live_neighbor_count() {
    let mut universe = Universe {
        height: 3,
        width: 3,
        iteration: 0,
        cells: vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
        ],
    };

    assert_eq!(universe.live_neighbor_count(0, 0), 3);
    assert_eq!(universe.live_neighbor_count(1, 1), 2);
    assert_eq!(universe.live_neighbor_count(0, 1), 2);
    assert_eq!(universe.live_neighbor_count(1, 0), 3);
}

#[test]
fn test_get_index() {
    let universe = Universe {
        height: 3,
        width: 3,
        iteration: 0,
        cells: vec![Cell::Dead; 9],
    };

    assert_eq!(universe.get_index(0, 0), 0);
    assert_eq!(universe.get_index(1, 0), 3);
    assert_eq!(universe.get_index(0, 1), 1);
    assert_eq!(universe.get_index(1, 1), 4);
}

#[test]
fn test_tick() {
    let mut universe = Universe {
        height: 3,
        width: 3,
        iteration: 0,
        cells: vec![
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
            Cell::Dead,
            Cell::Alive,
            Cell::Dead,
        ],
    };

    universe.tick();
    let mut expected_cells = vec![
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
        Cell::Alive,
    ];
    assert_eq!(universe.cells, expected_cells);

    universe.tick();
    expected_cells = vec![
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
        Cell::Dead,
    ];
    assert_eq!(universe.cells, expected_cells);
}
