use std::time::Instant;

use file::{lines, split_by_section};

mod file;
mod node;

fn main() {
    let now = Instant::now();
    let tree = split_by_section(lines("./test/index.pre").unwrap());

    for node in tree {
        println!("{}", node.print("".to_string()));
    }

    let elapsed_time = now.elapsed();
    println!(">> {} microseconds.", elapsed_time.as_micros());
    println!();
}
