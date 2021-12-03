use std::fs::File;
use std::io::Read;

mod one;
mod two;

fn main() {
    let one = r("input/one.txt");
    println!("one, first: {}", one::first(one.clone()));
    println!("one, second: {}", one::second(one.clone()));
    let two = r("input/two.txt");
    println!("two, first: {}", two::first(two));
}

fn r(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
