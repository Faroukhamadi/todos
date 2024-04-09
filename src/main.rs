use std::fs;
use std::io::prelude::Write;
use std::io::BufReader;
use std::{fs::OpenOptions, path::Path};

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("tds")
        .about("A tool to manage your todos")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("list").about("lists todos"))
        .subcommand(
            Command::new("add")
                .about("adds new todo to todos")
                .arg(arg!(<TASK> "The todo to add"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("do")
                .about("marks a todo as done")
                .arg(arg!(<TASK> "The todo to mark as done"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();
    let path = "/Users/faroukhamadi/.todos";

    match matches.subcommand() {
        Some(("list", _sub_matches)) => {
            if Path::new(&path).exists() {
                println!("Todos:");

                let lines: Vec<String> = fs::read_to_string(&path)
                    .unwrap()
                    .lines()
                    .map(String::from)
                    .collect();

                for line in lines {
                    println!("{line}");
                }
            } else {
                println!("No todos");
            }
        }
        Some(("add", sub_matches)) => {
            if Path::new(&path).exists() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path)
                    .expect("should be able to open file");

                let todo = sub_matches.get_one::<String>("TASK").expect("required");

                if let Err(e) = writeln!(file, "{}", todo) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                println!("IT JUST DOESNT EXIST");
            }
        }
        Some(("do", sub_matches)) => {
            if Path::new(&path).exists() {
                let mut file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .open(path)
                    .expect("should be able to open file");

                let todo = sub_matches.get_one::<String>("TASK").expect("required");

                let lines: Vec<String> = fs::read_to_string(&path)
                    .unwrap()
                    .lines()
                    .map(String::from)
                    .filter(|line| !line.contains(todo))
                    .collect();

                println!("{:?}", lines);
                fs::write(&path, lines.join("\n")).expect("should be able to write to file");


                if let Err(e) = writeln!(file, "{}", todo) {
                    eprintln!("Couldn't write to file: {}", e);
                }
            } else {
                println!("IT JUST DOESNT EXIST");
            }
        }
        Some(_) => {
            println!("random")
        }
        None => {
            println!("we don't have a subcommand")
        }
    }
}
