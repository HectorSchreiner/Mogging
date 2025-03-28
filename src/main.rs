mod config;
mod global;
mod macros;
mod mogger;

use std::fmt::format;
use std::io::{stdout, Write};
use std::time::Instant;

use config::Config;
use crossterm::style::Print;
use crossterm::terminal::disable_raw_mode;
use crossterm::{execute, queue};
use global::MOGGER;
use mogger::Mogger;
use mogger::*;

fn main() {
    let config = Config::builder().build();

    Mogger::new(config, LogFormat::PlainText).init();
    let amm = 10000;
    let a = Instant::now(); // Start timer
    for _ in 0..amm {
        execute!(stdout(), Print("someting")).unwrap();
    }

    let duration_a = a.elapsed(); // Stop timer

    let b = Instant::now(); // Start timer
    for _ in 0..amm {
        println!("Debug Log");
    }
    let duration_b = b.elapsed(); // Stop timer
    disable_raw_mode().unwrap();
    print!("{}[2J", 27 as char);
    println!();
    println!("Benchmark with: {} prints", amm);
    println!("Crossterm Logging a took : {:?}", duration_a);
    println!("Println Logging took     : {:?}", duration_b);
}
