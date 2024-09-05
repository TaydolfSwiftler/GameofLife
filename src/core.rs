// Grid Wraps on the Edges
// Use Vec for storing state
// index(row, column, universe) = row * width(universe) + column
use piston_window::*;
use rand;
use std::thread;
use std::time::Duration;

//Square size in Pixels, Distance to Window border in Pixels
const SQUARE_SIZE: u32 = 10;
const BORDER_DISTANCE: u32 = 1;
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

    pub fn new_empty(height: u32, width: u32) -> Universe {
        let cells = (0..height * width).map(|_| Cell::Dead).collect();
        Universe {
            height,
            width,
            iteration: 0,
            cells,
        }
    }

    pub fn new_random(height: u32, width: u32) -> Universe {
        let cells = (0..height * width)
            .map(|_| {
                if rand::random() {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();
        Universe {
            height,
            width,
            iteration: 0,
            cells,
        }
    }

    pub fn get_window(&mut self) {
        let window_height: u32 = self.height * SQUARE_SIZE + (self.height + 1) * BORDER_DISTANCE;
        let window_width: u32 = self.width * SQUARE_SIZE + (self.width + 1) * BORDER_DISTANCE;

        let mut window: PistonWindow =
            WindowSettings::new("Piston Window", (window_width, window_height))
                .exit_on_esc(true)
                .build()
                .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _d| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                for row in 0..self.height {
                    for col in 0..self.width {
                        let idx = self.get_index(row, col);
                        let color = match self.cells[idx] {
                            Cell::Dead => [0.0, 0.0, 0.0, 0.25],
                            Cell::Alive => [0.0, 0.0, 0.0, 1.0],
                        };
                        let square = rectangle::square(
                            (col * SQUARE_SIZE + (col + 1) * BORDER_DISTANCE) as f64,
                            (row * SQUARE_SIZE + (row + 1) * BORDER_DISTANCE) as f64,
                            SQUARE_SIZE as f64,
                        );
                        rectangle(color, square, c.transform, g);
                    }
                }
            });
            if let Some(Button::Keyboard(Key::Space)) = e.press_args() {
                self.tick();
            }
            if let Some(Button::Mouse(button)) = e.press_args() {
                self.tick();

            }
        }
    }
}

#[cfg(test)]
#[test]
fn test_live_neighbor_count() {
    let universe = Universe {
        height: 3,
        width: 3,
        iteration: 0,

        #[rustfmt::skip]
        cells: vec![
            Cell::Dead,  Cell::Alive, Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,
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
        #[rustfmt::skip]
        cells: vec![
            Cell::Dead,  Cell::Alive, Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,
            Cell::Dead,  Cell::Alive, Cell::Dead,
        ],
    };

    universe.tick();

    #[rustfmt::skip]
    let expected_cells = vec![
        Cell::Alive, Cell::Alive, Cell::Alive,
        Cell::Alive, Cell::Alive, Cell::Alive,
        Cell::Alive, Cell::Alive, Cell::Alive,
    ];
    assert_eq!(universe.cells, expected_cells);

    universe.tick();

    #[rustfmt::skip]
    let expected_cells = vec![
        Cell::Dead, Cell::Dead, Cell::Dead,
        Cell::Dead, Cell::Dead, Cell::Dead,
        Cell::Dead, Cell::Dead, Cell::Dead,
    ];
    assert_eq!(universe.cells, expected_cells);
}
