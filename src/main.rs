use rand::{self, prelude::*};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;
use std::time::Instant;

mod list;

const DEFAULT_LIST: &'static str = include_str!("100.txt");

type Word = String;

type Words = Vec<Word>;

type Memory = HashMap<usize, Words>;

#[derive(Default)]
struct Stats {
    correct: usize,
    failed: usize,
}

fn main() {
    let mut file_path = "/home/ow/dev/rust/mem100/stats.txt";
    if !Path::new(file_path).exists() {
        file_path = "stats.txt";
    }

    let mut stats_file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open(file_path)
        .unwrap();

    let mut rng = rand::thread_rng();
    let major = list::List::parse(DEFAULT_LIST);

    let mut numbers: Vec<usize> = major.keys();
    numbers.shuffle(&mut rng);

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!(
        "Major mnemonic system, training on list of {}",
        major.size()
    );

    let mut stats = Stats::default();
    let now = Instant::now();
    for number in numbers {
        let mut words = major.memorize_words(number);
        words.shuffle(&mut rng);

        let word = &words[0];

        answer_word(&mut stats, number, word);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    let stats_output = format!(
        "Stats correct {} failed {} elapsed {} minutes",
        stats.correct,
        stats.failed,
        now.elapsed().as_secs_f64() / 60.0
    );
    println!("{}", stats_output);
    if let Err(e) = writeln!(stats_file, "{}", stats_output) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn answer_word(stats: &mut Stats, number: usize, word: &str) {
    println!("Mnemonic number for word: {}", word);
    let mut attempt = 0;
    loop {
        if input::get_number() == number {
            stats.correct += 1;
            break;
        } else {
            stats.failed += 1;
            if attempt == 3 {
                println!("{} == {}", number, word);
                break;
            }
            println!("-");
            attempt += 1;
        }
    }
}

mod input {
    use std::io;

    pub fn get_word() -> String {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_buffer_read) => {}
            Err(_e) => {}
        }
        input.trim().to_string()
    }

    pub fn get_number() -> usize {
        loop {
            let input = get_word();
            match input.parse() {
                Ok(number) => return number,
                Err(e) => {
                    println!("Expected number, you provided [{}] err {}", input, e);
                    return get_number();
                }
            }
        }
    }
}
