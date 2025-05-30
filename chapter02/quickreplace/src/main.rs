use std::env;
use std::fs;
use text_colorizer::*;

/// 引数構造体
#[derive(Debug)]
struct Arguments {
    target: String,
    rfeplacement: String,
    filename: String,
    output: String,
}

/// Usageの関数
fn print_usage() {
    eprintln!(
        "{} change occurrences of one string into another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

fn parse_args() -> Arguments {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 4 {
        print_usage();
        eprintln!(
            "{}: wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }
    Arguments {
        target: args[0].clone(),
        rfeplacement: args[1].clone(),
        filename: args[2].clone(),
        output: args[3].clone(),
    }
}

use regex::Regex;
fn reapace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(text, replacement).to_string())
}

fn main() {
    let args = parse_args();

    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                args.filename,
                e
            );
            std::process::exit(1);
        },
    };

    let repalaced_data = match reapace(&args.target, &args.rfeplacement, &data) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{} failed to replace text {:?}", "Error:".red().bold(), e);
            std::process::exit(1);
        },
    };

    match fs::write(&args.output, &repalaced_data) {
        Ok(_) => {},
        Err(e) => {
            eprintln!(
                "{} failed to write to file '{}': {:?}",
                "Error:".red().bold(),
                args.output,
                e
            );
            std::process::exit(1);
        },
    };
}
