use std::fs::File;
use std::io::Read;

mod one;

fn main() {
    println!("{}", one::first(file_to_string("input/one.txt")));
    println!("{}", one::second(file_to_string("input/one.txt")));
}

fn file_to_string(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
