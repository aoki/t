use billboard::Billboard;
use colored::*;
use spinners::{Spinner, Spinners};
use std::{thread::sleep, time::Duration};
use skim::prelude::*;
use std::io::Cursor;

fn main() {
    let sp = Spinner::new(&Spinners::Dots, format!("{}", "Hello, World!".red().bold()));
    sleep(Duration::from_secs(1));
    sp.stop();
    println!();

    println!("{} {} !", "it".green(), "works".blue().bold());
    Billboard::default().display(&format!("{}", "Hello, World!".red().bold()));


    let options = SkimOptionsBuilder::default()
    .height(Some("50%"))
    .multi(true)
    .build()
    .unwrap();

let input = "aaaaa\nbbbb\nccc".to_string();

// `SkimItemReader` is a helper to turn any `BufRead` into a stream of `SkimItem`
// `SkimItem` was implemented for `AsRef<str>` by default
let item_reader = SkimItemReader::default();
let items = item_reader.of_bufread(Cursor::new(input));

// `run_with` would read and show items from the stream
let selected_items = Skim::run_with(&options, Some(items))
    .map(|out| out.selected_items)
    .unwrap_or_else(|| Vec::new());

for item in selected_items.iter() {
    print!("{}{}", item.output(), "\n");
}
}
