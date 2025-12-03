use crate::day::days;
use clap::Parser;
use dotenv::dotenv;
use regex::Regex;
use reqwest::blocking::Client;
use reqwest::header::{COOKIE, USER_AGENT};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::{env, fs, io};

pub mod day;
#[cfg(test)]
mod test;

#[derive(Parser)]
struct Cli {
    /// The day to run
    day: usize,
    /// Set up template and download puzzle for this day
    #[arg(long, default_value_t = false)]
    download: bool,
}

fn main() {
    let args = Cli::parse();
    load_dot_env().expect("Failed to load .env");

    let puzzle_file = format!("puzzles/day{:02}.txt", args.day);

    if args.download {
        copy_templates(args.day).expect("Failed to copy templates");
        check_puzzle_file(&puzzle_file, args.day);
    } else {
        let days = days();
        if let Some(constructor) = days.get(args.day - 1) {
            check_puzzle_file(&puzzle_file, args.day);
            let puzzle_input =
                fs::read_to_string(&puzzle_file).expect("Failed to read puzzle input");
            let day = constructor(puzzle_input);
            day.run();
        } else {
            eprintln!(
                "Day {} not found, run using --download to add this day",
                args.day
            );
        }
    }
}

fn check_puzzle_file(file: &str, day: usize) {
    if !file_exists(file) {
        eprintln!("{file} does not exist, downloading...");
        if let Ok(session) = env::var("AOC_SESSION")
            && !session.is_empty()
        {
            download_puzzle(2025, day, &session).expect("Failed to download puzzle");
        } else {
            panic!("Please set AOC_SESSION in .env");
        }
    }
}

fn file_exists(file: &str) -> bool {
    Path::new(file).exists()
}

fn copy_templates(day: usize) -> io::Result<()> {
    let struct_str = format!("Day{day:02}");
    let fn_str = format!("day{day:02}");

    let day_file = format!("src/day/day{day:02}.rs");
    if file_exists(&day_file) {
        eprintln!("{day_file} already exists");
    } else {
        eprintln!("Creating {day_file}");
        fs::copy("src/day/template.rs", &day_file)?;
        replace_in_file(&day_file, vec![("DayXX", &struct_str)])?;
        add_mod("src/day.rs", "day", day)?;
        add_day("src/day.rs", day)?;
    }

    let test_file = format!("src/test/test{day:02}.rs");
    if file_exists(&test_file) {
        eprintln!("{test_file} already exists");
    } else {
        eprintln!("Creating {test_file}");
        fs::copy("src/test/template.rs", &test_file)?;
        replace_in_file(&test_file, vec![("DayXX", &struct_str), ("dayXX", &fn_str)])?;
        add_mod("src/test.rs", "test", day)?;
    }

    Ok(())
}

fn replace_in_file(file: &str, find_replace: Vec<(&str, &str)>) -> Result<(), io::Error> {
    let mut contents = fs::read_to_string(file)?;
    for (find, replace) in find_replace {
        contents = contents.replace(find, replace)
    }

    let mut file = OpenOptions::new().write(true).truncate(true).open(file)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}
fn add_mod(file: &str, prefix: &str, day: usize) -> io::Result<()> {
    let regex = format!(r"pub mod {prefix}(\d+);");
    let new_line = format!("pub mod {prefix}{day:02};");

    add_line(file, &regex, &new_line)
}

fn add_day(file: &str, day: usize) -> io::Result<()> {
    let regex = r"        |input| Box::new(day(\d+)::Day(\d+)::new(input)),".to_string();
    let new_line = format!("        |input| Box::new(day{day:02}::Day{day:02}::new(input)),");

    add_line(file, &regex, &new_line)
}

fn add_line(file: &str, regex: &str, new_line: &str) -> io::Result<()> {
    let content = fs::read_to_string(file)?;

    let re = Regex::new(regex).unwrap();

    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let mut insert_index = 0;
    for (i, line) in lines.iter().enumerate() {
        if re.is_match(line) {
            insert_index = i + 1;
        }
    }

    lines.insert(insert_index, new_line.into());

    fs::write(file, lines.join("\n"))?;
    Ok(())
}

fn load_dot_env() -> Result<(), Box<dyn Error>> {
    if !file_exists(".env") {
        fs::copy(".env.dist", ".env")?;
    }
    dotenv()?;

    Ok(())
}

fn download_puzzle(year: usize, day: usize, session_cookie: &str) -> Result<(), Box<dyn Error>> {
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let client = Client::new();
    let response = client
        .get(&url)
        .header(USER_AGENT, "https://github.com/Davvos11/aoc_2025/")
        .header(COOKIE, format!("session={}", session_cookie))
        .send()?;

    response.error_for_status_ref()?;

    let contents = response.text()?;
    let file = format!("puzzles/day{day:02}.txt");
    fs::write(file, contents)?;

    Ok(())
}
