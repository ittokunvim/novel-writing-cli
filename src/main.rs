use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("novel.txt").expect("404");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("{contents}");
}
