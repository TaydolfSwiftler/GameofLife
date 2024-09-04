mod core;

use std::time::Instant;
fn main() {
    let start = Instant::now();


    let run_time = Instant::now() - start;
    println!("Time elapsed: {:?}", run_time);
}
