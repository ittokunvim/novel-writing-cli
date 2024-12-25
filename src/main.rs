use std::fs::File;
use std::io::{
    prelude::*,
    stdin,
    stdout,
};
use std::thread::sleep;
use std::time::Duration;

const DURATION_MILLIS: u64 = 50;

fn main() {
    let mut f = File::open("novel.txt").expect("404");
    let mut contents = String::new();
    // read file
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    // loop text lines
    for (_, line) in contents.lines().enumerate() {
        // pause when line is empty
        if line.is_empty() { read_input() }
        // loop line chars and output 1 char at time
        for c in line.chars() {
            write_char(c);
        }
        print!("\n");
    }
}

fn read_input() {
    let mut input = String::new();

    print!("↩︎");
    stdout().flush().expect("flush to failed");
    stdin().read_line(&mut input).expect("failed to read line");
}

fn write_char(c: char) {
    print!("{c}");
    stdout().flush().expect("flush to failed");
    sleep(Duration::from_millis(DURATION_MILLIS));
}
