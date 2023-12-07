use std::{fmt::Display, path::PathBuf};

use clap::{Parser, ValueEnum};

#[derive(PartialEq, Eq, Debug, ValueEnum, Clone, Copy, Hash)]
pub enum Part {
    Part1,
    Part2,
}
impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Part::*;
        write!(
            f,
            "{}",
            match self {
                Part1 => "Part 1",
                Part2 => "Part 2",
            }
        )
    }
}

#[derive(Parser, Debug)]
#[command(about,about="Launcher for Advent Of Code 2023 exercises",long_about=None)]
struct Cli {
    /// Exercise part to run.
    part: Part,

    /// Input filepath, defaults to day-XX/input.txt
    #[arg(short, long)]
    input: Option<PathBuf>,
}
// Read the command line arguments

fn read_command_line(input: &str) -> (Part, String, String) {
    let cli = Cli::parse();
    let (input, source) = if let Some(input) = cli.input {
        let source = input.as_path().display().to_string();
        let input = std::fs::read_to_string(input.clone())
            .unwrap_or_else(|_| panic!("Could not read file '{}'", source));
        (input, source)
    } else {
        (input.to_string(), String::from("input.txt"))
    };
    (cli.part, input, source)
}

fn prog() -> Option<String> {
    std::env::current_exe()
        .ok()?
        .file_name()?
        .to_str()?
        .to_owned()
        .into()
}

pub fn solve_aoc(input: &str, solver: fn(Part, &str) -> String) {
    let (part, input, source) = read_command_line(input);
    let exe_name = prog().unwrap_or(String::from("Unknown"));
    println!("Running '{}' {} on '{}'", exe_name, part, source);
    let result = solver(part, &input);
    println!("Result: {result}");
}
