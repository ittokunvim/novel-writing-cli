use std::fs::File;
use std::io::{
    prelude::*,
    stdin,
    stdout,
};
use std::thread::sleep;
use std::time::Duration;

use dialoguer::{theme::ColorfulTheme, Select};
use serde::Deserialize;

const PATH_JSON: &str = "data.json";
const DURATION_MILLIS: u64 = 50;

#[derive(Deserialize, Debug)]
struct Novel {
    title: String,
    path: String,
}

fn main() {
    println!("┌────────────────────────────┐");
    println!("│ program: novel writing cli │");
    println!("│ author: ittokunvim         │");
    println!("└────────────────────────────┘");
    // read json
    let novels = read_novel_json();
    // select novel
    let filename = select_novel(novels);
    // read file
    let contents = read_file(filename);
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

fn read_novel_json() -> Vec<Novel> {
    let json = std::fs::read_to_string(PATH_JSON).expect("JSON read failed");

    serde_json::from_str(&json).expect("JSON parse failed")
}

fn select_novel(novels: Vec<Novel>) -> String {
    let selections: Vec<String> = novels.iter().map(|novel| novel.title.clone()).collect();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a novel")
        .default(0)
        .items(&selections[..])
        .interact()
        .unwrap();

    novels[selection].path.clone()
}

fn read_file(filename: String) -> String {
    let mut f = File::open(filename).expect("404");
    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
 
    contents
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
