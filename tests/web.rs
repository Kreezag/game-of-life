//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate fixedbitset;
use fixedbitset::FixedBitSet;


extern crate game_of_life;
use game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}


fn main() {
    #[cfg(test)]
    pub fn input_field() -> Universe {
        let mut universe = Universe::new(3, 3, 0.5);

        let size = (universe.width() * universe.height()) as usize;
        let mut bitset = FixedBitSet::with_capacity(size);

        for bit in 0..size {
            bitset.set(bit, true);

        }

        universe.set_cells(bitset);

        universe.tick();

        universe
    }
    #[cfg(test)]
    pub fn output_field() -> Universe {
        let mut universe = Universe::new(3, 3, 0.5);

        let size = (universe.width() * universe.height()) as usize;
        let mut output = FixedBitSet::with_capacity(size);

        for bit in 0..size {
            output.set(bit, false);
        }

        universe.set_cells(output);

        universe
    }

    #[wasm_bindgen_test]
    fn test_tick() {
        let mut input_universe = input_field();
        input_universe.tick();

        let input = input_universe.get_cells();

        let output = output_field().get_cells();

        assert_eq!(input.to_string(), output.to_string())
    }
}