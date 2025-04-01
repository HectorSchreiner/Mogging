mod config;
mod global;
mod macros;
mod mogger;

use std::io::{self, stdout, BufWriter, Write};
use std::time::Instant;

use config::Config;
use crossterm::style::Print;
use crossterm::terminal::disable_raw_mode;
use crossterm::{execute, queue};
use global::*;
use mogger::Mogger;
use mogger::*;
use std::thread;

fn main() {
    benchmark();
}

fn benchmark() {
    let config = Config::builder().build();

    Mogger::new(config, LogFormat::PlainText).init();
    Mogger::create_default_mogger().init();
    let mut stdout = io::stdout();
    let amm = 100;
    let a = Instant::now(); // Start timer

    let mut writer = BufWriter::new(stdout);
    for _ in 0..amm {
        let msg = "Debug Log\n".as_bytes();
        writer.write_all(msg).unwrap();
    }

    writer.flush().unwrap(); // Ensure all data is written
    let duration_a = a.elapsed(); // Stop timer

    let b = Instant::now(); // Start timer
    for _ in 0..amm {
        println!("Debug Log");
    }
    let duration_b = b.elapsed(); // Stop timer

    let c = Instant::now(); // Start timer
    for _ in 0..amm {
        print!("Debug Log\n");
    }
    let duration_c = c.elapsed(); // Stop timer

    disable_raw_mode().unwrap();
    print!("{}[2J", 27 as char); // clear the screen
    println!();
    println!("Benchmark with: {} prints", amm);
    println!("_______________________________________________");
    println!("Mogger Logging took             : {:?}", duration_a);
    println!("Println Logging took            : {:?}", duration_b);
    println!("Print Logging took              : {:?}", duration_c);
}
