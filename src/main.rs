#![allow(dead_code)]
#![allow(unused_imports)]
mod core;

use crate::core::{Cell, Universe};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut universe = Universe::new(
        6,
        6,
        0,
        vec![
            Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Alive, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
            Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,
            Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,
        ],
    );

    universe.display();
    println!("-----------------");
    universe.tick();
    universe.display();
    println!("-----------------");
    universe.tick();
    universe.display();
    println!("-----------------");
    universe.tick();
    universe.display();
    println!("-----------------");
    universe.tick();
    universe.display();

    let run_time = Instant::now() - start;
    println!("Time elapsed: {:?}", run_time);
}
