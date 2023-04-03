extern crate clap;

use clap::Parser;
use std::io;
use std::result::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

/// Drop-The-Line util helps to remove specific line from the given file
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Input {
    /// file
    #[arg(index = 1)]
    filename: String,

    /// line number
    #[arg(index = 2)]
    line_number: usize,

    /// force
    #[arg(short = 'f', default_value_t = false)]
    force: bool,
}

impl Input {
    fn verify(&self) -> bool {
        if self.line_number <= 0 {
            return false;
        }
        return true;
    }

    fn line_number(&self) -> usize {
        return self.line_number - 1;
    }
}

fn main() {
    let input = Input::parse();
    if !input.verify() {
        eprintln!("line number argument should starts from 1");
        return;
    }
    let mut lines = read(&input.filename).expect("Cannot read file");
    match ask(&input, &lines[input.line_number()]) {
        true => {
            lines.remove(input.line_number());
            let content = prepare_content(lines);
            let mut f = File::create(input.filename).expect("Cannot open file");
            f.write_all(content.as_bytes())
                .expect("Unable to write file");
        }
        false => (),
    }
}

fn prepare_content(lines: Vec<String>) -> String {
    let mut result = lines.join("\r\n");
    result.push_str("\r\n");
    return result;
}

fn ask(input: &Input, content: &String) -> bool {
    if input.force {
        return true;
    }
    println!("Are you sure? [y|n]\n{}", content);
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read input");
    return match user_input.as_str().trim() {
        "y" => true,
        "Y" => true,
        _ => false,
    };
}

fn read(filename: &String) -> Result<Vec<String>, String> {
    let f = File::open(filename);
    if !f.is_ok() {
        return Err(format!("Cannot open file {}", filename));
    }
    let buf = BufReader::new(f.unwrap());
    let lines: Vec<String> = buf
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect();
    return Ok(lines);
}
