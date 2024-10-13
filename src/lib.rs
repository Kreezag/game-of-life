mod utils;

use std::fmt;
use wasm_bindgen::prelude::*;
use js_sys::Math::random;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;


fn main() {
    #[wasm_bindgen]
    pub struct Universe {
        width: u32,
        height: u32,
        cells: FixedBitSet,
    }

    #[wasm_bindgen]
    impl Universe {
        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

        pub fn cells(&self) -> *const usize {
            self.cells.as_slice().as_ptr()
        }

        pub fn get_index(&self, row: u32, column: u32) -> usize {
            (row * self.width + column) as usize
        }

        fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
            let mut count = 0;
            for delta_row in [self.height - 1, 0, 1].iter().cloned() {
                for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                    if delta_row == 0 && delta_col == 0 {
                        continue;
                    }

                    let neighbor_row = (row + delta_row) % self.height;
                    let neighbor_col = (column + delta_col) % self.width;
                    let idx = self.get_index(neighbor_row, neighbor_col);
                    count += self.cells[idx] as u8;
                }
            }
            count
        }

        pub fn tick(&mut self) {
            let mut next = self.cells.clone();

            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells[idx];
                    let live_neighbors = self.live_neighbor_count(row, col);

                    next.set(idx, match (cell, live_neighbors) {
                        (true, x) if x < 2 => false,
                        (true, 2) | (true, 3) => true,
                        (true, x) if x > 3 => false,
                        (false, 3) => true,
                        (otherwise, _) => otherwise
                    });

                    // let next_cell = match (cell, live_neighbors) {
                    //     // Rule 1: Any live cell with fewer than two live neighbours
                    //     // dies, as if caused by underpopulation.
                    //     (true, x) if x < 2 => false,
                    //     // Rule 2: Any live cell with two or three live neighbours
                    //     // lives on to the next generation.
                    //     (true, 2) | (true, 3) => true,
                    //     // Rule 3: Any live cell with more than three live
                    //     // neighbours dies, as if by overpopulation.
                    //     (true, x) if x > 3 => false,
                    //     // Rule 4: Any dead cell with exactly three live neighbours
                    //     // becomes a live cell, as if by reproduction.
                    //     (false, 3) => true,
                    //     // All other cells remain in the same state.
                    //     (otherwise, _) => otherwise,
                    // };

                    // next[idx] = next_cell;

                }
            }

            self.cells = next;
        }

        pub fn new(width: u32, height: u32, life_probability: f64) -> Universe {
            let size = (width * height) as usize;

            let mut cells = FixedBitSet::with_capacity(size);

            for n in 0..size {
                cells.set(n, random() >= life_probability)
            }

            Universe {
                width,
                height,
                cells,
            }
        }

        pub fn render(&self) -> String {
            self.to_string()
        }
    }

    impl fmt::Display for Universe {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for x in 0..self.cells.len() {
                write!(f, "{}", if self.cells[x] == false { '◻' } else { '◼' })?;
                if (x % self.width as usize == 0) {
                    write!(f, "\n")?;
                }
            }

            Ok(())
        }
    }
}