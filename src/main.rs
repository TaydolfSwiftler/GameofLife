#![allow(dead_code)]
#![allow(unused_imports)]
mod core;

extern crate piston_window;
use crate::core::{Cell, Universe};
use piston_window::*;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    /*    let mut window: PistonWindow = WindowSettings::new("Piston Window", (2400, 1150))
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _d| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                rectangle([0.0, 0.0, 0.0, 1.0], // red
                          [100.0, 0.0, 10.0, 10.0], // rectangle
                          c.transform, g);
                rectangle([0.0, 0.0, 0.0, 1.0], // red
                          [200.0, 0.0, 50.0, 10.0], // rectangle
                          c.transform, g);
            });
            window.draw_2d(&e, |c, g, _d| {
                rectangle([0.0, 0.0, 0.0, 1.0], // red
                          [500.0, 0.0, 10.0, 10.0], // rectangle
                          c.transform, g);
                });

        }
    */
    #[rustfmt::skip]
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

    let mut universe = Universe::new_random(75, 150);

    universe.get_window();
    universe.tick();

    /*    universe.display();
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
    universe.display();*/

    let run_time = Instant::now() - start;
    println!("Time elapsed: {:?}", run_time);
}
