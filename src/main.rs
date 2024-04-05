use std::io::prelude::Write;
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
                .arg(arg!(<TODO> "The todo to add"))
                .arg_required_else_help(true),
        )
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("list", _sub_matches)) => {
            println!("listing todos");
        }
        Some(("add", sub_matches)) => {
            let path = "/Users/faroukhamadi/.todos";
            if Path::new(&path).exists() {
                let mut file = OpenOptions::new()
                    .append(true)
                    .open(path)
                    .expect("should be able to open file");

                let todo = sub_matches.get_one::<String>("TODO").expect("required");

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
