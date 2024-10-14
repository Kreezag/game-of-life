mod utils;

use wasm_bindgen::prelude::*;

use js_sys::Math::random;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;


#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
}

fn main() {
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
    }

    impl Universe {
        pub fn set_cells(&mut self, input: FixedBitSet) {
            self.cells = input
        }
        pub fn get_cells(self) -> FixedBitSet {
            self.cells
        }
    }
}

