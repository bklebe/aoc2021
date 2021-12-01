use std::fs::File;
use std::io::Read;

mod one;

fn main() {
    println!("{}", one::first(r("input/one.txt")));
    println!("{}", one::second(r("input/one.txt")));
}

fn r(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
