mod config;
mod global;
mod macros;
mod mogger;

use std::io::{self, stdout, BufWriter, Write};
use std::time::Instant;

use config::{Config, LevelFormat, TimeFormat};
use crossterm::terminal::disable_raw_mode;
use global::*;
use mogger::Mogger;
use mogger::*;

fn main() {
    //benchmark();
    Mogger::default();

    debug!("Hello, World!");
    error!("Error Log here!")
}

fn _benchmark() {
    let config = Config::builder().build();

    Mogger::new(config, LogFormat::PlainText).init();
    Mogger::default();

    let amm = 100000;
    let a = Instant::now(); // Start timer

    let mut writer: BufWriter<io::Stdout> = BufWriter::new(stdout());
    for _ in 0..amm {
        let msg = "Debug Log\n".as_bytes();
        writer.write_all(msg).unwrap();
        writer.flush().unwrap(); // Ensure all data is written
    }
    stdout().flush().unwrap();

    let duration_a = a.elapsed(); // Stop timer
    let b = Instant::now(); // Start timer
    for _ in 0..amm {
        debug!("Debug Log");
    }
    stdout().flush().unwrap();
    let duration_b = b.elapsed(); // Stop timer

    let c = Instant::now(); // Start timer
    for _ in 0..amm {
        print!("Debug Log\n");
    }
    stdout().flush().unwrap();
    let duration_c = c.elapsed(); // Stop timer

    disable_raw_mode().unwrap();
    print!("{}[2J", 27 as char); // clear the screen

    println!();
    println!("Benchmark with: {} prints", amm);
    println!("_______________________________________________");
    println!("Bufwriter Logging took          : {:?}", duration_a);
    println!("Mogger Logging took             : {:?}", duration_b);
    println!("Print Logging took              : {:?}", duration_c);
}
