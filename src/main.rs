use file::{lines, split_by_section};

mod file;
mod node;

fn main() {
    for node in split_by_section(lines("./test/index.pre").unwrap()) {
        node.print("".to_string());
    }
    println!();
}
