use std::fs::File;
use std::io::{
    prelude::*,
    stdout,
};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut f = File::open("novel.txt").expect("404");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    for c in contents.chars() {
        print!("{c}");
        stdout().flush().expect("flushing to succeed");
        sleep(Duration::from_millis(50));
    }
}
